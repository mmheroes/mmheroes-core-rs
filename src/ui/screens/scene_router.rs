use crate::logic::*;
use crate::ui::renderer::{Line, Renderer};
use crate::ui::*;

pub(in crate::ui) fn display_scene_router(
    r: &mut Renderer,
    state: &GameState,
) -> WaitingState {
    display_header_stats(r, state);
    display_short_today_timetable(r, state.current_day(), state.player());
    r.set_color(Color::White, Color::Black);
    r.move_cursor_to(7, 0);
    let mut options = tiny_vec![capacity: 16];

    let i_am_done = ("С меня хватит!", Color::BlueBright, Action::IAmDone);
    match state.location() {
        Location::PUNK => todo!(),
        Location::PDMI => todo!(),
        Location::ComputerClass => {
            writeln!(r, "Ты в компьютерном классе. Что делать?");
            // TODO: Здесь может быть Климов
            options.push(("Пойти в общагу", Color::CyanBright, Action::GoToDorm));
            options.push(("Покинуть класс", Color::CyanBright, Action::GoToPUNK));
            options.push(("Поехать в ПОМИ", Color::CyanBright, Action::GoToPDMI));
            options.push(("Пойти в мавзолей", Color::CyanBright, Action::GoToMausoleum));
            if state.player().has_internet() {
                options.push((
                    "Провести 1 час в Inet'е",
                    Color::CyanBright,
                    Action::SurfInternet,
                ));
            }
            // TODO: Здесь могут быть однокурсники и Кузьменко
            if state.player().has_mmheroes_floppy() {
                options.push((
                    "Поиграть в MMHEROES",
                    Color::CyanBright,
                    Action::PlayMMHEROES,
                ));
            }
            options.push(i_am_done);
        }
        Location::Dorm => {
            writeln!(r, "Ты в общаге. Что делать?");
            options.push(("Готовиться", Color::CyanBright, Action::Study));
            options.push((
                "Посмотреть расписание",
                Color::CyanBright,
                Action::ViewTimetable,
            ));
            options.push(("Отдыхать", Color::CyanBright, Action::Rest));
            options.push(("Лечь спать", Color::CyanBright, Action::GoToBed));
            options.push(("Пойти на факультет", Color::CyanBright, Action::GoToPUNK));
            options.push(("Поехать в ПОМИ", Color::CyanBright, Action::GoToPDMI));
            options.push(("Пойти в мавзолей", Color::CyanBright, Action::GoToMausoleum));
            options.push(i_am_done);
            options.push(("ЧТО ДЕЛАТЬ ???", Color::BlueBright, Action::WhatToDo));
        }
        Location::Mausoleum => {
            writeln!(r, "Ты в мавзолее. Что делать?");
            options.push(("Идти в ПУНК", Color::CyanBright, Action::GoToPUNK));
            options.push(("Поехать в ПОМИ", Color::CyanBright, Action::GoToPDMI));
            options.push(("Идти в общагу", Color::CyanBright, Action::GoToDorm));
            options.push(("Отдыхать", Color::CyanBright, Action::Rest));
            for classmate_info in
                state.classmates().filter_by_location(Location::Mausoleum)
            {
                options.push((
                    classmate_name(classmate_info.classmate()),
                    Color::YellowBright,
                    Action::InteractWithClassmate(classmate_info.classmate()),
                ));
            }
            options.push(i_am_done);
        }
    }
    r.move_cursor_to(9, 0);
    if state.failed_attempt_to_sleep() {
        assert!(state.location() == Location::Dorm);
        inactive_dialog(r, &options);
        r.move_cursor_to(21, 0);
        write_colored!(White, r, "Тебя чего-то не тянет по-спать...");
        wait_for_any_key(r)
    } else {
        dialog(r, options)
    }
}

pub(in crate::ui::screens) fn display_header_stats(r: &mut Renderer, state: &GameState) {
    display_character_stats(r, state.current_day(), state.current_time(), state.player());
    display_knowledge(r, state.player());
}

