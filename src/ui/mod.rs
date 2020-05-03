pub mod renderer;
pub use renderer::*;

mod screens;

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
                Intro => screens::initial::display_intro(self.renderer)?,
                InitialParameters => screens::initial::display_initial_parameters(
                    self.renderer,
                    self.game.mode(),
                )?,
                Ding(_) => screens::initial::display_ding(self.renderer)?,
                GameScreen::Timetable(state) => screens::timetable::display_timetable(
                    self.renderer,
                    state.timetable(),
                )?,
                SceneRouter(state) => {
                    screens::scene_router::display_scene_router(self.renderer, state)?
                }
                IAmDone(_) => screens::game_end::display_i_am_done(self.renderer)?,
                GameEnd(state) => {
                    screens::game_end::display_game_end(self.renderer, state)?
                }
                WannaTryAgain => {
                    screens::game_end::display_wanna_try_again(self.renderer)?
                }
                Disclaimer => screens::game_end::display_disclaimer(self.renderer)?,
                WhatToDo(_) => screens::help::display_what_to_do(self.renderer)?,
                AboutScreen(_) => screens::help::display_about_screen(self.renderer)?,
                WhereToGoAndWhy(_) => {
                    screens::help::display_where_to_go_and_why(self.renderer)?
                }
                AboutProfessors(_) => {
                    screens::help::display_about_professors(self.renderer)?
                }
                AboutCharacters(_) => {
                    screens::help::display_about_characters(self.renderer)?
                }
                AboutThisProgram(_) => {
                    screens::help::display_about_this_program(self.renderer)?
                }
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
                return Ok(
                    Action::try_from(current_choice).expect("Unexpected action number")
                )
            }
            Input::Other => (),
            Input::EOF => return Ok(Action::Exit),
        }
        r.move_cursor_to(start.0, start.1)?;
    }
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
