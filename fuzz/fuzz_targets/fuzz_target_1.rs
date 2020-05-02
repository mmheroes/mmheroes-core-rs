#![no_main]
use libfuzzer_sys::fuzz_target;

use mmheroes_core::logic::{Game, GameMode};
use mmheroes_core::ui::{recorded_input::RecordedInputRenderer, GameUI};

fuzz_target!(|input: &[u8]| {
    let structured_input = input.iter().map(|b| {
        use mmheroes_core::ui::Input::*;
        match b {
            0   => KeyUp,
            1   => KeyDown,
            2   => Enter,
            255 => EOF,
            _   => Other,
        }
    });
    let mut renderer = RecordedInputRenderer::new(structured_input);
    let game = Game::new(GameMode::God, 0 /* TODO: Randomize seed */);
    let mut runner = GameUI::new(&mut renderer, game);
    let _result = runner.run();
});
