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
            _ => Err(()),
        }
    }
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
}

impl Game {
    pub fn new(mode: GameMode, seed: u64) -> Game {
        let rng = crate::random::Rng::new(seed);
        Game {
            screen: Start,
            rng,
            mode,
        }
    }

    pub fn screen(&self) -> &GameScreen {
        &self.screen
    }

    pub fn mode(&self) -> GameMode {
        self.mode
    }

    /// Accepts an action, returns the number of actions available in the updated state.
    pub fn perform_action(&mut self, action: Action) -> usize {
        match &self.screen {
            Start => {
                self.screen = Intro;
                // Intro screen. Press any key to continue.
                1
            }
            Terminal => 0,
            Intro => match self.mode {
                GameMode::SelectInitialParameters => {
                    self.screen = InitialParameters;
                    // The player can choose from 4 initial states:
                    // - random student
                    // - clever student
                    // - impudent student
                    // - sociable student
                    4
                }
                GameMode::God => {
                    self.screen = InitialParameters;
                    // The player can choose from 5 initial states:
                    // - random student
                    // - clever student
                    // - impudent student
                    // - sociable student
                    // - god mode
                    5
                }
                GameMode::Normal => {
                    self.screen = Ding(self.initialize_player(Action::_0 /* random student */));
                    // Ding screen. Press any key to continue.
                    1
                }
            },
            InitialParameters => {
                self.screen = Ding(self.initialize_player(action));
                // Ding screen. Press any key to continue.
                1
            }
            Ding(player) => {
                self.screen = GameScreen::Timetable(GameState {
                    player: player.clone(),
                    timetable: timetable::Timetable::random(&mut self.rng),
                    location: Location::Dorm,
                });
                // Timetable screen. Press any key to continue.
                1
            }
            Timetable(state) => {
                let state = state.clone();
                self.make_scene_router(state)
            }
            SceneRouter(state) => {
                let state = state.clone();
                self.handle_scene_router_action(state, action)
            }
        }
    }

    fn initialize_player(&mut self, parameters: Action) -> Player {
        match parameters {
            // Average student
            Action::_0 => Player::new(
                false,
                BrainLevel(self.rng.random_number_in_range(4i8..7)),
                StaminaLevel(self.rng.random_number_in_range(4i8..7)),
                CharismaLevel(self.rng.random_number_in_range(4i8..7)),
            ),
            // Clever student
            Action::_1 => Player::new(
                false,
                BrainLevel(self.rng.random_number_in_range(5i8..10)),
                StaminaLevel(self.rng.random_number_in_range(2i8..5)),
                CharismaLevel(self.rng.random_number_in_range(2i8..5)),
            ),
            // Impudent student
            Action::_2 => Player::new(
                false,
                BrainLevel(self.rng.random_number_in_range(2i8..5)),
                StaminaLevel(self.rng.random_number_in_range(5i8..10)),
                CharismaLevel(self.rng.random_number_in_range(2i8..5)),
            ),
            // Sociable student
            Action::_3 => Player::new(
                false,
                BrainLevel(self.rng.random_number_in_range(2i8..5)),
                StaminaLevel(self.rng.random_number_in_range(2i8..5)),
                CharismaLevel(self.rng.random_number_in_range(5i8..10)),
            ),
            // God
            Action::_4 => Player::new(false, BrainLevel(30), StaminaLevel(30), CharismaLevel(30)),
            _ => panic!("invalid state, expected action from 0 to 4."),
        }
    }

    fn make_scene_router(&mut self, state: GameState) -> usize {
        // TODO: assert that no exam is in progress
        match state.location {
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
        todo!()
    }
}
