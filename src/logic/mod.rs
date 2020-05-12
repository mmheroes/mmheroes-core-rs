pub mod timetable;
pub use timetable::*;

pub mod characteristics;
pub use characteristics::*;

pub mod game_state;
pub use game_state::*;

pub mod subjects;
pub use subjects::*;

pub mod npc;
pub use npc::*;

use npc::Classmate::*;

use crate::random;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Action {
    AnyKey,
    Yes,
    No,
    InteractWithClassmate(Classmate),
    InteractWithProfessor(Subject),
    RandomStudent,
    CleverStudent,
    ImpudentStudent,
    SociableStudent,
    GodMode,
    Study,
    ViewTimetable,
    Rest,
    GoToBed,
    GoToDorm,
    GoToPUNK,
    GoToComputerClass,
    GoToPDMI,
    GoToMausoleum,
    GoToCafe,
    SurfInternet,
    PlayMMHEROES,
    GoToProfessor,
    GoToWork,
    LookAtBestScores,
    IAmDone,
    WhatToDo,
    AboutScreen,
    WhereToGoAndWhy,
    AboutProfessors,
    AboutCharacters,
    AboutThisProgram,
    GoBack,
}

macro_rules! illegal_action {
    ($action:expr) => {
        panic!("Illegal action: {:?}", $action)
    };
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CauseOfDeath {
    /// Умер по пути на факультет.
    OnTheWayToPUNK,

    /// Умер по пути в мавзолей.
    OnTheWayToMausoleum,

    /// Умер по пути домой. Бывает.
    OnTheWayToDorm,

    /// Упал с лестницы у главного входа.
    FellFromStairs,

    /// Сгорел на работе.
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

    /// Взаимодействие с Колей.
    KolyaInteraction(GameState, npc::KolyaInteraction),

    /// Взаимодействие с Пашей.
    PashaInteraction(GameState, npc::PashaInteraction),

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

    /// Терминальное состояние. Ему тоже соответствует никакой экран.
    /// Игра завершена безвозвратно.
    Terminal,
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
            screen: Intro,
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

    /// Возвращает текущее состояние игры, если оно доступно.
    /// Оно может быть недоступно, например, если игра ещё не началась
    /// или уже закончилась.
    pub fn game_state(&self) -> Option<&GameState> {
        match &self.screen {
            Timetable(state)
            | SceneRouter(state)
            | IAmDone(state)
            | GameEnd(state)
            | WhatToDo(state)
            | AboutScreen(state)
            | WhereToGoAndWhy(state)
            | AboutProfessors(state)
            | AboutCharacters(state)
            | AboutThisProgram(state)
            | KolyaInteraction(state, _)
            | PashaInteraction(state, _) => Some(state),
            Intro | InitialParameters | Ding(_) | WannaTryAgain | Disclaimer
            | Terminal => None,
        }
    }

    pub fn available_actions(&self) -> usize {
        self.available_actions
    }

    pub fn perform_action(&mut self, action: Action) {
        self.available_actions = self._perform_action(action)
    }

    /// Accepts an action, returns the number of actions available in the updated state.
    fn _perform_action(&mut self, action: Action) -> usize {
        match &self.screen {
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
            KolyaInteraction(state, interaction) => {
                let state = state.clone();
                let interaction = *interaction;
                self.proceed_with_kolya(state, action, interaction)
            }
            PashaInteraction(state, interaction) => {
                let state = state.clone();
                let interaction = *interaction;
                self.proceed_with_pasha(state, action, interaction)
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
            GameMode::Normal => self.ding(Action::RandomStudent),
        }
    }

    fn ding(&mut self, action: Action) -> usize {
        self.screen = Ding(self.initialize_player(action));
        // "Нажми любую клавишу ..."
        1
    }

    fn initialize_player(&mut self, parameters: Action) -> Player {
        let (god_mode, brain, stamina, charisma) = match parameters {
            Action::RandomStudent => (
                false,
                BrainLevel(self.rng.random_number_in_range(4..7)),
                StaminaLevel(self.rng.random_number_in_range(4..7)),
                CharismaLevel(self.rng.random_number_in_range(4..7)),
            ),
            Action::CleverStudent => (
                false,
                BrainLevel(self.rng.random_number_in_range(5..10)),
                StaminaLevel(self.rng.random_number_in_range(2..5)),
                CharismaLevel(self.rng.random_number_in_range(2..5)),
            ),
            Action::ImpudentStudent => (
                false,
                BrainLevel(self.rng.random_number_in_range(2..5)),
                StaminaLevel(self.rng.random_number_in_range(5..10)),
                CharismaLevel(self.rng.random_number_in_range(2..5)),
            ),
            Action::SociableStudent => (
                false,
                BrainLevel(self.rng.random_number_in_range(2..5)),
                StaminaLevel(self.rng.random_number_in_range(2..5)),
                CharismaLevel(self.rng.random_number_in_range(5..10)),
            ),
            Action::GodMode => {
                (true, BrainLevel(30), StaminaLevel(30), CharismaLevel(30))
            }
            _ => illegal_action!(parameters),
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
        let state = self.game_state().unwrap();
        match location {
            Location::PDMI => todo!(),
            Location::PUNK => {
                // - Идти к преподу
                // - Посмотреть на баобаб
                // - Пойти в общагу
                // - Поехать в ПОМИ
                // - Пойти в мавзолей
                // - (Пойти в компьютерный класс)?
                // - (Сходить в кафе)?
                // - (NPC)*
                // - (Пойти в ТЕРКОМ, поработать)?
                // - С меня хватит!
                6 + (state.current_time.is_computer_class_open() as usize)
                    + (state.current_time.is_cafe_open() as usize)
                    + state.classmates.filter_by_location(location).count()
                    + (state.player.is_employed_at_terkom() as usize)
            }
            Location::Mausoleum => {
                // - Идти в ПУНК
                // - Поехать в ПОМИ
                // - Идти в общагу
                // - Отдыхать
                // - (NPC)*
                // - С меня хватит!
                5 + state.classmates.filter_by_location(location).count()
            }
            Location::Dorm => {
                // - Готовиться
                // - Посмотреть расписание
                // - Отдыхать
                // - Лечь спать
                // - Пойти на факультет
                // - Поехать в ПОМИ
                // - Пойти в мавзолей
                // - С меня хватит!
                // - ЧТО ДЕЛАТЬ ???
                9
            }
            Location::ComputerClass => todo!(),
        }
    }

    fn handle_scene_router_action(&mut self, state: GameState, action: Action) -> usize {
        match state.location() {
            Location::PUNK => self.handle_punk_action(state, action),
            Location::PDMI => todo!(),
            Location::ComputerClass => todo!(),
            Location::Dorm => self.handle_dorm_action(state, action),
            Location::Mausoleum => self.handle_mausoleum_action(state, action),
        }
    }

    fn handle_dorm_action(&mut self, mut state: GameState, action: Action) -> usize {
        assert!(state.location == Location::Dorm);
        match action {
            Action::AnyKey => {
                assert!(state.failed_attempt_to_sleep);
                state.failed_attempt_to_sleep = false;
                self.scene_router(state)
            }
            Action::Study => todo!("Study"),
            Action::ViewTimetable => self.view_timetable(state),
            Action::Rest => self.rest_in_dorm(state),
            Action::GoToBed => self.try_to_sleep(state),
            Action::GoToPUNK => {
                state.location = Location::PUNK;
                self.decrease_health(
                    HealthLevel::location_change_large_penalty(),
                    state,
                    CauseOfDeath::OnTheWayToPUNK,
                    /*if_alive=*/ |game, state| game.scene_router(state),
                )
            }
            Action::GoToPDMI => todo!("Go to PDMI"),
            Action::GoToMausoleum => {
                state.location = Location::Mausoleum;
                self.decrease_health(
                    HealthLevel::location_change_large_penalty(),
                    state,
                    CauseOfDeath::OnTheWayToMausoleum,
                    /*if_alive=*/ |game, state| game.scene_router(state),
                )
            }
            Action::IAmDone => self.i_am_done(state),
            Action::WhatToDo => {
                self.screen = WhatToDo(state);
                HELP_SCREEN_OPTION_COUNT
            }
            _ => illegal_action!(action),
        }
    }

    fn interact_with_pasha(&mut self, state: GameState) -> usize {
        assert_eq!(state.location, Location::PUNK);
        let interaction = if state.player.got_stipend {
            npc::PashaInteraction::Inspiration
        } else {
            npc::PashaInteraction::Stipend
        };

        self.screen = GameScreen::PashaInteraction(state, interaction);
        // "Нажми любую клавишу ..."
        1
    }

    fn proceed_with_pasha(
        &mut self,
        mut state: GameState,
        action: Action,
        interaction: npc::PashaInteraction,
    ) -> usize {
        assert_eq!(action, Action::AnyKey);
        assert_eq!(state.location, Location::PUNK);
        assert!(matches!(self.screen, GameScreen::PashaInteraction(_, _)));
        match interaction {
            npc::PashaInteraction::Stipend => {
                state.player.got_stipend = true;
                state.player.money += Money::stipend();
            }
            npc::PashaInteraction::Inspiration => {
                state.player.stamina += 1;
                for (subject, _) in SUBJECTS.iter() {
                    let knowledge =
                        &mut state.player.status_for_subject_mut(*subject).knowledge;
                    if *knowledge > BrainLevel(3) {
                        *knowledge -= self.rng.random_number_with_upper_bound(3);
                    }
                }
            }
        }
        self.scene_router(state)
    }

    fn handle_punk_action(&mut self, mut state: GameState, action: Action) -> usize {
        assert_eq!(state.location, Location::PUNK);
        match action {
            Action::GoToProfessor => todo!(),
            Action::LookAtBestScores => todo!(),
            Action::GoToDorm => {
                state.location = Location::Dorm;
                self.scene_router(state)
            }
            Action::GoToPDMI => todo!(),
            Action::GoToMausoleum => {
                state.location = Location::Mausoleum;
                self.decrease_health(
                    HealthLevel::location_change_large_penalty(),
                    state,
                    CauseOfDeath::OnTheWayToMausoleum,
                    /*if_alive=*/ |game, state| game.scene_router(state),
                )
            }
            Action::GoToComputerClass => {
                assert!(state.current_time.is_computer_class_open());
                state.location = Location::ComputerClass;
                self.decrease_health(
                    HealthLevel::location_change_small_penalty(),
                    state,
                    CauseOfDeath::FellFromStairs,
                    /*if_alive=*/ |game, state| game.scene_router(state),
                )
            }
            Action::GoToCafe => {
                assert!(state.current_time.is_cafe_open());
                todo!()
            }
            Action::InteractWithClassmate(Pasha) => {
                assert!(matches!(
                    state.classmates[Pasha].current_location(),
                    ClassmateLocation::Location(Location::PUNK)
                ));
                self.interact_with_pasha(state)
            }
            Action::InteractWithClassmate(Misha) => {
                assert!(matches!(
                    state.classmates[Misha].current_location(),
                    ClassmateLocation::Location(Location::PUNK)
                ));
                todo!()
            }
            Action::InteractWithClassmate(Serj) => {
                assert!(matches!(
                    state.classmates[Serj].current_location(),
                    ClassmateLocation::Location(Location::PUNK)
                ));
                todo!()
            }
            Action::InteractWithClassmate(Sasha) => {
                assert!(matches!(
                    state.classmates[Sasha].current_location(),
                    ClassmateLocation::Location(Location::PUNK)
                ));
                todo!()
            }
            Action::GoToWork => {
                assert!(state.player.is_employed_at_terkom);
                todo!()
            }
            Action::IAmDone => self.i_am_done(state),
            _ => illegal_action!(action),
        }
    }

    /// Возвращает `Some`, если Коля помог решить задачи по алгебре, иначе — `None`.
    fn kolya_maybe_solve_algebra_problems(
        &mut self,
        player: &mut Player,
    ) -> Option<usize> {
        use Subject::AlgebraAndNumberTheory;
        let has_enough_charisma =
            player.charisma > self.rng.random_number_with_upper_bound(CharismaLevel(10));
        let algebra = player.status_for_subject(AlgebraAndNumberTheory);
        let problems_done = algebra.problems_done;
        let required_problems = SUBJECTS[AlgebraAndNumberTheory].1.required_problems;
        let has_at_least_2_remaining_problems = problems_done + 2 <= required_problems;
        if has_enough_charisma && has_at_least_2_remaining_problems {
            // "Нажми любую клавишу ..."
            Some(1)
        } else {
            None
        }
    }

    fn interact_with_kolya(&mut self, mut state: GameState) -> usize {
        assert_eq!(state.location, Location::Mausoleum);
        let player = &mut state.player;
        let (num_actions, interaction) =
            if let Some(num_actions) = self.kolya_maybe_solve_algebra_problems(player) {
                (
                    num_actions,
                    npc::KolyaInteraction::SolvedAlgebraProblemsForFree,
                )
            } else if player.money < Money::oat_tincture_cost() {
                // "Коля достает тормозную жидкость, и вы распиваете еще по стакану."
                // "Нажми любую клавишу ..."
                (1, npc::KolyaInteraction::BrakeFluidNoMoney)
            } else {
                // "Знаешь, пиво, конечно, хорошо, но настойка овса - лучше!"
                // "Заказать Коле настойку овса?"
                // "Да" или "Нет"
                (2, npc::KolyaInteraction::PromptOatTincture)
            };
        self.screen = GameScreen::KolyaInteraction(state, interaction);
        num_actions
    }

    fn proceed_with_kolya(
        &mut self,
        mut state: GameState,
        action: Action,
        interaction: npc::KolyaInteraction,
    ) -> usize {
        assert_eq!(state.location, Location::Mausoleum);
        assert!(matches!(self.screen, GameScreen::KolyaInteraction(_, _)));
        let player = &mut state.player;
        match action {
            Action::AnyKey => {
                let algebra_status =
                    player.status_for_subject_mut(Subject::AlgebraAndNumberTheory);
                match interaction {
                    npc::KolyaInteraction::SolvedAlgebraProblemsForFree => {
                        algebra_status.problems_done += 2;
                        return self.hour_pass(state);
                    }
                    npc::KolyaInteraction::PromptOatTincture => unreachable!(),
                    npc::KolyaInteraction::SolvedAlgebraProblemsForOatTincture => {
                        algebra_status.problems_done += 2;
                        player.money -= Money::oat_tincture_cost();
                        return self.hour_pass(state);
                    }
                    npc::KolyaInteraction::BrakeFluidNoMoney => {
                        player.brain -= 1;
                        if player.brain <= BrainLevel(0) {
                            player.health = HealthLevel(0);
                            player.cause_of_death = Some(CauseOfDeath::DrankTooMuch);
                            return self.game_end(state);
                        }
                    }
                    npc::KolyaInteraction::BrakeFluidBecauseRefused => {
                        state.player.brain -= 1;
                        // Забавно, что в этой ветке можно бесконечно пить тормозную
                        // жидкость и никогда не спиться. Баг в оригинальной реализации.
                    }
                    npc::KolyaInteraction::Altruism => {
                        player.money -= Money::oat_tincture_cost();
                    }
                }
                self.scene_router(state)
            }
            Action::Yes => {
                assert_eq!(interaction, npc::KolyaInteraction::PromptOatTincture);
                if let Some(num_actions) = self.kolya_maybe_solve_algebra_problems(player)
                {
                    // "Коля решил тебе ещё 2 задачи по алгебре!"
                    self.screen = GameScreen::KolyaInteraction(
                        state,
                        npc::KolyaInteraction::SolvedAlgebraProblemsForOatTincture,
                    );
                    num_actions
                } else {
                    // "Твой альтруизм навсегда останется в памяти потомков."
                    self.screen = GameScreen::KolyaInteraction(
                        state,
                        npc::KolyaInteraction::Altruism,
                    );
                    // "Нажми любую клавишу ..."
                    1
                }
            }
            Action::No => {
                assert_eq!(interaction, npc::KolyaInteraction::PromptOatTincture);
                // "Зря, ой, зря ..."
                // "Коля достает тормозную жидкость, и вы распиваете еще по стакану."
                self.screen = GameScreen::KolyaInteraction(
                    state,
                    npc::KolyaInteraction::BrakeFluidBecauseRefused,
                );
                // "Нажми любую клавишу ..."
                1
            }
            _ => illegal_action!(action),
        }
    }

    fn handle_mausoleum_action(&mut self, mut state: GameState, action: Action) -> usize {
        assert!(state.location == Location::Mausoleum);
        match action {
            Action::GoToPUNK => {
                state.location = Location::PUNK;
                self.decrease_health(
                    HealthLevel::location_change_large_penalty(),
                    state,
                    CauseOfDeath::OnTheWayToPUNK,
                    /*if_alive=*/ |game, state| game.scene_router(state),
                )
            }
            Action::GoToPDMI => todo!(),
            Action::GoToDorm => {
                state.location = Location::Dorm;
                self.scene_router(state)
            }
            Action::Rest => todo!(),
            Action::InteractWithClassmate(Kolya) => {
                assert!(matches!(
                    state.classmates[Kolya].current_location(),
                    ClassmateLocation::Location(Location::Mausoleum)
                ));
                self.interact_with_kolya(state)
            }
            Action::InteractWithClassmate(Grisha) => {
                assert!(matches!(
                    state.classmates[Grisha].current_location(),
                    ClassmateLocation::Location(Location::Mausoleum)
                ));
                todo!()
            }
            Action::InteractWithClassmate(Serj) => {
                assert!(matches!(
                    state.classmates[Serj].current_location(),
                    ClassmateLocation::Location(Location::Mausoleum)
                ));
                todo!()
            }
            _ => illegal_action!(action),
        }
    }

    fn view_timetable(&mut self, state: GameState) -> usize {
        self.screen = Timetable(state);
        // "Нажми любую клавишу ..."
        1
    }

    fn decrease_health<F: FnOnce(&mut Game, GameState) -> usize>(
        &mut self,
        delta: HealthLevel,
        mut state: GameState,
        cause_of_death: CauseOfDeath,
        if_alive: F,
    ) -> usize {
        if state.player.health <= delta {
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

    fn run_classmate_routines(&mut self, state: &mut GameState) {
        let timetable = &state.timetable;
        let day = &timetable.days()[state.current_day_index];
        let time = state.current_time;
        let classmates = &mut state.classmates;
        for classmate in classmates.iter_mut() {
            classmate.update(&mut self.rng, state.location, day, time);
        }
    }

    fn hour_pass(&mut self, mut state: GameState) -> usize {
        // TODO: Lot of stuff going on here

        // TODO: Поменять эти строки местами и не забыть отредактировать метод
        // Time::is_between_9_and_19()!
        self.run_classmate_routines(&mut state);
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
            Action::No => self.scene_router(state),
            Action::Yes => self.game_end(state),
            _ => illegal_action!(action),
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
            Action::Yes => self.start_game(),
            Action::No => {
                self.screen = Disclaimer;
                // "Нажми любую клавишу ..."
                1
            }
            _ => illegal_action!(action),
        }
    }

    fn handle_what_to_do(&mut self, state: GameState, action: Action) -> usize {
        assert_eq!(state.location(), Location::Dorm);
        match action {
            Action::WhatToDo => self.screen = WhatToDo(state),
            Action::AboutScreen => self.screen = AboutScreen(state),
            Action::WhereToGoAndWhy => self.screen = WhereToGoAndWhy(state),
            Action::AboutProfessors => self.screen = AboutProfessors(state),
            Action::AboutCharacters => self.screen = AboutCharacters(state),
            Action::AboutThisProgram => self.screen = AboutThisProgram(state),
            Action::GoBack => {
                return self.scene_router(state);
            }
            _ => illegal_action!(action),
        };
        HELP_SCREEN_OPTION_COUNT
    }
}
