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

pub(crate) mod cp866_encoding;
pub mod recording;

pub mod high_scores;
use high_scores::HighScore;

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
        options: tiny_vec_ty![DialogOption; 16],
    },
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Milliseconds(pub i32);

pub struct GameUI<'game> {
    renderer: Renderer,
    game: &'game mut Game,
    pub high_scores: [HighScore; high_scores::SCORE_COUNT],
}

impl GameUI<'_> {
    pub fn new(
        game: &mut Game,
        high_scores: Option<[HighScore; high_scores::SCORE_COUNT]>,
    ) -> GameUI {
        let default_high_scores = high_scores::default_high_scores();
        GameUI {
            renderer: Renderer::new(),
            game,
            high_scores: high_scores.unwrap_or(default_high_scores),
        }
    }

    pub fn continue_game(&mut self, input: Input) -> bool {
        use GameScreen::*;

        self.renderer.clear();

        if let Some(ref waiting_state) = self.renderer.waiting_state {
            let waiting_state = waiting_state.clone();

            let action = match waiting_state {
                WaitingState::PressAnyKey => Action::AnyKey,
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
                                Some(current_choice),
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
                                Some(current_choice),
                                &*options,
                            );
                            self.renderer.waiting_state = Some(WaitingState::Dialog {
                                current_choice,
                                start,
                                options,
                            });
                            return true;
                        }
                        Input::Enter => {
                            display_dialog(&mut self.renderer, start, None, &*options);
                            options[current_choice as usize].2
                        }
                        Input::Other => return true, // Do nothing
                    }
                }
            };

            self.game.perform_action(action);
        }

        let new_waiting_state = match self.game.screen() {
            Intro => screens::initial::display_intro(&mut self.renderer),
            InitialParameters => screens::initial::display_initial_parameters(
                &mut self.renderer,
                self.game.available_actions(),
                self.game.mode(),
            ),
            Ding(_) => screens::initial::display_ding(&mut self.renderer),
            GameScreen::Timetable(state) => {
                screens::timetable::display_timetable(&mut self.renderer, state)
            }
            SceneRouter(state) => screens::scene_router::display_scene_router(
                &mut self.renderer,
                self.game.available_actions(),
                state,
            ),
            Sleep(state) => {
                screens::scene_router::display_sleeping(&mut self.renderer, state)
            }
            HighScores(_) => screens::high_scores::display_high_scores(
                &mut self.renderer,
                &self.high_scores,
            ),
            RestInMausoleum(state) => screens::rest::display_rest_in_mausoleum(
                &mut self.renderer,
                self.game.available_actions(),
                state,
            ),
            KolyaInteraction(state, interaction) => {
                screens::npc::display_kolya_interaction(
                    &mut self.renderer,
                    state,
                    self.game.available_actions(),
                    *interaction,
                )
            }
            PashaInteraction(state, interaction) => {
                screens::npc::display_pasha_interaction(
                    &mut self.renderer,
                    state,
                    *interaction,
                )
            }
            GrishaInteraction(state, interaction) => {
                screens::npc::display_grisha_interaction(
                    &mut self.renderer,
                    state,
                    self.game.available_actions(),
                    *interaction,
                )
            }
            KuzmenkoInteraction(state, interaction) => {
                screens::npc::display_kuzmenko_interaction(
                    &mut self.renderer,
                    state,
                    *interaction,
                )
            }
            GoToProfessor(state) => {
                screens::scene_router::display_available_professors(&mut self.renderer, state, self.game.available_actions())
            },
            Exam(state, subject) => todo!(),
            SurfInternet(state, found_program) => {
                screens::scene_router::display_surfing_internet(
                    &mut self.renderer,
                    state,
                    *found_program,
                )
            }
            IAmDone(_) => screens::game_end::display_i_am_done(
                &mut self.renderer,
                self.game.available_actions(),
            ),
            GameEnd(state) => {
                screens::game_end::display_game_end(&mut self.renderer, state)
            }
            WannaTryAgain => screens::game_end::display_wanna_try_again(
                &mut self.renderer,
                self.game.available_actions(),
            ),
            Disclaimer => screens::game_end::display_disclaimer(&mut self.renderer),
            WhatToDo(_) => screens::help::display_what_to_do(
                &mut self.renderer,
                self.game.available_actions(),
            ),
            AboutScreen(_) => screens::help::display_about_screen(
                &mut self.renderer,
                self.game.available_actions(),
            ),
            WhereToGoAndWhy(_) => screens::help::display_where_to_go_and_why(
                &mut self.renderer,
                self.game.available_actions(),
            ),
            AboutProfessors(_) => screens::help::display_about_professors(
                &mut self.renderer,
                self.game.available_actions(),
            ),
            AboutCharacters(_) => screens::help::display_about_characters(
                &mut self.renderer,
                self.game.available_actions(),
            ),
            AboutThisProgram(_) => screens::help::display_about_this_program(
                &mut self.renderer,
                self.game.available_actions(),
            ),
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

type DialogOption = (&'static str, Color, Action);

fn display_dialog(
    r: &mut Renderer,
    start: (Line, Column),
    current_choice: Option<u8>,
    options: &[DialogOption],
) {
    for (i, &(name, color, _)) in options.iter().enumerate() {
        r.move_cursor_to(start.0 + i as Line, start.1);
        r.set_color(color, Color::Black);
        write!(r, "{}", name);
    }
    if let Some(current_choice) = current_choice {
        r.move_cursor_to(start.0 + current_choice, start.1);
        r.set_color(Color::Black, Color::White);
        write!(r, "{}", options[current_choice as usize].0);
    }
    r.flush();
}

fn dialog(r: &mut Renderer, available_actions: &[Action]) -> WaitingState {
    let options = dialog_options_for_actions(available_actions);
    let start = r.get_cursor_position();
    let current_choice = 0;
    display_dialog(r, start, Some(current_choice), &*options);
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

pub fn classmate_name(classmate: Classmate) -> &'static str {
    match classmate {
        Classmate::Kolya => "Коля",
        Classmate::Pasha => "Паша",
        Classmate::Diamond => "Diamond",
        Classmate::RAI => "RAI",
        Classmate::Misha => "Миша",
        Classmate::Serj => "Серж",
        Classmate::Sasha => "Саша",
        Classmate::NiL => "NiL",
        Classmate::Kuzmenko => "Кузьменко В.Г.",
        Classmate::DJuG => "DJuG",
        Classmate::Andrew => "Эндрю",
        Classmate::Grisha => "Гриша",
    }
}

fn dialog_option_for_action(action: Action) -> DialogOption {
    let option_name = match action {
        Action::Yes => "Да",
        Action::No => "Нет",
        Action::InteractWithClassmate(classmate) => {
            return (classmate_name(classmate), Color::YellowBright, action);
        }
        Action::Exam(subject) => {
            if subject == Subject::ComputerScience {
                return (professor_name(subject), Color::YellowBright, action);
            } else {
                professor_name(subject)
            }
        }
        Action::DontGoToProfessor => "Ни к кому",
        Action::RandomStudent => "Случайный студент",
        Action::CleverStudent => "Шибко умный",
        Action::ImpudentStudent => "Шибко наглый",
        Action::SociableStudent => "Шибко общительный",
        Action::GodMode => "GOD-режим",
        Action::Study => "Готовиться",
        Action::ViewTimetable => "Посмотреть расписание",
        Action::Rest => "Отдыхать",
        Action::GoToBed => "Лечь спать",
        Action::GoFromPunkToDorm => "Пойти в общагу",
        Action::GoFromDormToPunk => "Пойти на факультет",
        Action::GoFromMausoleumToDorm => "Идти в общагу",
        Action::RestByOurselvesInMausoleum => "Расслабляться будем своими силами.",
        Action::NoRestIsNoGood => "Нет, отдыхать - это я зря сказал.",
        Action::GoFromMausoleumToPunk => "Идти в ПУНК",
        Action::GoToComputerClass => "Пойти в компьютерный класс",
        Action::LeaveComputerClass => "Покинуть класс",
        Action::GoToPDMI => "Поехать в ПОМИ",
        Action::GoToMausoleum => "Пойти в мавзолей",
        Action::GoToCafePUNK => "Сходить в кафе",
        Action::SurfInternet => "Провести 1 час в Inet'е",
        Action::PlayMMHEROES => "Поиграть в MMHEROES",
        Action::GoToProfessor => "Идти к преподу",
        Action::GoToWork => "Пойти в ТЕРКОМ, поработать",
        Action::LookAtBaobab => "Посмотреть на баобаб",
        Action::OrderCola => "Стакан колы за 4 р.",
        Action::OrderSoup => "Суп, 6 р. все удовольствие",
        Action::OrderBeer => "0,5 пива за 8 р.",
        Action::AcceptEmploymentAtTerkom => "Да, мне бы не помешало.",
        Action::DeclineEmploymentAtTerkom => "Нет, я лучше поучусь уще чуток.",
        Action::IAmDone => {
            return ("С меня хватит!", Color::BlueBright, action);
        }
        Action::NoIAmNotDone => "Нет, не хочу!",
        Action::IAmCertainlyDone => "Я же сказал: с меня хватит!",
        Action::WantToTryAgain => "ДА!!! ДА!!! ДА!!!",
        Action::DontWantToTryAgain => "Нет... Нет... Не-э-эт...",
        Action::WhatToDo => {
            return ("ЧТО ДЕЛАТЬ ???", Color::BlueBright, action);
        }
        Action::WhatToDoAtAll => " А что вообще делать? ",
        Action::AboutScreen => " Об экране            ",
        Action::WhereToGoAndWhy => " Куда и зачем ходить? ",
        Action::AboutProfessors => " О преподавателях     ",
        Action::AboutCharacters => " О персонажах         ",
        Action::AboutThisProgram => " Об этой программе    ",
        Action::ThanksButNothing => " Спасибо, ничего      ",
        Action::AnyKey => panic!("Action {:?} cannot be used in a dialog", action),
    };
    (option_name, Color::CyanBright, action)
}

fn dialog_options_for_actions(actions: &[Action]) -> tiny_vec_ty![DialogOption; 16] {
    actions
        .iter()
        .cloned()
        .map(dialog_option_for_action)
        .collect()
}
