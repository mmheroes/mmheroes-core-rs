pub mod timetable;
pub use timetable::*;

#[derive(Copy, Clone, Debug)]
pub enum Action {
    Exit = -1,
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
}

impl core::convert::TryFrom<i16> for Action {
    type Error = ();

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        use Action::*;
        match value {
            -1 => Ok(Exit),
            0 => Ok(_0),
            1 => Ok(_1),
            2 => Ok(_2),
            3 => Ok(_3),
            4 => Ok(_4),
            5 => Ok(_5),
            6 => Ok(_6),
            7 => Ok(_7),
            8 => Ok(_8),
            _ => Err(()),
        }
    }
}

macro_rules! invalid_action {
    ($from:literal, $to:literal) => {
        panic!(concat!(
            "invalid action, expected from ",
            $from,
            " to ",
            $to
        ))
    };
}

#[derive(Clone, Debug)]
pub struct GameState {
    player: Player,
    timetable: timetable::Timetable,
    location: Location,
}

impl GameState {
    pub fn player(&self) -> &Player {
        &self.player
    }

    pub fn timetable(&self) -> &timetable::Timetable {
        &self.timetable
    }

    pub fn location(&self) -> Location {
        self.location
    }
}

pub enum GameScreen {
    Start,
    Terminal,
    Intro,
    InitialParameters,
    Ding(Player),
    Timetable(GameState),
    SceneRouter(GameState),
    WhatToDo(GameState),
    AboutScreen(GameState),
    WhereToGoAndWhy(GameState),
    AboutProfessors(GameState),
    AboutCharacters(GameState),
    AboutThisProgram(GameState),
}

macro_rules! define_characteristic {
    ($name:ident) => {
        #[repr(transparent)]
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug)]
        struct $name(i8);

        impl core::ops::AddAssign<i8> for $name {
            fn add_assign(&mut self, rhs: i8) {
                self.0 += rhs
            }
        }

        impl core::ops::SubAssign<i8> for $name {
            fn sub_assign(&mut self, rhs: i8) {
                self.0 -= rhs
            }
        }
    };
}

define_characteristic!(BrainLevel);
define_characteristic!(StaminaLevel);
define_characteristic!(CharismaLevel);

#[derive(Debug, Clone)]
pub struct Player {
    // subject: []
    god_mode: bool,
    garlic: i16,
    has_mmheroes_floppy: bool,
    has_internet: bool,
    is_invited: bool,
    inception: bool,
    employed_at_terkom: bool,
    got_stipend: bool,
    has_ticket: bool,
    knows_djug: bool,

    brain: BrainLevel,
    stamina: StaminaLevel,
    charisma: CharismaLevel,
    exams_left: i16,
}

