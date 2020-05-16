use crate::logic::npc::{GrishaInteraction, KolyaInteraction, PashaInteraction};
use crate::logic::*;
use crate::ui::{renderer::Renderer, screens::scene_router, *};

pub(in crate::ui) fn display_kolya_interaction(
    r: &mut Renderer,
    state: &GameState,
    available_actions: &[Action],
    interaction: npc::KolyaInteraction,
) -> WaitingState {
    use KolyaInteraction::*;

    match interaction {
        SolvedAlgebraProblemsForFree | PromptOatTincture | BrakeFluidNoMoney => {
            r.clear_screen();
            scene_router::display_header_stats(r, state);
            r.move_cursor_to(7, 0);
            writeln_colored!(
                White,
                r,
                "Коля смотрит на тебя немного окосевшими глазами."
            );
        }
        _ => (),
    }

    let solved_algebra_problems = |r: &mut Renderer| {
        writeln_colored!(
            WhiteBright,
            r,
            "\"У тебя остались нерешенные задачи по Всемирнову? Давай сюда!\""
        );
        write_colored!(White, r, "Коля решил тебе еще ");
        write_colored!(WhiteBright, r, "{}", 2);
        writeln_colored!(White, r, " задачи по алгебре!");
    };

    match interaction {
        SolvedAlgebraProblemsForFree => {
            solved_algebra_problems(r);
            return wait_for_any_key(r);
        }
        PromptOatTincture | BrakeFluidNoMoney => {
            writeln_colored!(
                WhiteBright,
                r,
                "\"Знаешь, пиво, конечно, хорошо, но настойка овса - лучше!\""
            );
        }
        _ => {}
    }

    let brake_fluid = |r: &mut Renderer| {
        writeln_colored!(
            MagentaBright,
            r,
            "Коля достает тормозную жидкость, и вы распиваете еще по стакану."
        );
    };

    match interaction {
        SolvedAlgebraProblemsForFree => unreachable!(),
        PromptOatTincture => {
            writeln_colored!(White, r, "Заказать Коле настойку овса?");
            scene_router::display_short_today_timetable(
                r,
                11,
                state.current_day(),
                state.player(),
            );
            r.move_cursor_to(14, 0);
            return dialog(r, available_actions);
        }
        SolvedAlgebraProblemsForOatTincture => {
            r.move_cursor_to(18, 0);
            solved_algebra_problems(r);
        }
        BrakeFluidNoMoney => {
            r.move_cursor_to(14, 0);
            brake_fluid(r);
            return wait_for_any_key(r);
        }
        BrakeFluidBecauseRefused => {
            r.move_cursor_to(18, 0);
            writeln_colored!(WhiteBright, r, "\"Зря, ой, зря ...\"");
            brake_fluid(r);
        }
        Altruism => {
            r.move_cursor_to(18, 0);
            writeln_colored!(
                White,
                r,
                "Твой альтруизм навсегда останется в памяти потомков."
            );
        }
    }

    wait_for_any_key(r)
}

pub(in crate::ui) fn display_pasha_interaction(
    r: &mut Renderer,
    state: &GameState,
    interaction: npc::PashaInteraction,
) -> WaitingState {
    scene_router::display_header_stats(r, state);
    r.move_cursor_to(7, 0);
    match interaction {
        PashaInteraction::Stipend => {
            write_colored!(White, r, "Паша вручает тебе твою стипуху за май: ");
            write_colored!(WhiteBright, r, "{}", Money::stipend());
            write_colored!(White, r, " руб.");
        }
        PashaInteraction::Inspiration => {
            writeln_colored!(Green, r, "Паша воодушевляет тебя на великие дела.");
            writeln_colored!(RedBright, r, "Вместе с этим он немного достает тебя.");
        }
    }
    wait_for_any_key(r)
}

