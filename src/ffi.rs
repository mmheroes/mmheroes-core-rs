use crate::ui::*;
use crate::ui::Milliseconds;
use core::ffi::c_void;

pub type RendererContext = Option<*mut c_void>;

/// This type is declared here to make cbindgen happy:
/// https://github.com/eqrion/cbindgen/issues/399
#[allow(non_camel_case_types)]
type c_char = u8;

/// A renderer for use in non-Rust clients.
/// Set its fields to the necessary values yourself.
#[repr(C)]
pub struct PolymorphicRenderer {
    /// An opaque object that will be passed as a first argument of
    /// the renderer functions.
    /// For example, if you implement the renderer using curses, this will be
    /// the window object.
    pub renderer_ctx: RendererContext,
    pub clear_screen: Option<fn(RendererContext)>,
    pub flush: Option<fn(RendererContext)>,
    pub move_cursor_to: Option<fn(RendererContext, i32, i32)>,
    pub set_color: Option<fn(RendererContext, Color)>,
    pub write_str: Option<fn(RendererContext, *const c_char, usize)>,
    pub getch: Option<fn(RendererContext) -> Input>,
    pub sleep_ms: Option<fn(RendererContext, Milliseconds)>,
}

impl Renderer for PolymorphicRenderer {
    fn clear_screen(&mut self) {
        self.clear_screen.map(|f| f(self.renderer_ctx));
    }

    fn flush(&mut self) {
        self.flush.map(|f| f(self.renderer_ctx));
    }

    fn move_cursor_to(&mut self, line: i32, column: i32) {
        self.move_cursor_to
            .map(|f| f(self.renderer_ctx, line, column));
    }

    fn set_color(&mut self, color: Color) {
        self.set_color.map(|f| f(self.renderer_ctx, color));
    }

    fn write_str<S: AsRef<str>>(&mut self, string: S) {
        let s = string.as_ref();
        self.write_str
            .map(|f| f(self.renderer_ctx, s.as_ptr(), s.len()));
    }

    fn getch(&mut self) -> Input {
        self.getch.map_or(Input::EOF, |f| f(self.renderer_ctx))
    }

    fn sleep_ms(&mut self, ms: Milliseconds) {
        self.sleep_ms.map(|f| f(self.renderer_ctx, ms));
    }
}

#[no_mangle]
pub extern "C" fn mmheroes_run_game(
    renderer: &mut PolymorphicRenderer,
    mode: crate::logic::GameMode,
    seed: u64,
) {
    let run_game = move || {
        let game = crate::logic::Game::new(mode, seed);
        let mut game_ui = GameUI::new(renderer, game);
        game_ui.run()
    };

    // Unwinding through FFI boundaries is undefined behavior, so we stop any unwinding and abort.
    #[cfg(feature = "std")]
    let mut safely_run = || {
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

    safely_run();
}
