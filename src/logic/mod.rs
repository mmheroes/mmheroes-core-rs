pub mod timetable;
pub use timetable::*;

pub mod characteristics;
pub use characteristics::*;

pub mod game_state;
pub use game_state::*;

pub mod subjects;
pub use subjects::*;

use crate::random;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CauseOfDeath {
    /// Умер по пути на факультет.
    OnTheWayToPUNK,

    /// Умер по пути в мавзолей.
    OnTheWayToMausoleum,

    /// Умер по пути домой.
    OnTheWayToDorm,

    /// Упал с лестницы у главного входа.
    FellFromStairs,

    /// Сгорел на работе
    Burnout,

    /// Заучился.
    Overstudied,

    /// Зубрежка до добра не доводит!
    StudiedTooWell,

    /// Не смог расстаться с компьютером.
    CouldntLeaveTheComputer,

    /// В электричке нашли бездыханное тело.
    CorpseFoundInTheTrain,

    /// Контролеры жизни лишили.
    KilledByInspectors,

    /// Заснул в электричке и не проснулся.
    FellAsleepInTheTrain,

    /// Раздвоение ложной личности.
    SplitPersonality,

    /// Пивной алкоголизм, батенька...
    BeerAlcoholism,

    /// Спился.
    DrankTooMuch,

    /// Губит людей не пиво, а избыток пива.
    DrankTooMuchBeer,

    /// Альтруизм не довел до добра.
    Altruism,

    /// Превратился в овощ.
    TurnedToVegetable,

    /// <препод> замучил.
    TorturedByProfessor(Subject),

    /// Бурно прогрессирующая паранойя
    Paranoia,

    /// Время вышло.
    TimeOut,

    /// Вышел сам.
    Suicide,

    /// Раздавлен безжалостной ошибкой в программе.
    SoftwareBug,
}

/// Максимальное число возможных вариантов на главном экране.
pub const MAX_OPTIONS_IN_SCENE_ROUTER: usize = 12;

pub enum GameScreen {
    /// Начальное состояние. Ему не соответствует никакой экран.
    Start,

    /// Терминальное состояние. Ему тоже не соответствует никакой экран.
    /// Игра завершена безвозвратно.
    Terminal,

    /// Самый первый экран, который видет пользователь.
    Intro,

    /// Экран, который видит пользователь, если запускает игру с каким-то аргументом.
    /// Предлагает выбрать стиль игры.
    InitialParameters,

    /// Экран с предысторией ("ты просыпаешься от звонка будильника...")
    Ding(Player),

    /// Экран с расписанием.
    Timetable(GameState),

    /// Главный экран.
    SceneRouter(GameState),

    /// Экран "ты серьёзно хочешь закончить игру?"
    IAmDone(GameState),

    /// Финальный экран с описанием причины смерти/отчисления, либо поздравлением.
    GameEnd(GameState),

    /// Пользователю предлагается либо повторить игру, либо выйти.
    WannaTryAgain,

    /// Экран, который отображается пользователю, если он решил выйти из игры.
    Disclaimer,

    /// Экран помощи с описанием цели игры.
    WhatToDo(GameState),

    /// Экран помощи с описанием главного экрана.
    AboutScreen(GameState),

    /// Экран помощи с описанием локаций.
    WhereToGoAndWhy(GameState),

    /// Экран помощи с описанием преподавателей.
    AboutProfessors(GameState),

    /// Экран помощи с описанием NPC-шек.
    AboutCharacters(GameState),

