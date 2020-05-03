use crate::logic::*;
use crate::ui::*;

fn output_remaining_problems<R: Renderer>(
    r: &mut R,
    number_of_problems: u8,
) -> Result<(), R::Error> {
    let (line, column) = r.get_cursor_position()?;
    r.set_color(Color::White, Color::Black)?;
    write!(r, "Осталось")?;
    r.move_cursor_to(line + 1, column)?;
    r.set_color(Color::WhiteBright, Color::Black)?;
    // TODO: Output the actual number of remaining problems
    write!(r, "{}", number_of_problems)?;
    r.set_color(Color::White, Color::Black)?;
    match number_of_problems {
        1 => write!(r, " задание"),
        2..=4 => write!(r, " задания"),
        _ => write!(r, " заданий"),
    }
}

fn output_remaining_exams<R: Renderer>(
    r: &mut R,
    number_of_exams: usize,
) -> Result<(), R::Error> {
    assert!(number_of_exams as usize <= NUM_SUBJECTS);

    let mut output = |a, b| {
        r.set_color(Color::White, Color::Black)?;
        write!(r, "{} ", a)?;
        r.set_color(Color::YellowBright, Color::Black)?;
        write!(r, "{}", number_of_exams)?;
        r.set_color(Color::White, Color::Black)?;
        write!(r, " {}", b)
    };

    match number_of_exams {
        0 => {
            r.set_color(Color::WhiteBright, Color::Black)?;
            return write!(r, "Все уже сдано!");
        }
        1 => output("Остался", "зачет!"),
        2..=4 => output("Осталось", "зачета."),
        _ => output("Осталось", "зачетов."),
    }
}

const TIMETABLE_START_X: i32 = 0;
const TIMETABLE_DAYS_START_X: i32 = 24;
const TIMETABLE_START_Y: i32 = 1;
const TIMETABLE_COLUMN_WIDTH: i32 = 7;
const TIMETABLE_ROW_HEIGHT: i32 = 3;
const TIMETABLE_REMAINING_PROBLEMS_X: i32 = 70;

fn display_timetable_cell<R: Renderer>(
    r: &mut R,
    day: &Day,
    subject: Subject,
) -> Result<(), R::Error> {
    let (line, column) = r.get_cursor_position()?;
    // TODO: Set a different color for today
    r.set_color(Color::White, Color::Black)?;
    if let Some(exam) = day.exam(subject) {
        write!(r, "{}", exam.location())?;
        r.move_cursor_to(line + 1, column)?;
        write!(r, "{}-{}", exam.from(), exam.to())
    } else {
        // TODO: Set a different color for today
        r.set_color(Color::Black, Color::Gray)?;
        write!(r, "      ")?;
        r.move_cursor_to(line + 1, column)?;
        write!(r, "      ")
    }
}

pub(in crate::ui) fn display_timetable<R: Renderer>(
    r: &mut R,
    timetable: &timetable::Timetable,
) -> Result<Action, R::Error> {
    for (i, (subject, subject_info)) in SUBJECTS.iter().enumerate() {
        let line = (i as i32) * TIMETABLE_ROW_HEIGHT + TIMETABLE_START_Y;
        r.move_cursor_to(line, TIMETABLE_START_X)?;
        r.set_color(Color::Green, Color::Black)?;
        writeln!(r, "{}", professor_name(*subject))?;
        r.move_cursor_to(line + 1, TIMETABLE_START_X)?;
        r.set_color(Color::CyanBright, Color::Black)?;
        write!(r, "{}", subject_name(*subject))?;

        for (j, day) in timetable.days().iter().enumerate() {
            r.move_cursor_to(
                line,
                (j as i32) * TIMETABLE_COLUMN_WIDTH + TIMETABLE_DAYS_START_X,
            )?;
            display_timetable_cell(r, day, *subject)?;
        }

        r.move_cursor_to(line, TIMETABLE_REMAINING_PROBLEMS_X)?;
        // TODO: Output the actual number of remaining problems
        output_remaining_problems(r, subject_info.required_problems())?;
    }

    r.set_color(Color::CyanBright, Color::Black)?;
    for (i, day) in timetable.days().iter().enumerate() {
        r.move_cursor_to(
            0,
            (i as i32) * TIMETABLE_COLUMN_WIDTH + TIMETABLE_DAYS_START_X,
        )?;
        write!(r, "{}", day_date(day))?;
    }

    r.move_cursor_to(22, 0)?;
    // TODO: Output the actual number of remaining exams
    output_remaining_exams(r, NUM_SUBJECTS)?;
    wait_for_any_key(r)
}
