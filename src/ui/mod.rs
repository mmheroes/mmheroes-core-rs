pub mod recorded_input;
pub use recorded_input::*;

use crate::logic::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(C)]
pub enum Color {
    Black = 0,
    Yellow = 3,
    White = 7,
    Gray = 8,
    Red = 9,
    Green = 10,
    YellowBright = 11,
    Cyan = 14,
    WhiteBright = 15,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Input {
    KeyUp,
    KeyDown,
    Enter,
    Other,
    EOF,
}

impl Default for Input {
    fn default() -> Self {
        Input::EOF
    }
}

pub trait Renderer {
    type Error;

    fn clear_screen(&mut self) -> Result<(), Self::Error>;

    fn flush(&mut self) -> Result<(), Self::Error>;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error>;

    fn move_cursor_to(&mut self, line: i32, column: i32) -> Result<(), Self::Error>;

    /// Return the cursor position. This method not necessarily returns the actual cursor position.
    /// For example, `RecordedInputRenderer` always returns `(0, 0)`.
    /// Because of that, callers should not make decisions based on what this method returns.
    /// It only should be used to save the position in order to move to it later.
    fn get_cursor_position(&mut self) -> Result<(i32, i32), Self::Error>;

    fn set_color(&mut self, foreground: Color, background: Color) -> Result<(), Self::Error>;

    fn getch(&mut self) -> Result<Input, Self::Error>;

    fn sleep_ms(&mut self, ms: Milliseconds) -> Result<(), Self::Error>;

    fn write_fmt(&mut self, fmt: core::fmt::Arguments) -> Result<(), Self::Error> {
        use core::fmt::{write as fmt_write, Error as FmtError, Result as FmtResult, Write};

        // Create a shim which translates a Renderer to a core::fmt::Write and saves
        // off renderer errors. instead of discarding them
        struct Adaptor<'a, T: Renderer + ?Sized + 'a> {
            inner: &'a mut T,
            error: Result<(), T::Error>,
        }

        impl<T: Renderer + ?Sized> Write for Adaptor<'_, T> {
            fn write_str(&mut self, s: &str) -> FmtResult {
                match self.inner.write_str(s) {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        self.error = Err(e);
                        Err(FmtError)
                    }
                }
            }
        }

        let mut output = Adaptor {
            inner: self,
            error: Ok(()),
        };
        match fmt_write(&mut output, fmt) {
            Ok(()) => Ok(()),
            Err(..) => Err(output.error.expect_err("formatter error")),
        }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Milliseconds(pub i32);

pub struct GameUI<'r, R: Renderer> {
    renderer: &'r mut R,
    game: Game,
}

impl<'r, R: Renderer> GameUI<'r, R> {
    pub fn new(renderer: &'r mut R, game: Game) -> Self {
        Self { renderer, game }
    }

    pub fn run(&mut self) -> Result<(), R::Error> {
        loop {
            use GameState::*;
            self.renderer.clear_screen()?;
            let action = match self.game.state() {
                Start => Action::_0,
                Terminal => break,
                Intro => display_intro(self.renderer)?,
                InitialParameters => display_initial_parameters(self.renderer, self.game.mode())?,
                Ding(_) => display_ding(self.renderer)?,
                GameState::Timetable(player, timetable) => {
                    display_timetable(self.renderer, timetable)?
                }
                SceneRouter(player, location) => display_scene_router(self.renderer, *location)?,
            };
            self.game.perform_action(action);
        }
        Ok(())
    }
}

fn sleep<R: Renderer>(r: &mut R, ms: Milliseconds) -> Result<(), R::Error> {
    r.flush()?;
    r.sleep_ms(ms)
}

fn wait_for_any_key<R: Renderer>(r: &mut R) -> Result<Action, R::Error> {
    r.move_cursor_to(23, 0)?;
    r.set_color(Color::YellowBright, Color::Black)?;
    write!(r, "Нажми любую клавишу ...")?;
    r.flush()?;
    if let Input::EOF = r.getch()? {
        Ok(Action::Exit)
    } else {
        Ok(Action::_0)
    }
}

fn display_intro<R: Renderer>(r: &mut R) -> Result<Action, R::Error> {
    r.set_color(Color::Gray, Color::Black)?;
    writeln!(r, "                                                Нам понятен этот смех")?;
    writeln!(r, "                                                Не попавших на Мат-Мех")?;
    writeln!(r, "                                                  (надпись на парте)")?;
    writeln!(r)?;
    writeln!(r)?;
    writeln!(r)?;
    r.set_color(Color::WhiteBright, Color::Black)?;
    writeln!(r, " H H  EEE  RR    O   EEE  SS       M   M  A   A TTTTT       M   M  EEE  X   X")?;
    writeln!(r, " H H  E    R R  O O  E   S         MM MM  AAAAA   T         MM MM    E   X X")?;
    writeln!(r, " HHH  EE   RR   O O  EE   S    OF  M M M  A   A   T    &&&  M M M   EE    X")?;
    writeln!(r, " H H  E    R R  O O  E     S       M   M   A A    T         M   M    E   X X")?;
    writeln!(r, " H H  EEE  R R   O   EEE SS        M   M    A     T         M   E  EEE  X   X")?;
    writeln!(r)?;
    writeln!(r)?;
    writeln!(r)?;
    r.set_color(Color::Red, Color::Black)?;
    writeln!(r, "                             ГЕРОИ МАТА И МЕХА ;)")?;
    writeln!(r)?;
    writeln!(r)?;
    r.set_color(Color::Cyan, Color::Black)?;
    writeln!(r, "(P) CrWMM Development Team, 2001.")?;
    writeln!(r, "Версия gamma3.14.")?;
    writeln!(r, "Загляните на нашу страничку: mmheroes.chat.ru !")?;
    wait_for_any_key(r)
}

