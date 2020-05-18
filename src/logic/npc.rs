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
                            if rng.random(10) > 5 {
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
            RAI => {
                self.current_location = if current_location.is_exam_here_now(
                    Subject::AlgebraAndNumberTheory,
                    today,
                    time,
                ) {
                    ClassmateLocation::Exam(Subject::AlgebraAndNumberTheory)
                } else if current_location.is_exam_here_now(
                    Subject::Calculus,
                    today,
                    time,
                ) {
                    ClassmateLocation::Exam(Subject::Calculus)
                } else if time.is_between_9_and_19() {
                    ClassmateLocation::Location(Location::ComputerClass)
                } else {
                    ClassmateLocation::Nowhere
                }
            }
            Misha => { /* TODO */ }
            Serj => { /* TODO */ }
            Sasha => {
                self.current_location = if time.is_between_9_and_19() && rng.roll_dice(4)
                {
                    ClassmateLocation::Location(Location::PUNK)
                } else {
                    ClassmateLocation::Nowhere
                }
            }
            NiL => { /* TODO */ }
            Kuzmenko => {
                self.current_location = if time.is_between_9_and_19() && rng.roll_dice(4)
                {
                    ClassmateLocation::Location(Location::ComputerClass)
                } else {
                    ClassmateLocation::Nowhere
                }
            }
            DJuG => { /* TODO */ }
            Andrew => { /* TODO */ }
            Grisha => {
                self.current_location = if rng.roll_dice(3) {
                    ClassmateLocation::Location(Location::Mausoleum)
                } else {
                    ClassmateLocation::Nowhere
                }
            }
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GrishaInteraction {
    /// "А ты не хочешь устроиться в ТЕРКОМ? Может, кое-чего подзаработаешь..."
    /// (да или нет)
    PromptEmploymentAtTerkom,

    /// "Поздравляю, теперь ты можешь идти в "контору"!"
    CongratulationsYouAreNowEmployed,

    /// "Как хочешь. Только смотри, не заучись там ..."
    AsYouWantButDontOverstudy,

    /// "Кстати, я тут знаю один качественно работающий прокси-сервер..."
    ProxyAddress,

    /// "Хочу халявы!"
    WantFreebie { drink_beer: bool, hour_pass: bool },

    /// "Прийди же, о халява!"
    FreebieComeToMe { drink_beer: bool, hour_pass: bool },

    /// "Халява есть - ее не может не быть."
    FreebieExists { drink_beer: bool, hour_pass: bool },

    /// "Давай организуем клуб любетелей халявы!"
    LetsOrganizeFreebieLoversClub { drink_beer: bool, hour_pass: bool },

    /// "Чтобы получить диплом, учиться совершенно необязательно!"
    NoNeedToStudyToGetDiploma { drink_beer: bool, hour_pass: bool },

    /// "Ну вот, ты готовился... Помогло это тебе?"
    YouStudiedDidItHelp { drink_beer: bool, hour_pass: bool },

    /// "На третьем курсе на лекции уже никто не ходит. Почти никто."
    ThirdYearStudentsDontAttendLectures { drink_beer: bool, hour_pass: bool },

    /// "Вот, бери пример с Коли."
    TakeExampleFromKolya { drink_beer: bool, hour_pass: bool },

    /// "Ненавижу Льва Толстого! Вчера "Войну и мир" <йк> ксерил..."
    HateLevTolstoy { drink_beer: bool, hour_pass: bool },

    /// "А в ПОМИ лучше вообще не ездить!"
    DontGoToPDMI { drink_beer: bool, hour_pass: bool },

    /// "Имена главных халявчиков и алкоголиков висят на баобабе."
    NamesOfFreebieLovers { drink_beer: bool, hour_pass: bool },

    /// "Правильно, лучше посидим здесь и оттянемся!"
    LetsHaveABreakHere { drink_beer: bool, hour_pass: bool },

    /// "Конспектировать ничего не надо. В мире есть ксероксы!"
    NoNeedToTakeLectureNotes { drink_beer: bool, hour_pass: bool },

    /// "А с четвертого курса вылететь уже почти невозможно."
    CantBeExpelledInFourthYear { drink_beer: bool, hour_pass: bool },

    /// "Вот у механиков - у них халява!"
    MechanicsHaveFreebie { drink_beer: bool, hour_pass: bool },
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum KuzmenkoInteraction {
    /// "Вы знаете, Климова можно найти в компьютерном классе 24-го мая с 10 по 11ч.."
    AdditionalComputerScienceExam { day_index: usize },

    /// "... отформатировать дискету так, чтобы 1ый сектор был 5ым ..."
    FormatFloppy,

    /// "А Вы нигде не видели литературы по фильтрам в Windows?"
    FiltersInWindows,

    /// "... написать визуализацию байта на ассемблере за 11 байт ..."
    ByteVisualization,

    /// "У вас Олег Плисс ведет какие-нибудь занятия?"
    OlegPliss,

    /// "Bill Gates = must die = кабысдох (рус.)."
    BillGatesMustDie,

    /// "Вы читали журнал "Монитор"? Хотя вряд ли..."
    MonitorJournal,

    /// "Я слышал, что mmHeroes написана на BP 7.0."
    MmheroesBP7,

    /// "Записывайтесь на мой семинар по языку Си!"
    CSeminar,

    /// "На третьем курсе я буду вести у вас спецвычпрактикум."
    ThirdYear,

    /// "Интересно, когда они снова наладят STAR?"
    STAR,

    /// "Получите себе ящик rambler'e или на mail.ru !"
    GetYourselvesAnEmail,

    /// "А разве Терехов-старший ничего не рассказывает про IBM PC?"
    TerekhovSenior,
}
