#![no_main]
use libfuzzer_sys::fuzz_target;

use mmheroes_core::logic::{Game, GameMode};
use mmheroes_core::ui::GameUI;

fuzz_target!(|input: &[u8]| {
    let structured_input = input.iter().map(|b| {
        use mmheroes_core::ui::Input::*;
        match b {
            0   => KeyUp,
            1   => KeyDown,
            2   => Enter,
            _   => Other,
        }
    });
    let mut game = Game::new(GameMode::God, 0);
    let mut game_ui = GameUI::new(&mut game);

    for input_value in structured_input {
        if !game_ui.continue_game(input_value) {
            break;
        }
    }
});
