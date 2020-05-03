pub mod recorded_input;
pub use recorded_input::*;

use crate::logic::*;
use crate::util::*;

use core::fmt::Display;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(C)]
pub enum Color {
    Black = 0,
    Yellow = 3,
    Cyan = 6,
    White = 7,
    Gray = 8,
    Red = 9,
    Green = 10,
    YellowBright = 11,
    BlueBright = 12,
    MagentaBright = 13,
    CyanBright = 14,
    WhiteBright = 15,
}


impl Default for Color {
    fn default() -> Self {
        Color::White
    }
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
            use GameScreen::*;
            self.renderer.clear_screen()?;
            let action = match self.game.screen() {
                Start => Action::_0,
                Terminal => break,
                Intro => display_intro(self.renderer)?,
                InitialParameters => display_initial_parameters(self.renderer, self.game.mode())?,
                Ding(_) => display_ding(self.renderer)?,
                GameScreen::Timetable(state) => {
                    display_timetable(self.renderer, state.timetable())?
                }
                SceneRouter(state) => display_scene_router(self.renderer, state)?,
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
    r.set_color(Color::CyanBright, Color::Black)?;
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
            r.move_cursor_to(start.0 + i as i32, start.1)?;
            if i == current_choice as usize {
                r.set_color(Color::Black, Color::White)?;
            } else {
                r.set_color(color, Color::Black)?;
            }
            write!(r, "{}", name)?;
            if i == current_choice as usize {
                chosen_line_end_position = r.get_cursor_position()?;
            }
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
        ("Случайный студент", Color::CyanBright),
        ("Шибко умный", Color::CyanBright),
        ("Шибко наглый", Color::CyanBright),
        ("Шибко общительный", Color::CyanBright),
        ("GOD-режим", Color::CyanBright),
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

pub fn professor_name(subject: Subject) -> &'static str {
    match subject {
        Subject::AlgebraAndNumberTheory => "Всемирнов М.А.",
        Subject::Calculus => "Дубцов Е.С.",
        Subject::GeometryAndTopology => "Подкорытов С.С.",
        Subject::ComputerScience => "Климов А.А.",
        Subject::English => "Влащенко Н.П.",
        Subject::PhysicalEducation => "Альбинский Е.Г.",
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Gender {
    Male,
    Female,
}

pub fn professor_gender(subject: Subject) -> Gender {
    match subject {
        Subject::AlgebraAndNumberTheory => Gender::Male,
        Subject::Calculus => Gender::Male,
        Subject::GeometryAndTopology => Gender::Male,
        Subject::ComputerScience => Gender::Male,
        Subject::English => Gender::Female,
        Subject::PhysicalEducation => Gender::Male,
    }
}

pub fn subject_name(subject: Subject) -> &'static str {
    match subject {
        Subject::AlgebraAndNumberTheory => "Алгебра и Т.Ч.",
        Subject::Calculus => "Мат. Анализ",
        Subject::GeometryAndTopology => "Геометрия и Топология",
        Subject::ComputerScience => "Информатика",
        Subject::English => "English",
        Subject::PhysicalEducation => "Физ-ра",
    }
}

pub fn subject_short_name(subject: Subject) -> &'static str {
    match subject {
        Subject::AlgebraAndNumberTheory => "АиТЧ",
        Subject::Calculus => "МатАн",
        Subject::GeometryAndTopology => "ГиТ",
        Subject::ComputerScience => "Инф",
        Subject::English => "ИнЯз",
        Subject::PhysicalEducation => "Физ-ра",
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = match self {
            Location::Dorm => "Общага",
            Location::PUNK => "ПУНК",
            Location::Mausoleum => "Мавзолей",
            Location::ComputerClass => "Компы",
            Location::PDMI => "ПОМИ",
        };
        f.write_fmt(format_args!("{}", name))
    }
}

pub fn day_date(day: &Day) -> &'static str {
    const DATES: [&str; NUM_DAYS] = ["22.5", "23.5", "24.5", "25.5", "26.5", "27.5"];
    DATES[day.index()]
}

fn display_scene_router<R: Renderer>(r: &mut R, state: &GameState) -> Result<Action, R::Error> {
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