impl Player {
    fn new(
        god_mode: bool,
        brain: BrainLevel,
        stamina: StaminaLevel,
        charisma: CharismaLevel,
    ) -> Player {
        Player {
            god_mode,
            garlic: 0,
            has_mmheroes_floppy: false,
            has_internet: false,
            is_invited: false,
            inception: false,
            employed_at_terkom: false,
            got_stipend: false,
            has_ticket: false,
            knows_djug: false,
            brain,
            stamina,
            charisma,
            exams_left: 0,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Location {
    PUNK = 1,
    PDMI = 2,
    ComputerClass = 3,
    Dorm = 4,
    Mausoleum = 5,
}

/// The game mode selector.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[repr(C)]
pub enum GameMode {
    /// Normal game mode, the character has average characteristics.
    /// This is the default.
    Normal,

    /// The player will be prompted to select initial character characteristics:
    /// - Random student, same as `Normal` mode.
    /// - Clever student: better brain, but worse stamina and charisma
    /// - Impudent student: better stamina, but worse brain and charisma
    /// - Sociable student: better charisma, but worse brain and stamina.
    SelectInitialParameters,

    /// Same as `SelectInitialParameters`, but another option is available —
    /// God mode. It enables maximum abilities.
    ///
    /// This mode is enable by passing a special flag to the executable.
    God,
}

#[derive(Debug)]
#[allow(non_snake_case)] // TODO: Remove this
pub struct SubjectInfo {
    required_problems: u8,
    exam_days: u16,
    exam_min_duration: Duration,
    exam_max_duration: Duration,
    exam_places: [Location; 3],

    // TODO: Rename
    member0xFA: i16,
    member0xFC: i16,
    member0x100: i16,
}

impl SubjectInfo {
    pub fn required_problems(&self) -> u8 {
        self.required_problems
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Subject {
    AlgebraAndNumberTheory,
    Calculus,
    GeometryAndTopology,
    ComputerScience,
    English,
    PhysicalEducation,
}

pub const NUM_SUBJECTS: usize = 6;

pub struct Subjects([(Subject, SubjectInfo); NUM_SUBJECTS]);

impl Subjects {
    const fn new() -> Subjects {
        use Location::*;
        use Subject::*;
        Subjects([
            (
                AlgebraAndNumberTheory,
                SubjectInfo {
                    required_problems: 12,
                    exam_days: 4,
                    exam_min_duration: Duration(2),
                    exam_max_duration: Duration(4),
                    exam_places: [PUNK, PUNK, PDMI],
                    member0xFA: 10,
                    member0xFC: 17,
                    member0x100: 3,
                },
            ),
            (
                Calculus,
                SubjectInfo {
                    required_problems: 10,
                    exam_days: 4,
                    exam_min_duration: Duration(2),
                    exam_max_duration: Duration(3),
                    exam_places: [PUNK, PUNK, PUNK],
                    member0xFA: 8,
                    member0xFC: 14,
                    member0x100: 2,
                },
            ),
            (
                GeometryAndTopology,
                SubjectInfo {
                    required_problems: 3,
                    exam_days: 2,
                    exam_min_duration: Duration(1),
                    exam_max_duration: Duration(3),
                    exam_places: [PUNK, PDMI, PDMI],
                    member0xFA: 4,
                    member0xFC: 8,
                    member0x100: 3,
                },
            ),
            (
                ComputerScience,
                SubjectInfo {
                    required_problems: 2,
                    exam_days: 2, // FIXME: May be 3.
                    exam_min_duration: Duration(1),
                    exam_max_duration: Duration(2),
                    exam_places: [ComputerClass, ComputerClass, ComputerClass],
                    member0xFA: 5,
                    member0xFC: 6,
                    member0x100: 3,
                },
            ),
            (
                English,
                SubjectInfo {
                    required_problems: 3,
                    exam_days: 2,
                    exam_min_duration: Duration(2),
                    exam_max_duration: Duration(2),
                    exam_places: [PUNK, PUNK, PUNK],
                    member0xFA: 7,
                    member0xFC: 10,
                    member0x100: 1,
                },
            ),
            (
                PhysicalEducation,
                SubjectInfo {
                    required_problems: 1,
                    exam_days: 2,
                    exam_min_duration: Duration(1),
                    exam_max_duration: Duration(1),
                    exam_places: [PUNK, PUNK, PUNK],
                    member0xFA: 7,
                    member0xFC: 20,
                    member0x100: 1,
                },
            ),
        ])
    }

    pub fn iter(&self) -> core::slice::Iter<'_, (Subject, SubjectInfo)> {
        self.0.iter()
    }
}

pub const SUBJECTS: Subjects = Subjects::new();

pub const HELP_SCREEN_OPTION_COUNT: usize = 7;

impl core::ops::Index<Subject> for Subjects {
    type Output = (Subject, SubjectInfo);

    fn index(&self, index: Subject) -> &Self::Output {
        &self.0[index as usize]
    }
}

use GameScreen::*;

pub struct Game {
    screen: GameScreen,
    rng: crate::random::Rng,
    mode: GameMode,
    available_actions: usize,
}

impl Game {
    pub fn new(mode: GameMode, seed: u64) -> Game {
        let rng = crate::random::Rng::new(seed);
        Game {
            screen: Start,
            rng,
            mode,
            available_actions: 1,
        }
    }

    pub fn screen(&self) -> &GameScreen {
        &self.screen
    }

    pub fn mode(&self) -> GameMode {
        self.mode
    }

    pub fn available_actions(&self) -> usize {
        self.available_actions
    }

    pub fn perform_action(&mut self, action: Action) {
        assert!(
            (action as usize) < self.available_actions,
            "Unexpected action"
        );
        self.available_actions = self._perform_action(action)
    }

    /// Accepts an action, returns the number of actions available in the updated state.
    fn _perform_action(&mut self, action: Action) -> usize {
        match &self.screen {
            Start => {
                self.screen = Intro;
                // Начальный экран. Нажми любую клавишу.
                1
            }
            Terminal => 0,
            Intro => match self.mode {
                GameMode::SelectInitialParameters => {
                    self.screen = InitialParameters;
                    // Можно выбрать 4 стиля игры:
                    // - Случайный студент
                    // - Шибко умный
                    // - Шибко наглый
                    // - Шибко общительный
                    4
                }
                GameMode::God => {
                    self.screen = InitialParameters;
                    // Можно выбрать 5 стилей игры:
                    // - Случайный студент
                    // - Шибко умный
                    // - Шибко наглый
                    // - Шибко общительный
                    // - GOD-режим
                    5
                }
                GameMode::Normal => {
                    self.ding(Action::_0 /* Случайный студент */)
                }
            },
            InitialParameters => self.ding(action),
            Ding(player) => {
                let state = GameState {
                    player: player.clone(),
                    timetable: timetable::Timetable::random(&mut self.rng),
                    location: Location::Dorm,
                };
                self.view_timetable(state)
            }
            Timetable(state) => {
                let state = state.clone();
                self.scene_router(state)
            }
            SceneRouter(state) => {
                let state = state.clone();
                self.handle_scene_router_action(state, action)
            }
            WhatToDo(state)
            | AboutScreen(state)
            | WhereToGoAndWhy(state)
            | AboutProfessors(state)
            | AboutCharacters(state)
            | AboutThisProgram(state) => {
                let state = state.clone();
                self.handle_what_to_do(state, action)
            }
        }
    }

    fn ding(&mut self, action: Action) -> usize {
        self.screen = Ding(self.initialize_player(action));
        // "Нажми любую клавишу ..."
        1
    }

    fn initialize_player(&mut self, parameters: Action) -> Player {
        match parameters {
            // "Случайный студент"
            Action::_0 => Player::new(
                false,
                BrainLevel(self.rng.random_number_in_range(4..7)),
                StaminaLevel(self.rng.random_number_in_range(4..7)),
                CharismaLevel(self.rng.random_number_in_range(4..7)),
            ),
            // "Шибко умный"
            Action::_1 => Player::new(
                false,
                BrainLevel(self.rng.random_number_in_range(5..10)),
                StaminaLevel(self.rng.random_number_in_range(2..5)),
                CharismaLevel(self.rng.random_number_in_range(2..5)),
            ),
            // "Шибко наглый"
            Action::_2 => Player::new(
                false,
                BrainLevel(self.rng.random_number_in_range(2..5)),
                StaminaLevel(self.rng.random_number_in_range(5..10)),
                CharismaLevel(self.rng.random_number_in_range(2..5)),
            ),
            // "Шибко общительный"
            Action::_3 => Player::new(
                false,
                BrainLevel(self.rng.random_number_in_range(2..5)),
                StaminaLevel(self.rng.random_number_in_range(2..5)),
                CharismaLevel(self.rng.random_number_in_range(5..10)),
            ),
            // "GOD-режим"
            Action::_4 => {
                Player::new(false, BrainLevel(30), StaminaLevel(30), CharismaLevel(30))
            }
            _ => invalid_action!(0, 4),
        }
    }

    fn scene_router(&mut self, state: GameState) -> usize {
        // TODO: assert that no exam is in progress
        match state.location() {
            Location::PDMI => todo!(),
            Location::PUNK => todo!(),
            Location::Mausoleum => todo!(),
            Location::Dorm => {
                self.screen = GameScreen::SceneRouter(state);
                9
            }
            Location::ComputerClass => todo!(),
        }
    }

    fn handle_scene_router_action(&mut self, state: GameState, action: Action) -> usize {
        match state.location() {
            Location::PUNK => todo!(),
            Location::PDMI => todo!(),
            Location::ComputerClass => todo!(),
            Location::Dorm => match action {
                Action::_0 => todo!("Study"),
                Action::_1 => self.view_timetable(state),
                Action::_2 => todo!("Rest"),
                Action::_3 => todo!("Go to bed"),
                Action::_4 => todo!("Go to PUNK"),
                Action::_5 => todo!("Go to PDMI"),
                Action::_6 => todo!("Go to mausoleum"),
                Action::_7 => todo!("I'm done!"),
                Action::_8 => {
                    self.screen = WhatToDo(state);
                    HELP_SCREEN_OPTION_COUNT
                }
                _ => invalid_action!(0, 8),
            },
            Location::Mausoleum => todo!(),
        }
    }

    fn view_timetable(&mut self, state: GameState) -> usize {
        self.screen = Timetable(state);
        // "Нажми любую клавишу ..."
        1
    }

    fn handle_what_to_do(&mut self, state: GameState, action: Action) -> usize {
        assert_eq!(state.location(), Location::Dorm);
        match action {
            Action::_0 => {
                // "А что вообще делать?"
                self.screen = WhatToDo(state)
            }
            Action::_1 => {
                // "Об экране"
                self.screen = AboutScreen(state)
            }
            Action::_2 => {
                // "Куда и зачем ходить?"
                self.screen = WhereToGoAndWhy(state)
            }
            Action::_3 => {
                // "О преподавателях"
                self.screen = AboutProfessors(state)
            }
            Action::_4 => {
                // "О персонажах"
                self.screen = AboutCharacters(state)
            }
            Action::_5 => {
                // "Об этой программе"
                self.screen = AboutThisProgram(state)
            }
            Action::_6 => {
                // "Спасибо, ничего"
                // Возвращаемся на главный экран
                return self.scene_router(state);
            }
            _ => invalid_action!(0, 6),
        };
        HELP_SCREEN_OPTION_COUNT
    }
}
