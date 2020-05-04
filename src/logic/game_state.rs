use super::*;

#[derive(Clone, Debug)]
pub struct GameState {
    pub(in crate::logic) player: Player,
    pub(in crate::logic) current_day_index: usize,
    pub(in crate::logic) current_time: Time,
    pub(in crate::logic) failed_attempt_to_sleep: bool,
    pub(in crate::logic) timetable: timetable::Timetable,
    pub(in crate::logic) location: Location,
}

impl GameState {
    pub(in crate::logic) fn new(
        player: Player,
        timetable: timetable::Timetable,
        location: Location,
    ) -> GameState {
        GameState {
            player,
            current_day_index: 0,
            current_time: Time(8),
            failed_attempt_to_sleep: false,
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

    pub fn failed_attempt_to_sleep(&self) -> bool {
        self.failed_attempt_to_sleep
    }
}

#[derive(Debug, Clone)]
pub struct SubjectStatus {
    pub(in crate::logic) subject: Subject,
    pub(in crate::logic) knowledge: BrainLevel,
    pub(in crate::logic) passed_exam_day_index: Option<usize>,
    pub(in crate::logic) problems_done: u8,
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
    pub(in crate::logic) subjects: [SubjectStatus; NUM_SUBJECTS],
    pub(in crate::logic) god_mode: bool,

    /// Запах чеснока изо рта
    pub(in crate::logic) garlic: i16,

    /// Получил ли персонаж дискету с новой версией MMHEROES от Diamond
    pub(in crate::logic) has_mmheroes_floppy: bool,
    pub(in crate::logic) has_internet: bool,
    pub(in crate::logic) is_invited: bool,
    pub(in crate::logic) inception: bool,
    pub(in crate::logic) employed_at_terkom: bool,
    pub(in crate::logic) got_stipend: bool,
    pub(in crate::logic) has_ticket: bool,
    pub(in crate::logic) knows_djug: bool,

    pub(in crate::logic) health: HealthLevel,
    pub(in crate::logic) money: Money,
    pub(in crate::logic) brain: BrainLevel,
    pub(in crate::logic) stamina: StaminaLevel,
    pub(in crate::logic) charisma: CharismaLevel,

    pub(in crate::logic) cause_of_death: Option<CauseOfDeath>,
}

impl Player {
    pub(in crate::logic) fn new(
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
            cause_of_death: None,
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

    pub fn has_mmheroes_floppy(&self) -> bool {
        self.has_mmheroes_floppy
    }

    pub fn has_internet(&self) -> bool {
        self.has_internet
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

    pub fn cause_of_death(&self) -> Option<CauseOfDeath> {
        self.cause_of_death
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