    /// Экран помощи с информацией о программе.
    AboutThisProgram(GameState),
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

pub const HELP_SCREEN_OPTION_COUNT: usize = 7;

use GameScreen::*;

pub struct Game {
    screen: GameScreen,
    rng: random::Rng,
    mode: GameMode,
    available_actions: usize,
}

impl Game {
    pub fn new(mode: GameMode, seed: u64) -> Game {
        let rng = random::Rng::new(seed);
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
                // Начальный экран. "Нажми любую клавишу".
                1
            }
            Terminal => panic!("Attempted to perform an action in terminal state"),
            Intro => self.start_game(),
            InitialParameters => self.ding(action),
            Ding(player) => {
                let state = GameState::new(
                    player.clone(),
                    timetable::Timetable::random(&mut self.rng),
                    Location::Dorm,
                );
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
            IAmDone(state) => {
                let state = state.clone();
                self.handle_i_am_done(state, action)
            }
            GameEnd(_) => self.wanna_try_again(),
            WannaTryAgain => self.handle_wanna_try_again(action),
            Disclaimer => {
                self.screen = Terminal;
                0
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

    fn start_game(&mut self) -> usize {
        match self.mode {
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
        }
    }

    fn ding(&mut self, action: Action) -> usize {
        self.screen = Ding(self.initialize_player(action));
        // "Нажми любую клавишу ..."
        1
    }

    fn initialize_player(&mut self, parameters: Action) -> Player {
        let (god_mode, brain, stamina, charisma) = match parameters {
            // "Случайный студент"
            Action::_0 => (
                false,
                BrainLevel(self.rng.random_number_in_range(4..7)),
                StaminaLevel(self.rng.random_number_in_range(4..7)),
                CharismaLevel(self.rng.random_number_in_range(4..7)),
            ),
            // "Шибко умный"
            Action::_1 => (
                false,
                BrainLevel(self.rng.random_number_in_range(5..10)),
                StaminaLevel(self.rng.random_number_in_range(2..5)),
                CharismaLevel(self.rng.random_number_in_range(2..5)),
            ),
            // "Шибко наглый"
            Action::_2 => (
                false,
                BrainLevel(self.rng.random_number_in_range(2..5)),
                StaminaLevel(self.rng.random_number_in_range(5..10)),
                CharismaLevel(self.rng.random_number_in_range(2..5)),
            ),
            // "Шибко общительный"
            Action::_3 => (
                false,
                BrainLevel(self.rng.random_number_in_range(2..5)),
                StaminaLevel(self.rng.random_number_in_range(2..5)),
                CharismaLevel(self.rng.random_number_in_range(5..10)),
            ),
            // "GOD-режим"
            Action::_4 => (true, BrainLevel(30), StaminaLevel(30), CharismaLevel(30)),
            _ => invalid_action!(0, 4),
        };

        let health =
            HealthLevel(self.rng.random_number_with_upper_bound(stamina.0 * 2) + 40);

        Player::new(god_mode, health, brain, stamina, charisma, |_| {
            self.rng.random_number_with_upper_bound(brain)
        })
    }

    fn scene_router(&mut self, state: GameState) -> usize {
        // TODO: assert that no exam is in progress
        let location = state.location;
        self.screen = GameScreen::SceneRouter(state);
        match location {
            Location::PDMI => todo!(),
            Location::PUNK => todo!(),
            Location::Mausoleum => {
                4 // TODO: This number should be based on what NPCs are available
            }
            Location::Dorm => 9,
            Location::ComputerClass => todo!(),
        }
    }

    fn handle_scene_router_action(&mut self, state: GameState, action: Action) -> usize {
        match state.location() {
            Location::PUNK => todo!(),
            Location::PDMI => todo!(),
            Location::ComputerClass => todo!(),
            Location::Dorm => self.handle_dorm_action(state, action),
            Location::Mausoleum => self.handle_mausoleum_action(state, action),
        }
    }

    fn handle_dorm_action(&mut self, mut state: GameState, action: Action) -> usize {
        assert!(state.location == Location::Dorm);
        match action {
            // Готовиться
            Action::_0 => {
                if state.failed_attempt_to_sleep {
                    state.failed_attempt_to_sleep = false;
                    self.scene_router(state)
                } else {
                    todo!("Study")
                }
            }
            // Посмотреть расписание
            Action::_1 => self.view_timetable(state),
            // Отдыхать
            Action::_2 => self.rest_in_dorm(state),
            // Лечь спать
            Action::_3 => self.try_to_sleep(state),
            // Пойти на факультет
            Action::_4 => {
                state.location = Location::PUNK;
                self.decrease_health(
                    3,
                    state,
                    CauseOfDeath::OnTheWayToPUNK,
                    /*if_alive=*/ |game, state| game.scene_router(state),
                )
            }
            // Поехать в ПОМИ
            Action::_5 => todo!("Go to PDMI"),
            // Пойти в мавзолей
            Action::_6 => {
                state.location = Location::Mausoleum;
                self.decrease_health(
                    3,
                    state,
                    CauseOfDeath::OnTheWayToMausoleum,
                    /*if_alive=*/ |game, state| game.scene_router(state),
                )
            }
            // С меня хватит!
            Action::_7 => self.i_am_done(state),
            // ЧТО ДЕЛАТЬ ???
            Action::_8 => {
                self.screen = WhatToDo(state);
                HELP_SCREEN_OPTION_COUNT
            }
            _ => invalid_action!(0, 8),
        }
    }

    fn handle_mausoleum_action(&mut self, mut state: GameState, action: Action) -> usize {
        todo!()
    }

    fn view_timetable(&mut self, state: GameState) -> usize {
        self.screen = Timetable(state);
        // "Нажми любую клавишу ..."
        1
    }

    fn decrease_health<F: FnOnce(&mut Game, GameState) -> usize>(
        &mut self,
        delta: u8,
        mut state: GameState,
        cause_of_death: CauseOfDeath,
        if_alive: F,
    ) -> usize {
        if state.player.health <= HealthLevel(delta) {
            state.player.cause_of_death = Some(cause_of_death);
            self.game_end(state)
        } else {
            state.player.health -= delta;
            if_alive(self, state)
        }
    }

    fn try_to_sleep(&mut self, mut state: GameState) -> usize {
        assert!(state.location == Location::Dorm);
        if state.current_time > Time(3) && state.current_time < Time(20) {
            state.failed_attempt_to_sleep = true;
            self.scene_router(state)
        } else {
            self.go_to_sleep(state)
        }
    }

    fn go_to_sleep(&mut self, _state: GameState) -> usize {
        todo!()
    }

    fn midnight(&mut self, state: GameState) -> usize {
        match state.location {
            Location::PUNK => todo!("sub_1E907"),
            Location::PDMI => todo!("sub_1E7F8"),
            Location::ComputerClass => {
                unreachable!("Компьютерный класс уже должен быть закрыт в полночь!")
            }
            Location::Dorm => self.go_to_sleep(state),
            Location::Mausoleum => todo!("sub_1E993"),
        }
    }

    fn hour_pass(&mut self, mut state: GameState) -> usize {
        // TODO: Lot of stuff going on here
        state.current_time += Duration(1);

        if state.current_time.is_midnight() {
            state.current_day_index += 1;
            state.current_time = Time(0);
            return self.midnight(state);
        }

        self.scene_router(state)
    }

    fn rest_in_dorm(&mut self, mut state: GameState) -> usize {
        state.player.health += self.rng.random_number_in_range(7..15);
        self.hour_pass(state)
    }

    fn i_am_done(&mut self, state: GameState) -> usize {
        self.screen = IAmDone(state);
        2
    }

    fn handle_i_am_done(&mut self, state: GameState, action: Action) -> usize {
        match action {
            Action::_0 => self.scene_router(state),
            Action::_1 => self.game_end(state),
            _ => invalid_action!(0, 1),
        }
    }

    fn game_end(&mut self, state: GameState) -> usize {
        self.screen = GameEnd(state);
        // "Нажми любую клавишу ..."
        1
    }

    fn wanna_try_again(&mut self) -> usize {
        self.screen = WannaTryAgain;
        // Хочешь попробовать снова? Да или нет.
        2
    }

    fn handle_wanna_try_again(&mut self, action: Action) -> usize {
        match action {
            Action::_0 => self.start_game(),
            Action::_1 => {
                self.screen = Disclaimer;
                // "Нажми любую клавишу ..."
                1
            }
            _ => invalid_action!(0, 1),
        }
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
