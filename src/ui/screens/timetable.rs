use crate::logic::*;
use crate::ui::renderer::{Column, Line, Renderer};
use crate::ui::*;

fn output_remaining_problems(
    r: &mut Renderer,
    timetable: &timetable::Timetable,
    subject_status: &SubjectStatus,
) {
    let (line, column) = r.get_cursor_position();
    let problems_remaining = SUBJECTS[subject_status.subject()]
        .1
        .required_problems()
        .saturating_sub(subject_status.problems_done());
    if let Some(passed_day) = subject_status.passed_exam_day(timetable) {
        r.set_color(Color::WhiteBright, Color::Black);
        write!(r, "ЗАЧЕТ");
        r.move_cursor_to(line + 1, column);
        write!(r, "{}", day_date(passed_day))
    } else if problems_remaining == 0 {
        r.set_color(Color::White, Color::Black);
        write!(r, "Подойти с");
        r.move_cursor_to(line + 1, column);
        write!(r, "зачеткой")
    } else {
        r.set_color(Color::White, Color::Black);
        write!(r, "Осталось");
        r.move_cursor_to(line + 1, column);
        r.set_color(Color::WhiteBright, Color::Black);
        write!(r, "{}", problems_remaining);
        r.set_color(Color::White, Color::Black);
        match problems_remaining {
            0 => unreachable!(),
            1 => write!(r, " задание"),
            2..=4 => write!(r, " задания"),
            _ => write!(r, " заданий"),
        }
    }
}

fn output_remaining_exams(r: &mut Renderer, number_of_exams: usize) {
    assert!(number_of_exams as usize <= NUM_SUBJECTS);

    let mut output = |a, b| {
        r.set_color(Color::White, Color::Black);
        write!(r, "{} ", a);
        r.set_color(Color::YellowBright, Color::Black);
        write!(r, "{}", number_of_exams);
        r.set_color(Color::White, Color::Black);
        write!(r, " {}", b)
    };

    match number_of_exams {
        0 => {
            r.set_color(Color::WhiteBright, Color::Black);
            return write!(r, "Все уже сдано!");
        }
        1 => output("Остался", "зачет!"),
        2..=4 => output("Осталось", "зачета."),
        _ => output("Осталось", "зачетов."),
    }
}

const TIMETABLE_START_X: Column = 0;
const TIMETABLE_DAYS_START_X: Column = 24;
const TIMETABLE_START_Y: Line = 1;
const TIMETABLE_COLUMN_WIDTH: Column = 7;
const TIMETABLE_ROW_HEIGHT: Line = 3;
const TIMETABLE_REMAINING_PROBLEMS_X: Column = 70;

fn display_timetable_cell(
    r: &mut Renderer,
    day: &Day,
    today: bool,
    passed: bool,
    subject: Subject,
) {
    let (line, column) = r.get_cursor_position();
    if today {
        r.set_color(Color::YellowBright, Color::Black);
    } else {
        r.set_color(Color::White, Color::Black);
    }
    if let Some(exam) = day.exam(subject) {
        if passed {
            r.set_color(Color::Blue, Color::Black);
        }
        write!(r, "{}", exam.location());
        r.move_cursor_to(line + 1, column);
        write!(r, "{}-{}", exam.from(), exam.to())
    } else {
        if today {
            r.set_color(Color::Black, Color::Yellow);
        } else {
            r.set_color(Color::Black, Color::Gray);
        }
        write!(r, "      ");
        r.move_cursor_to(line + 1, column);
        write!(r, "      ")
    }
}

pub(in crate::ui) fn display_timetable(
    r: &mut Renderer,
    state: &GameState,
) -> WaitingState {
    r.clear_screen();
    let today = state.current_day();
    for (i, (subject, _)) in SUBJECTS.iter().enumerate() {
        let line = (i as Line) * TIMETABLE_ROW_HEIGHT + TIMETABLE_START_Y;
        r.move_cursor_to(line, TIMETABLE_START_X);
        r.set_color(Color::Green, Color::Black);
        writeln!(r, "{}", professor_name(*subject));
        r.move_cursor_to(line + 1, TIMETABLE_START_X);
        r.set_color(Color::CyanBright, Color::Black);
        write!(r, "{}", subject_name(*subject));

        for (j, day) in state.timetable().days().iter().enumerate() {
            r.move_cursor_to(
                line,
                (j as Column) * TIMETABLE_COLUMN_WIDTH + TIMETABLE_DAYS_START_X,
            );
            display_timetable_cell(
                r,
                day,
                day.index() == today.index(),
                state.player().status_for_subject(*subject).passed(),
                *subject,
            );
        }

        r.move_cursor_to(line, TIMETABLE_REMAINING_PROBLEMS_X);
        output_remaining_problems(
            r,
            state.timetable(),
            state.player().status_for_subject(*subject),
        );
    }

    r.set_color(Color::CyanBright, Color::Black);
    for (i, day) in state.timetable().days().iter().enumerate() {
        r.move_cursor_to(
            0,
            (i as Column) * TIMETABLE_COLUMN_WIDTH + TIMETABLE_DAYS_START_X,
        );
        write!(r, "{}", day_date(day));
    }

    r.move_cursor_to(22, 0);
    output_remaining_exams(r, state.player().exams_left());
    wait_for_any_key(r)
}
