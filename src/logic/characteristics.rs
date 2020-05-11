use super::*;
use core::convert::TryFrom;

macro_rules! define_characteristic {
    ($name:ident) => {
        #[repr(transparent)]
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug)]
        pub struct $name(pub(in crate::logic) i16);

        impl core::ops::AddAssign for $name {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0
            }
        }

        impl core::ops::AddAssign<i16> for $name {
            fn add_assign(&mut self, rhs: i16) {
                self.0 += rhs
            }
        }

        impl core::ops::SubAssign for $name {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0
            }
        }

        impl core::ops::SubAssign<i16> for $name {
            fn sub_assign(&mut self, rhs: i16) {
                self.0 -= rhs
            }
        }

        impl TryFrom<u64> for $name {
            type Error = core::num::TryFromIntError;

            fn try_from(value: u64) -> Result<Self, Self::Error> {
                i16::try_from(value).map($name)
            }
        }

        impl TryFrom<$name> for u64 {
            type Error = core::num::TryFromIntError;

            fn try_from(value: $name) -> Result<Self, Self::Error> {
                u64::try_from(value.0)
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Display::fmt(&self.0, f)
            }
        }
    };
}

define_characteristic!(HealthLevel);
define_characteristic!(Money);
define_characteristic!(BrainLevel);
define_characteristic!(StaminaLevel);
define_characteristic!(CharismaLevel);

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum HealthAssessment {
    /// "живой труп"
    LivingDead,

    /// "пора помирать ..."
    TimeToDie,

    /// "плохое"
    Bad,

    /// "так себе"
    SoSo,

    /// "среднее"
    Average,

    /// "хорошее"
    Good,

    /// "отличное"
    Great,
}

impl HealthLevel {
    pub fn assessment(&self) -> HealthAssessment {
        use HealthAssessment::*;
        let scale = [
            (HealthLevel(1), LivingDead),
            (HealthLevel(9), TimeToDie),
            (HealthLevel(17), Bad),
            (HealthLevel(25), SoSo),
            (HealthLevel(33), Average),
            (HealthLevel(41), Good),
        ];
        *crate::util::assess(&scale, self, &Great)
    }
}

impl Money {
    pub const fn zero() -> Money {
        Money(0)
    }

    /// Стоимость настойки овса для Коли
    pub const fn oat_tincture_cost() -> Money {
        Money(15)
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum KnowledgeAssessment {
    Bad,
    Satisfactory,
    Good,
    VeryGood,
    Excellent,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum BrainAssessment {
    /// "Клиническая смерть мозга"
    ClinicalBrainDeath,

    /// "Голова просто никакая"
    BrainIsAlmostNonFunctioning,

    /// "Думать практически невозможно"
    ThinkingIsAlmostImpossible,

    /// "Думать трудно"
    ThinkingIsDifficult,

    /// "Голова почти в норме"
    BrainIsAlmostOK,

    /// "Голова в норме"
    BrainIsOK,

    /// "Голова свежая"
    BrainIsFresh,

    /// "Легкость в мыслях необыкновенная"
    ExtraordinaryEaseOfThought,

    /// "Обратитесь к разработчику ;)"
    ContactTheDeveloper, // TODO: Find out if this is ever reachable :)
}

impl BrainLevel {
    pub fn assessment(&self) -> BrainAssessment {
        use BrainAssessment::*;
        let scale = [
            (BrainLevel(0), ClinicalBrainDeath),
            (BrainLevel(1), BrainIsAlmostNonFunctioning),
            (BrainLevel(2), ThinkingIsAlmostImpossible),
            (BrainLevel(3), ThinkingIsDifficult),
            (BrainLevel(4), BrainIsAlmostOK),
            (BrainLevel(5), BrainIsOK),
            (BrainLevel(6), BrainIsFresh),
            (BrainLevel(101), ExtraordinaryEaseOfThought),
        ];
        *crate::util::assess(&scale, self, &ContactTheDeveloper)
    }

    pub fn absolute_knowledge_assessment(&self) -> KnowledgeAssessment {
        use KnowledgeAssessment::*;
        let scale = [
            (BrainLevel(6), Bad),
            (BrainLevel(13), Satisfactory),
            (BrainLevel(21), Good),
            (BrainLevel(31), VeryGood),
        ];
        *crate::util::assess(&scale, self, &Excellent)
    }

    pub fn relative_knowledge_assessment(&self, subject: Subject) -> KnowledgeAssessment {
        *crate::util::assess(
            &SUBJECTS[subject].1.assessment_bounds,
            self,
            &KnowledgeAssessment::Excellent,
        )
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum StaminaAssessment {
    /// "Мама, роди меня обратно!"
    MamaTakeMeBack,

    /// "Окончательно заучился"
    CompletelyOverstudied,

    /// "Я так больше немогууу!"
    ICantTakeIt,

    /// "Скорее бы все это кончилось..."
    IWishItAllEndedSoon,

    /// "Еще немного и пора отдыхать"
    ALittleMoreAndThenRest,

    /// "Немного устал"
    ABitTired,

    /// "Готов к труду и обороне"
    ReadyForEverything,

    /// "Нас ждут великие дела"
    GreatThingsAwaitUs,
}

impl StaminaLevel {
    pub fn assessment(&self) -> StaminaAssessment {
        use StaminaAssessment::*;
        let scale = [
            (StaminaLevel(0), MamaTakeMeBack),
            (StaminaLevel(1), CompletelyOverstudied),
            (StaminaLevel(2), ICantTakeIt),
            (StaminaLevel(3), IWishItAllEndedSoon),
            (StaminaLevel(4), ALittleMoreAndThenRest),
            (StaminaLevel(5), ABitTired),
            (StaminaLevel(6), ReadyForEverything),
        ];
        *crate::util::assess(&scale, self, &GreatThingsAwaitUs)
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum CharismaAssessment {
    /// "Очень замкнутый товарищ"
    VeryIntroverted,

    /// "Предпочитаешь одиночество"
    PreferSolitariness,

    /// "Тебе трудно общаться с людьми"
    VeryHardToTalkToPeople,

    /// "Тебе непросто общаться с людьми"
    NotEasyToTalkToPeople,

    /// "Ты нормально относишься к окружающим"
    Normal,

    /// "У тебя много друзей"
    ManyFriends,

    /// "У тебя очень много друзей"
    TonsOfFriends,
}

impl CharismaLevel {
    pub fn assessment(&self) -> CharismaAssessment {
        use CharismaAssessment::*;
        let scale = [
            (CharismaLevel(1), VeryIntroverted),
            (CharismaLevel(2), PreferSolitariness),
            (CharismaLevel(3), VeryHardToTalkToPeople),
            (CharismaLevel(4), NotEasyToTalkToPeople),
            (CharismaLevel(5), Normal),
            (CharismaLevel(6), ManyFriends),
        ];
        *crate::util::assess(&scale, self, &TonsOfFriends)
    }
}
