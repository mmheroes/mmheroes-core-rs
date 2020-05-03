#![macro_use]

pub mod recorded_input;
pub use recorded_input::*;

use crate::ui::*;

pub trait Renderer {
    type Error;

    fn clear_screen(&mut self) -> Result<(), Self::Error>;

    fn flush(&mut self) -> Result<(), Self::Error>;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error>;

    fn move_cursor_to(&mut self, line: i32, column: i32) -> Result<(), Self::Error>;

    /// Return the cursor position. This method not necessarily returns the actual cursor
    /// position. For example, `RecordedInputRenderer` always returns `(0, 0)`.
    /// Because of that, callers should not make decisions based on what this method
    /// returns. It only should be used to save the position in order to move to it later.
    fn get_cursor_position(&mut self) -> Result<(i32, i32), Self::Error>;

    fn set_color(
        &mut self,
        foreground: Color,
        background: Color,
    ) -> Result<(), Self::Error>;

    fn getch(&mut self) -> Result<Input, Self::Error>;

    fn sleep_ms(&mut self, ms: Milliseconds) -> Result<(), Self::Error>;

    fn write_fmt(&mut self, fmt: core::fmt::Arguments) -> Result<(), Self::Error> {
        use core::fmt::{
            write as fmt_write, Error as FmtError, Result as FmtResult, Write,
        };

        // Create a shim which translates a Renderer to a core::fmt::Write and saves
        // off renderer errors. instead of discarding them
        struct Adaptor<'a, T: Renderer + ?Sized + 'a> {
            inner: &'a mut T,
            error: Result<(), T::Error>,
        }

        impl<T: Renderer + ?Sized> Write for Adaptor<'_, T> {
            fn write_str(&mut self, s: &str) -> FmtResult {
                match self.inner.write_str(s) {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        self.error = Err(e);
                        Err(FmtError)
                    }
                }
            }
        }

        let mut output = Adaptor {
            inner: self,
            error: Ok(()),
        };
        match fmt_write(&mut output, fmt) {
            Ok(()) => Ok(()),
            Err(..) => Err(output.error.expect_err("formatter error")),
        }
    }
}

// FIXME: Maybe this should save and restore graphic state instead of setting the color
// globally.
macro_rules! write_colored {
    ($color:ident, $renderer:expr, $($arg:tt)*) => {
        match $renderer.set_color(Color::$color, Color::Black) {
            Ok(()) => write!($renderer, $($arg)*),
            Err(err) => Err(err),
        }
    };
}

macro_rules! writeln_colored {
    ($color:ident, $renderer:expr, $($arg:tt)*) => {
        match $renderer.set_color(Color::$color, Color::Black) {
            Ok(()) => writeln!($renderer, $($arg)*),
            Err(err) => Err(err),
        }
    };
}
