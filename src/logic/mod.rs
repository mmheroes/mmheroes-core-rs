pub mod timetable;
pub use timetable::*;

pub mod characteristics;
pub use characteristics::*;

use crate::random;

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
    current_day_index: usize,
    current_time: Time,
    timetable: timetable::Timetable,
    location: Location,
}

impl GameState {
    fn new(
        player: Player,
        timetable: timetable::Timetable,
        location: Location,
    ) -> GameState {
        GameState {
            player,
            current_day_index: 0,
            current_time: Time(8),
            timetable,
            location,
        }
    }

    pub fn current_day(&self) -> &Day {
        &self.timetable.days()[self.current_day_index]
    }

    pub fn current_time(&self) -> Time {
        self.current_time
    }

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

#[derive(Debug, Clone)]
pub struct SubjectStatus {
    subject: Subject,
    knowledge: BrainLevel,
    passed_exam_day_index: Option<usize>,
    problems_done: u8,
}

impl SubjectStatus {
    pub fn knowledge(&self) -> BrainLevel {
        self.knowledge
    }

    pub fn subject(&self) -> Subject {
        self.subject
    }

    pub fn problems_done(&self) -> u8 {
        self.problems_done
    }

    pub fn passed(&self) -> bool {
        self.passed_exam_day_index.is_some()
    }

    pub fn passed_exam_day<'a>(
        &self,
        timetable: &'a timetable::Timetable,
    ) -> Option<&'a Day> {
        self.passed_exam_day_index.map(|i| &timetable.days()[i])
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    subjects: [SubjectStatus; NUM_SUBJECTS],
    god_mode: bool,

    /// Запах чеснока изо рта
    garlic: i16,

    /// Получил ли персонаж дискету с новой версией MMHEROES от Diamond
    has_mmheroes_floppy: bool,
    has_internet: bool,
    is_invited: bool,
    inception: bool,
    employed_at_terkom: bool,
    got_stipend: bool,
    has_ticket: bool,
    knows_djug: bool,

    health: HealthLevel,
    money: Money,
    brain: BrainLevel,
    stamina: StaminaLevel,
    charisma: CharismaLevel,
}

impl Player {
    fn new(
        god_mode: bool,
        health: HealthLevel,
        brain: BrainLevel,
        stamina: StaminaLevel,
        charisma: CharismaLevel,
        mut knowledge: impl FnMut(Subject) -> BrainLevel,
    ) -> Player {
        let player = Player {
            subjects: [
                SubjectStatus {
                    subject: Subject::AlgebraAndNumberTheory,
                    knowledge: knowledge(Subject::AlgebraAndNumberTheory),
                    passed_exam_day_index: None,
                    problems_done: 0,
                },
                SubjectStatus {
                    subject: Subject::Calculus,
                    knowledge: knowledge(Subject::Calculus),
                    passed_exam_day_index: None,
                    problems_done: 0,
                },
                SubjectStatus {
                    subject: Subject::GeometryAndTopology,
                    knowledge: knowledge(Subject::GeometryAndTopology),
                    passed_exam_day_index: None,
                    problems_done: 0,
                },
                SubjectStatus {
                    subject: Subject::ComputerScience,
                    knowledge: knowledge(Subject::ComputerScience),
                    passed_exam_day_index: None,
                    problems_done: 0,
                },
                SubjectStatus {
                    subject: Subject::English,
                    knowledge: knowledge(Subject::English),
                    passed_exam_day_index: None,
                    problems_done: 0,
                },
                SubjectStatus {
                    subject: Subject::PhysicalEducation,
                    knowledge: knowledge(Subject::PhysicalEducation),
                    passed_exam_day_index: None,
                    problems_done: 0,
                },
            ],
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
            health,
            money: Money(0),
            brain,
            stamina,
            charisma,
        };

        for subject in player.subjects.iter() {
            assert!(subject.knowledge < player.brain);
        }

        player
    }

    pub fn status_for_subject(&self, subject: Subject) -> &SubjectStatus {
        &self.subjects[subject as usize]
    }

    pub fn exams_left(&self) -> usize {
        self.subjects
            .iter()
            .filter(|s| s.passed_exam_day_index.is_none())
            .count()
    }

    pub fn health(&self) -> HealthLevel {
        self.health
    }

    pub fn money(&self) -> Money {
        self.money
    }

    pub fn got_stipend(&self) -> bool {
        self.got_stipend
    }

    pub fn brain(&self) -> BrainLevel {
        self.brain
    }

    pub fn stamina(&self) -> StaminaLevel {
        self.stamina
    }

    pub fn charisma(&self) -> CharismaLevel {
        self.charisma
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
    member0xFC: i16, // Минимальный уровень познания?
    member0x100: i16,

    /// Какой уровень знаний соответствует какой оценке по шкале этого препода.
    assessment_bounds: [(BrainLevel, KnowledgeAssessment); 3],
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
        use KnowledgeAssessment::*;
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
                    assessment_bounds: [
                        (BrainLevel(11), Bad),
                        (BrainLevel(21), Satisfactory),
                        (BrainLevel(51), Good),
                    ],
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
                    assessment_bounds: [
                        (BrainLevel(9), Bad),
                        (BrainLevel(19), Satisfactory),
                        (BrainLevel(41), Good),
                    ],
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
                    assessment_bounds: [
                        (BrainLevel(6), Bad),
                        (BrainLevel(11), Satisfactory),
                        (BrainLevel(31), Good),
                    ],
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
                    assessment_bounds: [
                        (BrainLevel(10), Bad),
                        (BrainLevel(16), Satisfactory),
                        (BrainLevel(31), Good),
                    ],
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
                    assessment_bounds: [
                        (BrainLevel(5), Bad),
                        (BrainLevel(9), Satisfactory),
                        (BrainLevel(16), Good),
                    ],
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
                    assessment_bounds: [
                        (BrainLevel(5), Bad),
                        (BrainLevel(9), Satisfactory),
                        (BrainLevel(16), Good),
                    ],
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
                Action::_7 => self.i_am_done(state),
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