fn display_character_stats(r: &mut Renderer, today: &Day, now: Time, player: &Player) {
    write_colored!(White, r, "Сегодня ");
    // Первый день недели — 22-е мая.
    write_colored!(WhiteBright, r, "{}", today.index() + 22);
    write_colored!(White, r, "е мая; ");
    write_colored!(WhiteBright, r, "{}:00", now);
    r.move_cursor_to(0, 25);
    writeln_colored!(MagentaBright, r, "Версия gamma3.14");

    write_colored!(White, r, "Самочувствие: ");
    match player.health().assessment() {
        HealthAssessment::LivingDead => writeln_colored!(Magenta, r, "живой труп"),
        HealthAssessment::TimeToDie => writeln_colored!(Red, r, "пора помирать ..."),
        HealthAssessment::Bad => writeln_colored!(Red, r, "плохое"),
        HealthAssessment::SoSo => writeln_colored!(YellowBright, r, "так себе"),
        HealthAssessment::Average => writeln_colored!(YellowBright, r, "среднее"),
        HealthAssessment::Good => writeln_colored!(Green, r, "хорошее"),
        HealthAssessment::Great => writeln_colored!(Green, r, "отличное"),
    }

    write_colored!(White, r, "Финансы: ");
    if player.money() > Money::zero() {
        write_colored!(WhiteBright, r, "{}", player.money());
        writeln_colored!(White, r, " руб.");
    } else if !player.got_stipend() {
        writeln_colored!(RedBright, r, "Надо получить деньги за май...");
    } else {
        writeln_colored!(White, r, "Ты успел потратить все деньги.");
    }

    match player.brain().assessment() {
        BrainAssessment::ClinicalBrainDeath => {
            writeln_colored!(Magenta, r, "Клиническая смерть мозга")
        }
        BrainAssessment::BrainIsAlmostNonFunctioning => {
            writeln_colored!(Magenta, r, "Голова просто никакая")
        }
        BrainAssessment::ThinkingIsAlmostImpossible => {
            writeln_colored!(RedBright, r, "Думать практически невозможно")
        }
        BrainAssessment::ThinkingIsDifficult => {
            writeln_colored!(RedBright, r, "Думать трудно")
        }
        BrainAssessment::BrainIsAlmostOK => {
            writeln_colored!(YellowBright, r, "Голова почти в норме")
        }
        BrainAssessment::BrainIsOK => writeln_colored!(YellowBright, r, "Голова в норме"),
        BrainAssessment::BrainIsFresh => writeln_colored!(Green, r, "Голова свежая"),
        BrainAssessment::ExtraordinaryEaseOfThought => {
            writeln_colored!(Green, r, "Легкость в мыслях необыкновенная")
        }
        BrainAssessment::ContactTheDeveloper => {
            writeln_colored!(CyanBright, r, "Обратитесь к разработчику ;)")
        }
    }

    match player.stamina().assessment() {
        StaminaAssessment::MamaTakeMeBack => {
            writeln_colored!(Magenta, r, "Мама, роди меня обратно!")
        }
        StaminaAssessment::CompletelyOverstudied => {
            writeln_colored!(Magenta, r, "Окончательно заучился")
        }
        StaminaAssessment::ICantTakeIt => {
            writeln_colored!(RedBright, r, "Я так больше немогууу!")
        }
        StaminaAssessment::IWishItAllEndedSoon => {
            writeln_colored!(RedBright, r, "Скорее бы все это кончилось...")
        }
        StaminaAssessment::ALittleMoreAndThenRest => {
            writeln_colored!(YellowBright, r, "Еще немного и пора отдыхать")
        }
        StaminaAssessment::ABitTired => {
            writeln_colored!(YellowBright, r, "Немного устал")
        }
        StaminaAssessment::ReadyForEverything => {
            writeln_colored!(Green, r, "Готов к труду и обороне")
        }
        StaminaAssessment::GreatThingsAwaitUs => {
            writeln_colored!(Green, r, "Нас ждут великие дела")
        }
    }

    match player.charisma().assessment() {
        CharismaAssessment::VeryIntroverted => {
            writeln_colored!(Magenta, r, "Очень замкнутый товарищ")
        }
        CharismaAssessment::PreferSolitariness => {
            writeln_colored!(Magenta, r, "Предпочитаешь одиночество")
        }
        CharismaAssessment::VeryHardToTalkToPeople => {
            writeln_colored!(RedBright, r, "Тебе трудно общаться с людьми")
        }
        CharismaAssessment::NotEasyToTalkToPeople => {
            writeln_colored!(RedBright, r, "Тебе непросто общаться с людьми")
        }
        CharismaAssessment::Normal => {
            writeln_colored!(YellowBright, r, "Ты нормально относишься к окружающим")
        }
        CharismaAssessment::ManyFriends => {
            writeln_colored!(Green, r, "У тебя много друзей")
        }
        CharismaAssessment::TonsOfFriends => {
            writeln_colored!(Green, r, "У тебя очень много друзей")
        }
    }
}

