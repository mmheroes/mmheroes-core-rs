use crate::logic::*;
use crate::ui::{renderer::Renderer, screens::scene_router, *};

pub(in crate::ui) fn display_rest_in_mausoleum(
    r: &mut Renderer,
    available_actions: &[Action],
    state: &GameState,
) -> WaitingState {
    scene_router::display_header_stats(r, state);
    r.move_cursor_to(7, 0);
    writeln_colored!(White, r, "Выбери себе способ \"культурного отдыха\".");
    scene_router::display_short_today_timetable(
        r,
        10,
        state.current_day(),
        state.player(),
    );
    r.move_cursor_to(10, 0);
    dialog(r, available_actions)
}
