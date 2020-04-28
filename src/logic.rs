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

pub enum GameState {
    Start,
    Terminal,
    Intro,
    InitialParameters,
    Ding(Player),
    Timetable(Player, Timetable),
    SceneRouter(Player, Location),
}

pub struct Player {

}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Location {
    Dorm,
    Punk,
    Mausoleum,
    ComputerClass,
    Pomi,
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
                    self.state = Ding( self.initialize_player(Action::_0 /* random student */));
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
            Action::_0 => todo!("random student"),
            Action::_1 => todo!("clever student"),
            Action::_2 => todo!("impudent student"),
            Action::_3 => todo!("sociable student"),
            _ => unreachable!("invalid state, expected value 0..=4"),
        }
    }

    fn scene_router(&mut self, location: Location, action: Action) -> usize {
        todo!()
    }
}