pub(in crate::ui) fn display_grisha_interaction(
    r: &mut Renderer,
    state: &GameState,
    available_actions: &[Action],
    interaction: npc::GrishaInteraction,
) -> WaitingState {
    use GrishaInteraction::*;

    match interaction {
        PromptEmploymentAtTerkom
        | ProxyAddress
        | WantFreebie { .. }
        | FreebieComeToMe { .. }
        | FreebieExists { .. }
        | LetsOrganizeFreebieLoversClub { .. }
        | NoNeedToStudyToGetDiploma { .. }
        | YouStudiedDidItHelp { .. }
        | ThirdYearStudentsDontAttendLectures { .. }
        | TakeExampleFromKolya { .. }
        | HateLevTolstoy { .. }
        | DontGoToPDMI { .. }
        | NamesOfFreebieLovers { .. }
        | LetsHaveABreakHere { .. }
        | NoNeedToTakeLectureNotes { .. }
        | CantBeExpelledInFourthYear { .. }
        | MechanicsHaveFreebie { .. } => {
            r.clear_screen();
            scene_router::display_header_stats(r, state);
            r.move_cursor_to(7, 0);
            r.set_color(Color::White, Color::Black);
        }
        CongratulationsYouAreNowEmployed | AsYouWantButDontOverstudy => (),
    }

    let (reply, drink_beer, hour_pass) = match interaction {
        PromptEmploymentAtTerkom => {
            writeln_colored!(
                YellowBright,
                r,
                "\"А ты не хочешь устроиться в ТЕРКОМ? Может, кое-чего подзаработаешь...\""
            );
            writeln!(r);
            return dialog(r, available_actions);
        }
        CongratulationsYouAreNowEmployed => {
            r.move_cursor_to(13, 0);
            writeln_colored!(
                White,
                r,
                "\"Поздравляю, теперь ты можешь идти в \"контору\"!\""
            );
            return wait_for_any_key(r);
        }
        AsYouWantButDontOverstudy => {
            r.move_cursor_to(13, 0);
            writeln_colored!(
                White,
                r,
                "\"Как хочешь. Только смотри, не заучись там ...\""
            );
            return wait_for_any_key(r);
        }
        ProxyAddress => {
            writeln_colored!(
                White,
                r,
                "\"Кстати, я тут знаю один качественно работающий прокси-сервер...\""
            );
            writeln!(r);
            writeln_colored!(White, r, "Ты записываешь адрес. Вдруг пригодится?");
            return wait_for_any_key(r);
        }
        WantFreebie {
            drink_beer,
            hour_pass,
        } => ("Хочу халявы!", drink_beer, hour_pass),
        FreebieComeToMe {
            drink_beer,
            hour_pass,
        } => ("Прийди же, о халява!", drink_beer, hour_pass),
        FreebieExists {
            drink_beer,
            hour_pass,
        } => ("Халява есть - ее не может не быть.", drink_beer, hour_pass),
        LetsOrganizeFreebieLoversClub {
            drink_beer,
            hour_pass,
        } => (
            "Давай организуем клуб любетелей халявы!",
            drink_beer,
            hour_pass,
        ),
        NoNeedToStudyToGetDiploma {
            drink_beer,
            hour_pass,
        } => (
            "Чтобы получить диплом, учиться совершенно необязательно!",
            drink_beer,
            hour_pass,
        ),
        YouStudiedDidItHelp {
            drink_beer,
            hour_pass,
        } => (
            "Ну вот, ты готовился... Помогло это тебе?",
            drink_beer,
            hour_pass,
        ),
        ThirdYearStudentsDontAttendLectures {
            drink_beer,
            hour_pass,
        } => (
            "На третьем курсе на лекции уже никто не ходит. Почти никто.",
            drink_beer,
            hour_pass,
        ),
        TakeExampleFromKolya {
            drink_beer,
            hour_pass,
        } => ("Вот, бери пример с Коли.", drink_beer, hour_pass),
        HateLevTolstoy {
            drink_beer,
            hour_pass,
        } => (
            "Ненавижу Льва Толстого! Вчера \"Войну и мир\" <йк> ксерил...",
            drink_beer,
            hour_pass,
        ),
        DontGoToPDMI {
            drink_beer,
            hour_pass,
        } => ("А в ПОМИ лучше вообще не ездить!", drink_beer, hour_pass),
        NamesOfFreebieLovers {
            drink_beer,
            hour_pass,
        } => (
            "Имена главных халявчиков и алкоголиков висят на баобабе.",
            drink_beer,
            hour_pass,
        ),
        LetsHaveABreakHere {
            drink_beer,
            hour_pass,
        } => (
            "Правильно, лучше посидим здесь и оттянемся!",
            drink_beer,
            hour_pass,
        ),
        NoNeedToTakeLectureNotes {
            drink_beer,
            hour_pass,
        } => (
            "Конспектировать ничего не надо. В мире есть ксероксы!",
            drink_beer,
            hour_pass,
        ),
        CantBeExpelledInFourthYear {
            drink_beer,
            hour_pass,
        } => (
            "А с четвертого курса вылететь уже почти невозможно.",
            drink_beer,
            hour_pass,
        ),
        MechanicsHaveFreebie {
            drink_beer,
            hour_pass,
        } => ("Вот у механиков - у них халява!", drink_beer, hour_pass),
    };

    write_colored!(White, r, "Гриша:");
    writeln_colored!(YellowBright, r, "\"{}\"", reply);
    if drink_beer {
        writeln_colored!(White, r, "И еще по пиву...");
    }
    if hour_pass {
        writeln_colored!(White, r, "И еще один час прошел в бесплодных разговорах...");
    }

    wait_for_any_key(r)
}
