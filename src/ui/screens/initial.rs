use crate::logic::*;
use crate::ui::{renderer::Renderer, *};

pub(in crate::ui) fn display_intro(r: &mut Renderer) -> WaitingState {
    r.set_color(Color::Gray, Color::Black);
    writeln!(
        r,
        "                                                Нам понятен этот смех"
    );
    writeln!(
        r,
        "                                                Не попавших на Мат-Мех"
    );
    writeln!(
        r,
        "                                                  (надпись на парте)"
    );
    writeln!(r);
    writeln!(r);
    writeln!(r);
    r.set_color(Color::WhiteBright, Color::Black);
    writeln!(
        r,
        " H H  EEE  RR    O   EEE  SS       M   M  A   A TTTTT       M   M  EEE  X   X"
    );
    writeln!(
        r,
        " H H  E    R R  O O  E   S         MM MM  AAAAA   T         MM MM    E   X X"
    );
    writeln!(
        r,
        " HHH  EE   RR   O O  EE   S    OF  M M M  A   A   T    &&&  M M M   EE    X"
    );
    writeln!(
        r,
        " H H  E    R R  O O  E     S       M   M   A A    T         M   M    E   X X"
    );
    writeln!(
        r,
        " H H  EEE  R R   O   EEE SS        M   M    A     T         M   E  EEE  X   X"
    );
    writeln!(r);
    writeln!(r);
    writeln!(r);
    r.set_color(Color::RedBright, Color::Black);
    writeln!(r, "                             ГЕРОИ МАТА И МЕХА ;)");
    writeln!(r);
    writeln!(r);
    r.set_color(Color::CyanBright, Color::Black);
    writeln!(r, "(P) CrWMM Development Team, 2001.");
    writeln!(r, "Версия gamma3.14.");
    writeln!(r, "Загляните на нашу страничку: mmheroes.chat.ru !");
    wait_for_any_key(r)
}

pub(in crate::ui) fn display_initial_parameters(
    r: &mut Renderer,
    available_actions: &[Action],
    mode: GameMode,
) -> WaitingState {
    debug_assert!(mode == GameMode::God || mode == GameMode::SelectInitialParameters);
    r.set_color(Color::White, Color::Black);
    writeln!(r, "Выбери начальные параметры своего \"героя\":");
    writeln!(r);

    dialog(r, dialog_options_for_actions(available_actions))
}

pub(in crate::ui) fn display_ding(r: &mut Renderer) -> WaitingState {
    r.set_color(Color::Green, Color::Black);
    writeln!(r, "ДЗИНЬ!");
    sleep(r, Milliseconds(500));
    r.set_color(Color::YellowBright, Color::Black);
    writeln!(r, "ДДДЗЗЗЗЗИИИИИИННННННЬ !!!!");
    sleep(r, Milliseconds(700));
    r.set_color(Color::RedBright, Color::Black);
    writeln!(r, "ДДДДДДЗЗЗЗЗЗЗЗЗЗЗЗЗИИИИИИИИИИННННННННННННЬ !!!!!!!!!!");
    sleep(r, Milliseconds(1000));
    r.set_color(Color::White, Color::Black);
    writeln!(r, "Ты просыпаешься от звонка будильника 22-го мая в 8:00.");
    writeln!(r, "Неожиданно ты осознаешь, что началась зачетная неделя,");
    writeln!(
        r,
        "а твоя готовность к этому моменту практически равна нулю."
    );
    writeln!(r, "Натягивая на себя скромное одеяние студента,");
    writeln!(
        r,
        "ты всматриваешься в заботливо оставленное соседом на стене"
    );
    writeln!(r, "расписание: когда и где можно найти искомого препода ?");
    wait_for_any_key(r)
}
