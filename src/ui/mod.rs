#![macro_use]

macro_rules! write_colored {
    ($color:ident, $renderer:expr, $($arg:tt)*) => {{
        $renderer.set_color(Color::$color, Color::Black);
        write!($renderer, $($arg)*)
    }};
}

macro_rules! writeln_colored {
    ($color:ident, $renderer:expr, $($arg:tt)*) => {{
        $renderer.set_color(Color::$color, Color::Black);
        writeln!($renderer, $($arg)*);
    }};
}

mod screens;

pub mod renderer;
pub use renderer::RendererRequest;
use renderer::*;

use crate::logic::*;

use core::fmt::Display;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(C)]
pub enum Color {
    Black = 0,
    Red = 1,
    Yellow = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,
    Gray = 8,
    RedBright = 9,
    Green = 10,
    YellowBright = 11,
    BlueBright = 12,
    MagentaBright = 13,
    CyanBright = 14,
    WhiteBright = 15,
}

impl core::convert::TryFrom<u8> for Color {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let color = match value {
            0 => Color::Black,
            1 => Color::Red,
            3 => Color::Yellow,
            4 => Color::Blue,
            5 => Color::Magenta,
            6 => Color::Cyan,
            7 => Color::White,
            8 => Color::Gray,
            9 => Color::RedBright,
            10 => Color::Green,
            11 => Color::YellowBright,
            12 => Color::BlueBright,
            13 => Color::MagentaBright,
            14 => Color::CyanBright,
            15 => Color::WhiteBright,
            _ => return Err(()),
        };
        assert_eq!(color as u8, value);
        Ok(color)
    }
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
}

#[derive(Debug, Clone)]
enum WaitingState {
    PressAnyKey,
    Dialog {
        current_choice: u8,
        start: (Line, Column),
        options: tiny_vec_ty![(&'static str, Color); 16],
    },
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Milliseconds(pub i32);

pub struct GameUI<'game> {
    renderer: Renderer,
    game: &'game mut Game,
}

impl GameUI<'_> {
    pub fn new(game: &mut Game) -> GameUI {
        GameUI {
            renderer: Renderer::new(),
            game,
        }
    }

    pub fn continue_game(&mut self, input: Input) -> bool {
        use GameScreen::*;

        use core::convert::TryFrom;

        self.renderer.clear();

        if let Some(ref waiting_state) = self.renderer.waiting_state {
            let waiting_state = waiting_state.clone();

            let action = match waiting_state {
                WaitingState::PressAnyKey => Action::_0,
                WaitingState::Dialog {
                    current_choice,
                    start,
                    options,
                } => {
                    let option_count = options.len() as u8;
                    match input {
                        Input::KeyUp => {
                            let current_choice =
                                (option_count + current_choice - 1) % option_count;
                            display_dialog(
                                &mut self.renderer,
                                start,
                                current_choice,
                                &*options,
                            );
                            self.renderer.waiting_state = Some(WaitingState::Dialog {
                                current_choice,
                                start,
                                options,
                            });
                            return true;
                        }
                        Input::KeyDown => {
                            let current_choice =
                                (option_count + current_choice + 1) % option_count;
                            display_dialog(
                                &mut self.renderer,
                                start,
                                current_choice,
                                &*options,
                            );
                            self.renderer.waiting_state = Some(WaitingState::Dialog {
                                current_choice,
                                start,
                                options,
                            });
                            return true;
                        }
                        Input::Enter => Action::try_from(current_choice)
                            .expect("Unexpected action number"),
                        Input::Other => return true, // Do nothing
                    }
                }
            };

            self.game.perform_action(action);
        }

        self.renderer.clear_screen();
        let new_waiting_state = match self.game.screen() {
            Intro => screens::initial::display_intro(&mut self.renderer),
            InitialParameters => screens::initial::display_initial_parameters(
                &mut self.renderer,
                self.game.mode(),
            ),
            Ding(_) => screens::initial::display_ding(&mut self.renderer),
            GameScreen::Timetable(state) => {
                screens::timetable::display_timetable(&mut self.renderer, state)
            }
            SceneRouter(state) => {
                screens::scene_router::display_scene_router(&mut self.renderer, state)
            }
            IAmDone(_) => screens::game_end::display_i_am_done(&mut self.renderer),
            GameEnd(state) => {
                screens::game_end::display_game_end(&mut self.renderer, state)
            }
            WannaTryAgain => {
                screens::game_end::display_wanna_try_again(&mut self.renderer)
            }
            Disclaimer => screens::game_end::display_disclaimer(&mut self.renderer),
            WhatToDo(_) => screens::help::display_what_to_do(&mut self.renderer),
            AboutScreen(_) => screens::help::display_about_screen(&mut self.renderer),
            WhereToGoAndWhy(_) => {
                screens::help::display_where_to_go_and_why(&mut self.renderer)
            }
            AboutProfessors(_) => {
                screens::help::display_about_professors(&mut self.renderer)
            }
            AboutCharacters(_) => {
                screens::help::display_about_characters(&mut self.renderer)
            }
            AboutThisProgram(_) => {
                screens::help::display_about_this_program(&mut self.renderer)
            }
            Terminal => {
                self.renderer.waiting_state = None;
                return false;
            }
        };
        self.renderer.waiting_state = Some(new_waiting_state);
        true
    }

    pub fn requests(&self) -> RendererRequestIter<'_> {
        self.renderer.requests()
    }
}

type DialogOption = (&'static str, Color);

fn display_dialog(
    r: &mut Renderer,
    start: (Line, Column),
    current_choice: u8,
    options: &[DialogOption],
) {
    let mut chosen_line_end_position = start;
    for (i, &(name, color)) in options.iter().enumerate() {
        r.move_cursor_to(start.0 + i as Line, start.1);
        if i == current_choice as usize {
            r.set_color(Color::Black, Color::White);
        } else {
            r.set_color(color, Color::Black);
        }
        write!(r, "{}", name);
        if i == current_choice as usize {
            chosen_line_end_position = r.get_cursor_position();
        }
    }
    r.move_cursor_to(chosen_line_end_position.0, chosen_line_end_position.1);
    r.flush();
}

fn dialog(r: &mut Renderer, options: tiny_vec_ty![DialogOption; 16]) -> WaitingState {
    let start = r.get_cursor_position();
    let current_choice = 0;
    display_dialog(r, start, current_choice, &*options);
    WaitingState::Dialog {
        current_choice,
        start,
        options,
    }
}

fn sleep(r: &mut Renderer, ms: Milliseconds) {
    r.flush();
    r.sleep_ms(ms)
}

fn wait_for_any_key(r: &mut Renderer) -> WaitingState {
    r.move_cursor_to(23, 0);
    r.set_color(Color::YellowBright, Color::Black);
    write!(r, "Нажми любую клавишу ...");
    r.flush();
    WaitingState::PressAnyKey
}

fn inactive_dialog(r: &mut Renderer, options: &[(&str, Color)]) {
    let start = r.get_cursor_position();
    for (i, &(name, color)) in options.iter().enumerate() {
        r.move_cursor_to(start.0 + i as Line, start.1);
        r.set_color(color, Color::Black);
        write!(r, "{}", name);
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
        f.write_str(name)
    }
}

pub fn day_date(day: &Day) -> &'static str {
    const DATES: [&str; NUM_DAYS] = ["22.5", "23.5", "24.5", "25.5", "26.5", "27.5"];
    DATES[day.index()]
}
