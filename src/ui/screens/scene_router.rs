use crate::logic::*;
use crate::ui::*;

pub(in crate::ui) fn display_scene_router<R: Renderer>(
    r: &mut R,
    state: &GameState,
) -> Result<Action, R::Error> {
    display_character_stats(r)?;
    display_knowledge(r)?;
    display_short_today_timetable(r, state.timetable())?;
    r.set_color(Color::White, Color::Black)?;
    r.move_cursor_to(7, 0)?;
    let mut options = stack_allocated_vec![(&str, Color); 12];
    match state.location() {
        Location::PUNK => todo!(),
        Location::PDMI => todo!(),
        Location::ComputerClass => todo!(),
        Location::Dorm => {
            writeln!(r, "Ты в общаге. Что делать?")?;
            options.push(("Готовиться", Color::CyanBright));
            options.push(("Посмотреть расписание", Color::CyanBright));
            options.push(("Отдыхать", Color::CyanBright));
            options.push(("Лечь спать", Color::CyanBright));
            options.push(("Пойти на факультет", Color::CyanBright));
            options.push(("Поехать в ПОМИ", Color::CyanBright));
            options.push(("Пойти в мавзолей", Color::CyanBright));
            options.push(("С меня хватит!", Color::BlueBright));
            options.push(("ЧТО ДЕЛАТЬ ???", Color::BlueBright));
        }
        Location::Mausoleum => todo!(),
    }
    r.move_cursor_to(9, 0)?;
    dialog(r, &options)
}

fn display_character_stats<R: Renderer>(r: &mut R) -> Result<(), R::Error> {
    r.set_color(Color::White, Color::Black)?;
    write!(r, "Сегодня ")?;
    r.set_color(Color::WhiteBright, Color::Black)?;
    write!(r, "{}", 22 /* TODO: Write actual date */)?;
    r.set_color(Color::White, Color::Black)?;
    write!(r, "е мая; ")?;
    r.set_color(Color::WhiteBright, Color::Black)?;
    write!(r, "{}:00    ", 8 /* TODO: Write actual time */)?;
    r.set_color(Color::MagentaBright, Color::Black)?;
    write!(r, "Версия gamma3.14")?;
    writeln!(r)?;
    r.set_color(Color::White, Color::Black)?;
    write!(r, "Самочувствие: ")?;
    {
        // TODO: Write actual health
        r.set_color(Color::Green, Color::Black)?;
        writeln!(r, "отличное")?;
    }
    r.set_color(Color::White, Color::Black)?;
    write!(r, "Финансы: ")?;
    {
        // TODO: Write actual money
        r.set_color(Color::Red, Color::Black)?;
        writeln!(r, "Надо получить деньги за май...")?;
    }
    {
        // TODO: Write actual brain
        r.set_color(Color::Green, Color::Black)?;
        writeln!(r, "Голова свежая")?;
    }
    {
        // TODO: Write actual stamina
        r.set_color(Color::Green, Color::Black)?;
        writeln!(r, "Готов к труду и обороне")?;
    }
    {
        // TODO: Write actual charisma
        r.set_color(Color::YellowBright, Color::Black)?;
        write!(r, "Ты нормально относишься к окружающим")?;
    }
    Ok(())
}

fn display_knowledge<R: Renderer>(r: &mut R) -> Result<(), R::Error> {
    for (i, (subject, _)) in SUBJECTS.iter().enumerate() {
        let line = i as i32;
        r.move_cursor_to(line, 45)?;
        r.set_color(Color::CyanBright, Color::Black)?;
        write!(r, "{}", subject_name(*subject))?;
        r.move_cursor_to(line, 67)?;
        r.set_color(Color::Cyan, Color::Black)?;

        // TODO: Write actual knowledge level
        write!(r, "{}", 0)?;

        r.move_cursor_to(line, 71)?;

        // TODO: Write actual knowledge level description
        write!(r, "Плохо")?;
    }
    Ok(())
}

fn display_short_today_timetable<R: Renderer>(
    r: &mut R,
    timetable: &timetable::Timetable,
) -> Result<(), R::Error> {
    let today = &timetable.days()[0]; // TODO: Actual today
    for (i, (subject, subject_info)) in SUBJECTS.iter().enumerate() {
        let line = (i as i32) + 9;
        r.move_cursor_to(line, 50)?;

        // TODO: If passed the exam, set different colors for everything!!!

        r.set_color(Color::CyanBright, Color::Black)?;
        write!(r, "{}", subject_short_name(*subject))?;
        r.move_cursor_to(line, 58)?;
        r.set_color(Color::Red, Color::Black)?;
        if let Some(exam) = today.exam(*subject) {
            write!(r, "{}", exam.location())?;
            r.set_color(Color::WhiteBright, Color::Black)?;
            r.move_cursor_to(line, 64)?;
            write!(r, "{}-{}", exam.from(), exam.to())?;
        } else {
            write!(r, "----")?;
        }
        r.move_cursor_to(line, 72)?;
        r.set_color(Color::White, Color::Black)?;

        // TODO: Write the actual number of completed problems.
        write!(r, "{}/{}", 0, subject_info.required_problems())?;
    }
    Ok(())
}
