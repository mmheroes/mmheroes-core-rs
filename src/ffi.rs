use crate::ui::Milliseconds;
use crate::ui::*;
use core::ffi::c_void;

pub type RendererContext = *mut c_void;

pub type OpaqueError = *const c_void;

/// This type is declared here to make cbindgen happy:
/// https://github.com/eqrion/cbindgen/issues/399
#[allow(non_camel_case_types)]
type c_char = u8;

macro_rules! declare_renderer_routine {
    (($($args:ty),*) -> ($($ret_tys:ty),*)) => {
        Option<fn(RendererContext, $($args,)* $(&mut $ret_tys,)* &mut OpaqueError) -> bool>
    }
}

/// A renderer for use in non-Rust clients.
/// Set its fields to the necessary values yourself.
#[repr(C)]
pub struct PolymorphicRenderer {
    /// An opaque object that will be passed as a first argument of
    /// the renderer functions.
    /// For example, if you implement the renderer using curses, this will be
    /// the window object.
    pub renderer_ctx: RendererContext,

    /// If an error occurred, these functions should store the error in the last
    /// parameter and return `false`.
    /// Otherwise, the last parameter is not touched, and `true` is returned.
    pub clear_screen: declare_renderer_routine!(() -> ()),
    pub flush: declare_renderer_routine!(() -> ()),
    pub move_cursor_to: declare_renderer_routine!((i32, i32) -> ()),
    pub get_cursor_position: declare_renderer_routine!(() -> (i32, i32)),
    pub set_color: declare_renderer_routine!((Color, Color) -> ()),
    pub write_str: declare_renderer_routine!((*const c_char, usize) -> ()),
    pub getch: declare_renderer_routine!(() -> (Input)),
    pub sleep_ms: declare_renderer_routine!((Milliseconds) -> ()),
}

macro_rules! call_renderer_routine {
    ($renderer:expr, $func:ident, ($($args:expr),*) -> ($($ret_id:ident),*)) => {
        {
            let mut err: OpaqueError = core::ptr::null();
            $(let mut $ret_id = Default::default();)*
            if !$renderer.$func.map_or(true, |f| f($renderer.renderer_ctx,
                                                   $($args,)* $(&mut $ret_id,)* &mut err)) {
                Err(err)
            } else {
                Ok(($($ret_id),*))
            }
        }
    };
}

impl Renderer for PolymorphicRenderer {
    type Error = OpaqueError;

    fn clear_screen(&mut self) -> Result<(), Self::Error> {
        call_renderer_routine!(self, clear_screen, () -> ())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        call_renderer_routine!(self, flush, () -> ())
    }

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        call_renderer_routine!(self, write_str, (s.as_ptr(), s.len()) -> ())
    }

    fn move_cursor_to(&mut self, line: i32, column: i32) -> Result<(), Self::Error> {
        call_renderer_routine!(self, move_cursor_to, (line, column) -> ())
    }

    fn get_cursor_position(&mut self) -> Result<(i32, i32), Self::Error> {
        call_renderer_routine!(self, get_cursor_position, () -> (line, column))
    }

    fn set_color(&mut self, foreground: Color, background: Color) -> Result<(), Self::Error> {
        call_renderer_routine!(self, set_color, (foreground, background) -> ())
    }

    fn getch(&mut self) -> Result<Input, Self::Error> {
        call_renderer_routine!(self, getch, () -> (input))
    }

    fn sleep_ms(&mut self, ms: Milliseconds) -> Result<(), Self::Error> {
        call_renderer_routine!(self, sleep_ms, (ms) -> ())
    }
}

/// Run the game. If any of the functions in the renderer return an error,
/// the game stops and the error is returned via the last argument. Also,
/// `false` is returned. If the game has successfully completed, `true`
/// is returned.
#[no_mangle]
pub extern "C" fn mmheroes_run_game(
    renderer: &mut PolymorphicRenderer,
    mode: crate::logic::GameMode,
    seed: u64,
    error: &mut OpaqueError,
) -> bool {
    let run_game = move || {
        let game = crate::logic::Game::new(mode, seed);
        let mut game_ui = GameUI::new(renderer, game);
        game_ui.run()
    };

    // Unwinding through FFI boundaries is undefined behavior, so we stop any unwinding and abort.
    #[cfg(feature = "std")]
    let safely_run = || {
        use std::panic::*;
        // AssertUnwindSafe is okay here, since we'll abort anyway.
        match catch_unwind(AssertUnwindSafe(run_game)) {
            Ok(r) => r,
            Err(_) => std::process::abort(),
        }
    };

    // However, if this crate is compiled as no_std, there is no unwinding.
    // The client will define the panic behavior themselves using #[panic_handler]
    #[cfg(not(feature = "std"))]
    let mut safely_run = run_game;

    match safely_run() {
        Ok(()) => true,
        Err(err) => {
            *error = err;
            false
        }
    }
}
