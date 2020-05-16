use crate::logic::{Action, CauseOfDeath, GameState};
use crate::ui::{renderer::Renderer, *};

pub(in crate::ui) fn display_i_am_done(
    r: &mut Renderer,
    available_actions: &[Action],
) -> WaitingState {
    writeln_colored!(White, r, "Ну, может не надо так резко...");
    writeln_colored!(White, r, "Ты что, серьезно хочешь закончить игру?");
    writeln!(r);
    dialog(r, available_actions)
}

fn display_game_end_dead(r: &mut Renderer, cause: CauseOfDeath) -> WaitingState {
    use CauseOfDeath::*;
    r.set_color(Color::RedBright, Color::Black);
    writeln!(r, "Легче лбом колоть орехи,");
    writeln!(r, "чем учиться на МАТ-МЕХе.");
    r.set_color(Color::MagentaBright, Color::Black);
    match cause {
        OnTheWayToPUNK => writeln!(r, "Умер по пути на факультет."),
        OnTheWayToMausoleum => writeln!(r, "Умер по пути в мавзолей."),
        OnTheWayToDorm => writeln!(r, "Умер по пути домой. Бывает."),
        FellFromStairs => writeln!(r, "Упал с лестницы у главного входа."),
        Burnout => writeln!(r, "Сгорел на работе."),
        Overstudied => writeln!(r, "Заучился."),
        StudiedTooWell => writeln!(r, "Зубрежка до добра не доводит!"),
        CouldntLeaveTheComputer => writeln!(r, "Не смог расстаться с компьютером."),
        CorpseFoundInTheTrain => writeln!(r, "В электричке нашли бездыханное тело."),
        KilledByInspectors => writeln!(r, "Контролеры жизни лишили."),
        FellAsleepInTheTrain => writeln!(r, "Заснул в электричке и не проснулся."),
        SplitPersonality => writeln!(r, "Раздвоение ложной личности."),
        BeerAlcoholism => writeln!(r, "Пивной алкоголизм, батенька..."),
        DrankTooMuch => writeln!(r, "Спился."),
        DrankTooMuchBeer => writeln!(r, "Губит людей не пиво, а избыток пива."),
        Altruism => writeln!(r, "Альтруизм не довел до добра."),
        TurnedToVegetable => writeln!(r, "Превратился в овощ."),
        TorturedByProfessor(subject) => {
            let verb_ending = match professor_gender(subject) {
                Gender::Male => "",
                Gender::Female => "а",
            };
            writeln!(r, "{} замучил{}.", professor_name(subject), verb_ending)
        }
        Paranoia => writeln!(r, "Бурно прогрессирующая паранойя."),
        TimeOut => writeln!(r, "Время вышло."),
        Suicide => writeln!(r, "Вышел сам."),
        SoftwareBug => {
            debug_assert!(false);
            writeln!(r, "Раздавлен безжалостной ошибкой в программе.")
        }
    }
    wait_for_any_key(r)
}

fn display_game_end_alive(r: &mut Renderer) -> WaitingState {
    // TODO: Display proper text based on the final state
    // (cause of expelling, or congratulation)
    writeln_colored!(MagentaBright, r, "Уффффф! Во всяком случае, ты еще живой.");
    writeln!(r);
    write_colored!(RedBright, r, "У тебя нет целых ");
    write_colored!(
        WhiteBright,
        r,
        "{}",
        6 /* TODO: actual number of exams remaining */
    );
    writeln_colored!(RedBright, r, " зачетов!");
    writeln_colored!(MagentaBright, r, "ТЫ ОТЧИСЛЕН!");

    wait_for_any_key(r)
}

pub(in crate::ui) fn display_game_end(
    r: &mut Renderer,
    state: &GameState,
) -> WaitingState {
    if let Some(cause_of_death) = state.player().cause_of_death() {
        display_game_end_dead(r, cause_of_death)
    } else {
        display_game_end_alive(r)
    }
}

pub(in crate::ui) fn display_wanna_try_again(
    r: &mut Renderer,
    available_actions: &[Action],
) -> WaitingState {
    writeln_colored!(White, r, "Хочешь попробовать еще?");
    writeln!(r);
    writeln!(r);

    dialog(r, available_actions)
}

pub(in crate::ui) fn display_disclaimer(r: &mut Renderer) -> WaitingState {
    writeln_colored!(Green, r, "DISCLAIMER");
    writeln!(r);
    r.set_color(Color::BlueBright, Color::Black);
    writeln!(
        r,
        "1.) Все персонажи реальны. Эта программа является лишь неким отражением"
    );
    writeln!(r, "    мнения ее автора об окружающей действительности.");
    writeln!(
        r,
        "    Автор не ставил цели оценить чью-либо линию поведения."
    );
    writeln!(r);
    writeln!(
        r,
        "2.) Почти все события реальны. Естественно, многие из них"
    );
    writeln!(r, "    представлены в несколько аллегорическом виде.");
    writeln!(r);
    writeln!(
        r,
        "3.) Все совпадения с другими реальными зачетными неделями,"
    );
    writeln!(
        r,
        "    проведенными кем-либо в каком-либо ВУЗе, лишь подчеркивают"
    );
    writeln!(r, "    реалистичность взглядов автора на реальность.");
    writeln!(r);
    writeln!(r);
    r.set_color(Color::RedBright, Color::Black);
    writeln!(
        r,
        "*.) Если вы нашли в данной программе ошибку (любую, включая опечатки),"
    );
    writeln!(r, "    Ваши комментарии будут очень полезны.");
    writeln!(r);
    r.set_color(Color::Gray, Color::Black);
    writeln!(
        r,
        "Автор не несет ответственность за психическое состояние игрока."
    );

    wait_for_any_key(r)
}