fn color_for_assessment(assessment: KnowledgeAssessment) -> Color {
    match assessment {
        KnowledgeAssessment::Bad => Color::Cyan,
        KnowledgeAssessment::Satisfactory => Color::White,
        KnowledgeAssessment::Good => Color::WhiteBright,
        KnowledgeAssessment::VeryGood => Color::Green,
        KnowledgeAssessment::Excellent => Color::YellowBright,
    }
}

fn display_knowledge(r: &mut Renderer, player: &Player) {
    for (i, (subject, _)) in SUBJECTS.iter().enumerate() {
        let line = i as Line;
        r.move_cursor_to(line, 45);
        write_colored!(CyanBright, r, "{}", subject_name(*subject));

        let knowledge = player.status_for_subject(*subject).knowledge();
        r.move_cursor_to(line, 67);
        r.set_color(
            color_for_assessment(knowledge.absolute_knowledge_assessment()),
            Color::Black,
        );
        write!(r, "{}", knowledge);

        let relative_assessment = knowledge.relative_knowledge_assessment(*subject);
        r.move_cursor_to(line, 71);
        r.set_color(color_for_assessment(relative_assessment), Color::Black);
        let assessment_description = match relative_assessment {
            KnowledgeAssessment::Bad => "Плохо",
            KnowledgeAssessment::Satisfactory => "Удовл.",
            KnowledgeAssessment::Good => "Хорошо",
            KnowledgeAssessment::VeryGood => unreachable!(),
            KnowledgeAssessment::Excellent => "Отлично",
        };
        write!(r, "{}", assessment_description);
    }
}

pub(in crate::ui::screens) fn display_short_today_timetable(
    r: &mut Renderer,
    today: &Day,
    player: &Player,
) {
    for (i, (subject, subject_info)) in SUBJECTS.iter().enumerate() {
        let line = (i as Line) + 9;
        r.move_cursor_to(line, 50);
        let passed = player.status_for_subject(*subject).passed();
        let set_color_if_passed = |r: &mut Renderer, if_passed, if_not_passed| {
            r.set_color(if passed { if_passed } else { if_not_passed }, Color::Black)
        };
        set_color_if_passed(r, Color::Blue, Color::CyanBright);
        write!(r, "{}", subject_short_name(*subject));
        r.move_cursor_to(line, 58);
        set_color_if_passed(r, Color::Magenta, Color::RedBright);
        if let Some(exam) = today.exam(*subject) {
            write!(r, "{}", exam.location());
            set_color_if_passed(r, Color::Gray, Color::WhiteBright);
            r.move_cursor_to(line, 64);
            write!(r, "{}-{}", exam.from(), exam.to());
        } else {
            write!(r, "----");
        }

        r.move_cursor_to(line, 72);
        let problems_done = player.status_for_subject(*subject).problems_done();
        let problems_required = subject_info.required_problems();
        let problems_color = if problems_done == 0 {
            Color::White
        } else if problems_done == problems_required {
            Color::YellowBright
        } else {
            Color::Green
        };
        r.set_color(problems_color, Color::Black);

        write!(r, "{}/{}", problems_done, problems_required);
    }
}