fn dialog<R: Renderer>(r: &mut R, options: &[(&str, Color)]) -> Result<Action, R::Error> {
    use core::convert::{TryFrom, TryInto};

    let options_count: i16 = options.len().try_into().expect("Too many options given!");

    let mut current_choice = 0i16;
    let start = r.get_cursor_position()?;
    loop {
        let mut chosen_line_end_position = start;
        for (i, &(name, color)) in options.iter().enumerate() {
            if i == current_choice as usize {
                r.set_color(Color::Black, Color::White)?;
            } else {
                r.set_color(color, Color::Black)?;
            }
            write!(r, "{}", name)?;
            if i == current_choice as usize {
                chosen_line_end_position = r.get_cursor_position()?;
            }
            writeln!(r)?;
        }
        r.move_cursor_to(chosen_line_end_position.0, chosen_line_end_position.1)?;
        r.flush()?;
        match r.getch()? {
            Input::KeyDown => {
                current_choice = (options_count + current_choice + 1) % options_count;
            }
            Input::KeyUp => {
                current_choice = (options_count + current_choice - 1) % options_count;
            }
            Input::Enter => {
                return Ok(Action::try_from(current_choice).expect("Unexpected action number"))
            }
            Input::Other => (),
            Input::EOF => return Ok(Action::Exit),
        }
        r.move_cursor_to(start.0, start.1)?;
    }
}

fn display_initial_parameters<R: Renderer>(r: &mut R, mode: GameMode) -> Result<Action, R::Error> {
    debug_assert!(mode == GameMode::God || mode == GameMode::SelectInitialParameters);
    r.set_color(Color::White, Color::Black)?;
    writeln!(r, "Выбери начальные параметры своего \"героя\":")?;
    writeln!(r)?;

    let options = &[
        ("Случайный студент", Color::Cyan),
        ("Шибко умный", Color::Cyan),
        ("Шибко наглый", Color::Cyan),
        ("Шибко общительный", Color::Cyan),
        ("GOD-режим", Color::Cyan),
    ];

    dialog(
        r,
        if mode == GameMode::God {
            &options[..]
        } else {
            &options[..(options.len() - 1)]
        },
    )
}

fn display_ding<R: Renderer>(r: &mut R) -> Result<Action, R::Error> {
    r.set_color(Color::Green, Color::Black)?;
    writeln!(r, "ДЗИНЬ!")?;
    sleep(r, Milliseconds(500))?;
    r.set_color(Color::YellowBright, Color::Black)?;
    writeln!(r, "ДДДЗЗЗЗЗИИИИИИННННННЬ !!!!")?;
    sleep(r, Milliseconds(700))?;
    r.set_color(Color::Red, Color::Black)?;
    writeln!(r, "ДДДДДДЗЗЗЗЗЗЗЗЗЗЗЗЗИИИИИИИИИИННННННННННННЬ !!!!!!!!!!")?;
    sleep(r, Milliseconds(1000))?;
    r.set_color(Color::White, Color::Black)?;
    writeln!(r, "Ты просыпаешься от звонка будильника 22-го мая в 8:00.")?;
    writeln!(r, "Неожиданно ты осознаешь, что началась зачетная неделя,")?;
    writeln!(r, "а твоя готовность к этому моменту практически равна нулю.")?;
    writeln!(r, "Натягивая на себя скромное одеяние студента,")?;
    writeln!(r, "ты всматриваешься в заботливо оставленное соседом на стене")?;
    writeln!(r, "расписание: когда и где можно найти искомого препода ?")?;
    wait_for_any_key(r)
}

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

fn output_remaining_exams<R: Renderer>(r: &mut R, number_of_exams: usize) -> Result<(), R::Error> {
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

fn display_timetable<R: Renderer>(
    r: &mut R,
    timetable: &timetable::Timetable,
) -> Result<Action, R::Error> {
    for (i, (subject, subject_info)) in SUBJECTS.iter().enumerate() {
        let line = (i as i32) * TIMETABLE_ROW_HEIGHT + TIMETABLE_START_Y;
        r.move_cursor_to(line, TIMETABLE_START_X)?;
        r.set_color(Color::Green, Color::Black)?;
        writeln!(r, "{}", subject_info.professor())?;
        r.move_cursor_to(line + 1, TIMETABLE_START_X)?;
        r.set_color(Color::Cyan, Color::Black)?;
        write!(r, "{}", subject_info.name())?;

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

    r.set_color(Color::Cyan, Color::Black)?;
    for (i, day) in timetable.days().iter().enumerate() {
        r.move_cursor_to(
            0,
            (i as i32) * TIMETABLE_COLUMN_WIDTH + TIMETABLE_DAYS_START_X,
        )?;
        write!(r, "{}", day.date())?;
    }

    r.move_cursor_to(22, 0)?;
    // TODO: Output the actual number of remaining exams
    output_remaining_exams(r, NUM_SUBJECTS)?;
    wait_for_any_key(r)
}

fn display_scene_router<R: Renderer>(r: &mut R, location: Location) -> Result<Action, R::Error> {
    todo!()
}
