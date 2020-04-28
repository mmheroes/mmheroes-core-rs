pub type Milliseconds = u32;

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

impl std::convert::TryFrom<i16> for Action {
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
            _ => Err(())
        }
    }
}

pub enum GameState {
    Start,
    Terminal,
    Intro,
    InitialParameters,
    Ding(Player),
    Timetable(Player, Timetable),
    SceneRouter(Player, Location),
}

macro_rules! define_characteristic {
    ($name:ident) => {
        #[repr(transparent)]
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        struct $name(i16);

        impl std::ops::AddAssign<i16> for $name {
            fn add_assign(&mut self, rhs: i16) {
                self.0 += rhs
            }
        }

        impl std::ops::SubAssign<i16> for $name {
            fn sub_assign(&mut self, rhs: i16) {
                self.0 -= rhs
            }
        }
    };
}

define_characteristic!(BrainLevel);
define_characteristic!(StaminaLevel);
define_characteristic!(CharismaLevel);

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
            god_mode: false,
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
    Dorm,
    Punk,
    Mausoleum,
    ComputerClass,
    PDMI,
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

pub struct Timetable;

impl Timetable {
    fn random(rng: &mut crate::random::Rng) -> Timetable {
        Timetable // TODO
    }
}

use GameState::*;

pub struct Game {
    state: GameState,
    rng: crate::random::Rng,
    mode: GameMode,
}

impl Game {
    pub fn new(mode: GameMode, seed: u64) -> Game {
        let mut rng = crate::random::Rng::new(seed);
        Game {
            state: Start,
            rng,
            mode,
        }
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    pub fn mode(&self) -> GameMode {
        self.mode
    }

    /// Accepts an action, returns the number of actions available in the updated state.
    pub fn perform_action(&mut self, action: Action) -> usize {
        match &self.state {
            Start => {
                self.state = Intro;
                // Intro screen. Press any key to continue.
                1
            }
            Terminal => 0,
            Intro => match self.mode {
                GameMode::SelectInitialParameters => {
                    self.state = InitialParameters;
                    // The player can choose from 4 initial states:
                    // - random student
                    // - clever student
                    // - impudent student
                    // - sociable student
                    4
                }
                GameMode::God => {
                    self.state = InitialParameters;
                    // The player can choose from 5 initial states:
                    // - random student
                    // - clever student
                    // - impudent student
                    // - sociable student
                    // - god mode
                    5
                }
                GameMode::Normal => {
                    self.state = Ding(self.initialize_player(Action::_0 /* random student */));
                    // Ding screen. Press any key to continue.
                    1
                }
            },
            InitialParameters => {
                self.state = Ding(self.initialize_player(action));
                // Ding screen. Press any key to continue.
                1
            }
            Ding(player) => {
                self.state = GameState::Timetable(todo!(), Timetable::random(&mut self.rng));
                // Timetable screen. Press any key to continue.
                1
            }
            GameState::Timetable(player, timetable) => todo!(),
            SceneRouter(player, location) => self.scene_router(*location, action),
        }
    }

    fn initialize_player(&mut self, parameters: Action) -> Player {
        match parameters {
            // Average student
            Action::_0 => Player::new(
                false,
                BrainLevel(self.rng.random_number_in_range(4..7) as i16),
                StaminaLevel(self.rng.random_number_in_range(4..7) as i16),
                CharismaLevel(self.rng.random_number_in_range(4..7) as i16),
            ),
            // Clever student
            Action::_1 => Player::new(
                false,
                BrainLevel(self.rng.random_number_in_range(5..10) as i16),
                StaminaLevel(self.rng.random_number_in_range(2..5) as i16),
                CharismaLevel(self.rng.random_number_in_range(2..5) as i16),
            ),
            // Impudent student
            Action::_2 => Player::new(
                false,
                BrainLevel(self.rng.random_number_in_range(2..5) as i16),
                StaminaLevel(self.rng.random_number_in_range(5..10) as i16),
                CharismaLevel(self.rng.random_number_in_range(2..5) as i16),
            ),
            // Sociable student
            Action::_3 => Player::new(
                false,
                BrainLevel(self.rng.random_number_in_range(2..5) as i16),
                StaminaLevel(self.rng.random_number_in_range(2..5) as i16),
                CharismaLevel(self.rng.random_number_in_range(5..10) as i16),
            ),
            // God
            Action::_4 => Player::new(
                false,
                BrainLevel(30),
                StaminaLevel(30),
                CharismaLevel(30),
            ),
            _ => panic!("invalid state, expected action from 0 to 4."),
        }
    }

    fn scene_router(&mut self, location: Location, action: Action) -> usize {
        todo!()
    }
}
