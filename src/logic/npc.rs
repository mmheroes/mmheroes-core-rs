use crate::logic::{Day, HealthLevel, Location, Subject, Time};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Classmate {
    Kolya,
    Pasha,
    Diamond,
    RAI,
    Misha,
    Serj,
    Sasha,
    NiL,
    Kuzmenko,
    DJuG,
    Andrew,
    Grisha,
}

impl Classmate {
    pub(in crate::logic) fn health_penalty(self) -> HealthLevel {
        match self {
            Kolya | Pasha | Diamond | Kuzmenko | DJuG | Andrew | Grisha | Misha
            | Serj | Sasha => HealthLevel(0),
            RAI | NiL => HealthLevel(8),
        }
    }

    pub(in crate::logic) fn annoyance(self) -> i16 {
        match self {
            Kolya | Pasha | Diamond | Serj | Sasha | Kuzmenko | DJuG | Andrew
            | Grisha => 0,
            RAI => 4,
            Misha => 2,
            NiL => 6,
        }
    }
}

use Classmate::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ClassmateLocation {
    Nowhere,
    Exam(Subject),
    Location(Location),
}

#[derive(Debug, Clone)]
pub struct ClassmateInfo {
    classmate: Classmate,
    current_location: ClassmateLocation,
}

impl ClassmateInfo {
    pub fn classmate(&self) -> Classmate {
        self.classmate
    }

    pub fn current_location(&self) -> ClassmateLocation {
        self.current_location
    }

    pub(in crate::logic) fn update(
        &mut self,
        rng: &mut crate::random::Rng,
        current_location: Location,
        today: &Day,
        time: Time,
    ) {
        match self.classmate {
            Kolya => {
                self.current_location = if time.is_between_9_and_19() {
                    ClassmateLocation::Location(Location::Mausoleum)
                } else {
                    ClassmateLocation::Nowhere
                }
            }
            Pasha => {
                self.current_location = if time.is_between_9_and_19() {
                    ClassmateLocation::Location(Location::PUNK)
                } else {
                    ClassmateLocation::Nowhere
                };

                let subjects = [
                    Subject::AlgebraAndNumberTheory,
                    Subject::Calculus,
                    Subject::GeometryAndTopology,
                ];

                let mut at_least_one_exam_is_today = false;
                let mut pasha_is_present_at_some_exam = false;

                loop {
                    for subject in subjects.iter().cloned() {
                        if current_location.is_exam_here_on_day(subject, today) {
                            at_least_one_exam_is_today = true;
                            if rng.random_number_with_upper_bound(10) > 5 {
                                pasha_is_present_at_some_exam = true;
                                self.current_location = ClassmateLocation::Exam(subject)
                            }
                        }
                    }

                    if pasha_is_present_at_some_exam || !at_least_one_exam_is_today {
                        break;
                    }
                }
            }
            Diamond => { /* TODO */ }
            RAI => { /* TODO */ }
            Misha => { /* TODO */ }
            Serj => { /* TODO */ }
            Sasha => { /* TODO */ }
            NiL => { /* TODO */ }
            Kuzmenko => { /* TODO */ }
            DJuG => { /* TODO */ }
            Andrew => { /* TODO */ }
            Grisha => { /* TODO */ }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Classmates([ClassmateInfo; 12]);

impl Classmates {
    pub(in crate::logic) fn new() -> Classmates {
        Classmates([
            ClassmateInfo {
                classmate: Kolya,
                current_location: ClassmateLocation::Nowhere,
            },
            ClassmateInfo {
                classmate: Pasha,
                current_location: ClassmateLocation::Nowhere,
            },
            ClassmateInfo {
                classmate: Diamond,
                current_location: ClassmateLocation::Nowhere,
            },
            ClassmateInfo {
                classmate: RAI,
                current_location: ClassmateLocation::Nowhere,
            },
            ClassmateInfo {
                classmate: Misha,
                current_location: ClassmateLocation::Nowhere,
            },
            ClassmateInfo {
                classmate: Serj,
                current_location: ClassmateLocation::Nowhere,
            },
            ClassmateInfo {
                classmate: Sasha,
                current_location: ClassmateLocation::Nowhere,
            },
            ClassmateInfo {
                classmate: NiL,
                current_location: ClassmateLocation::Nowhere,
            },
            ClassmateInfo {
                classmate: Kuzmenko,
                current_location: ClassmateLocation::Nowhere,
            },
            ClassmateInfo {
                classmate: DJuG,
                current_location: ClassmateLocation::Nowhere,
            },
            ClassmateInfo {
                classmate: Andrew,
                current_location: ClassmateLocation::Nowhere,
            },
            ClassmateInfo {
                classmate: Grisha,
                current_location: ClassmateLocation::Nowhere,
            },
        ])
    }

    pub fn filter_by_location(
        &self,
        location: Location,
    ) -> impl Iterator<Item = &ClassmateInfo> {
        self.iter().filter(move |&classmate| {
            matches!(classmate.current_location, ClassmateLocation::Location(l) if l == location)
        })
    }

    pub fn filter_by_exam(
        &self,
        subject: Subject,
    ) -> impl Iterator<Item = &ClassmateInfo> {
        self.iter().filter(move |&classmate| {
            matches!(classmate.current_location, ClassmateLocation::Exam(s) if s == subject)
        })
    }
}

impl core::ops::Index<Classmate> for Classmates {
    type Output = ClassmateInfo;

    fn index(&self, index: Classmate) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl core::ops::IndexMut<Classmate> for Classmates {
    fn index_mut(&mut self, index: Classmate) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl core::ops::Deref for Classmates {
    type Target = [ClassmateInfo];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for Classmates {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum KolyaInteraction {
    /// "Коля решил тебе ещё 2 задачи по алгебре!"
    /// (не пришлось заказывать настойку овса)
    SolvedAlgebraProblemsForFree,

    /// "Заказать Коле настойку овса?"
    /// (да или нет)
    PromptOatTincture,

    /// "Коля решил тебе ещё 2 задачи по алгебре!"
    /// (пришлось заказать настойку овса для этого)
    SolvedAlgebraProblemsForOatTincture,

    /// "Коля достает тормозную жидкость, и вы распиваете еще по стакану."
    /// (так как нет денег на настойку овса)
    BrakeFluidNoMoney,

    /// "Коля достает тормозную жидкость, и вы распиваете еще по стакану."
    /// (отказался заказывать настойку овса)
    BrakeFluidBecauseRefused,

    /// "Твой альтруизм навсегда останется в памяти потомков."
    /// (заказал Коле настойку овса, но решать задачи он не стал)
    Altruism,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PashaInteraction {
    /// "Паша вручает тебе твою стипуху за май: 50 руб."
    Stipend,

    /// "Паша воодушевляет тебя на великие дела."
    Inspiration,
}
