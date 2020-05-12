use crate::logic::*;
use crate::ui::{renderer::Renderer, screens::scene_router, *};

pub(in crate::ui) fn display_rest_in_mausoleum(
    r: &mut Renderer,
    available_actions: usize,
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
    let mut options = tiny_vec![capacity: 16];
    let money = state.player().money();
    if money >= Money::cola_cost() {
        options.push(("Стакан колы за 4 р.", Color::CyanBright, Action::OrderCola));
    }
    if money >= Money::soup_cost() {
        options.push((
            "Суп, 6 р. все удовольствие",
            Color::CyanBright,
            Action::OrderSoup,
        ));
    }
    if money >= Money::soup_cost() {
        options.push(("0,5 пива за 8 р.", Color::CyanBright, Action::OrderBeer));
    }
    options.push((
        "Расслабляться будем своими силами.",
        Color::CyanBright,
        Action::Rest,
    ));
    options.push((
        "Нет, отдыхать - это я зря сказал.",
        Color::CyanBright,
        Action::GoBack,
    ));
    assert_eq!(options.len(), available_actions);
    dialog(r, options)
}
