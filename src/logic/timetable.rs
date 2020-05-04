pub use crate::logic::{Location, Subject, NUM_SUBJECTS, SUBJECTS};

use core::convert::TryFrom;
use core::fmt::{Display, Formatter};
use core::ops::{Add, AddAssign, Sub};

pub const NUM_DAYS: usize = 6;

/// A number of hours passed since midnight. Has semantics of a timestamp.
/// Instances of this type cannot be added to each other, but you can add
/// an instance of `Duration` and get some new `Time`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Time(pub u8);

pub const WORKDAY_BEGINS: Time = Time(9);
pub const WORKDAY_ENDS: Time = Time(18);

impl Time {
    pub fn is_midnight(self) -> bool {
        self.0 == 0 || self.0 == 24
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        assert!(self.0 < 24);
        f.write_fmt(format_args!("{}", self.0))
    }
}

/// A number of hours.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Duration(pub i8);

impl Add<Duration> for Time {
    type Output = Time;

    fn add(self, rhs: Duration) -> Self::Output {
        if rhs.0 < 0 {
            Time(self.0 - (-rhs.0 as u8))
        } else {
            Time(self.0 + rhs.0 as u8)
        }
    }
}

impl AddAssign<Duration> for Time {
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs
    }
}

impl Sub<Duration> for Time {
    type Output = Time;

    fn sub(self, rhs: Duration) -> Self::Output {
        self + Duration(-rhs.0)
    }
}

impl TryFrom<u64> for Time {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        u8::try_from(value).map(Time)
    }
}

impl TryFrom<Time> for u64 {
    type Error = core::convert::Infallible;

    fn try_from(value: Time) -> Result<Self, Self::Error> {
        u64::try_from(value.0)
    }
}

impl TryFrom<u64> for Duration {
    type Error = core::num::TryFromIntError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        i8::try_from(value).map(Duration)
    }
}

impl TryFrom<Duration> for u64 {
    type Error = core::num::TryFromIntError;

    fn try_from(value: Duration) -> Result<Self, Self::Error> {
        u64::try_from(value.0)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Exam {
    subject: Subject,
    from: Time,
    to: Time,
    location: Location,
}

impl Exam {
    pub fn subject(&self) -> Subject {
        self.subject
    }

    pub fn from(&self) -> Time {
        self.from
    }

    pub fn to(&self) -> Time {
        self.to
    }

    pub fn location(&self) -> Location {
        self.location
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Day {
    index: usize,
    exams: [Option<Exam>; NUM_SUBJECTS],
}

impl Day {
    pub fn exam(&self, subject: Subject) -> Option<&Exam> {
        self.exams[subject as usize].as_ref()
    }

    fn add_exam(&mut self, exam: Exam) {
        self.exams[exam.subject as usize] = Some(exam)
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Timetable {
    days: [Day; NUM_DAYS],
}

impl Timetable {
    pub(crate) fn random(rng: &mut crate::random::Rng) -> Timetable {
        let mut days = [Day {
            index: 0,
            exams: [None; NUM_SUBJECTS],
        }; NUM_DAYS];

        for i in 0..days.len() {
            days[i].index = i;
        }

        for (subject, subject_info) in SUBJECTS.iter() {
            let mut day_used = [false; NUM_DAYS];
            for _ in 0..subject_info.exam_days {
                let day_idx = loop {
                    let day = rng.random_number_in_range(0..NUM_DAYS) as usize;
                    if !day_used[day] {
                        day_used[day] = true;
                        break day;
                    }
                };

                let exam_ends_max = WORKDAY_ENDS - subject_info.exam_max_duration;

                let exam_start_time =
                    rng.random_number_in_range(WORKDAY_BEGINS..=exam_ends_max);
                let exam_duration = rng.random_number_in_range(
                    subject_info.exam_min_duration..=subject_info.exam_max_duration,
                );

                days[day_idx].add_exam(Exam {
                    subject: *subject,
                    from: exam_start_time,
                    to: exam_start_time + exam_duration,
                    location: *rng.random_element(&subject_info.exam_places),
                });
            }
        }
        Timetable { days }
    }

    pub fn days(&self) -> &[Day] {
        &self.days
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random::Rng;
    use Location::*;
    use Subject::*;

    #[test]
    fn generate_random_timetable() {
        let mut rng = Rng::new(42);

        {
            let timetable = Timetable::random(&mut rng);

            let expected = Timetable {
                days: [
                    Day {
                        index: 0,
                        exams: [
                            None,
                            Some(Exam {
                                subject: Calculus,
                                from: Time(13),
                                to: Time(16),
                                location: PUNK,
                            }),
                            None,
                            None,
                            None,
                            Some(Exam {
                                subject: PhysicalEducation,
                                from: Time(12),
                                to: Time(13),
                                location: PUNK,
                            }),
                        ],
                    },
                    Day {
                        index: 1,
                        exams: [
                            Some(Exam {
                                subject: AlgebraAndNumberTheory,
                                from: Time(10),
                                to: Time(12),
                                location: PUNK,
                            }),
                            Some(Exam {
                                subject: Calculus,
                                from: Time(10),
                                to: Time(13),
                                location: PUNK,
                            }),
                            None,
                            None,
                            Some(Exam {
                                subject: English,
                                from: Time(11),
                                to: Time(13),
                                location: PUNK,
                            }),
                            None,
                        ],
                    },
                    Day {
                        index: 2,
                        exams: [
                            Some(Exam {
                                subject: AlgebraAndNumberTheory,
                                from: Time(14),
                                to: Time(17),
                                location: PDMI,
                            }),
                            Some(Exam {
                                subject: Calculus,
                                from: Time(14),
                                to: Time(17),
                                location: PUNK,
                            }),
                            None,
                            None,
                            None,
                            Some(Exam {
                                subject: PhysicalEducation,
                                from: Time(12),
                                to: Time(13),
                                location: PUNK,
                            }),
                        ],
                    },
                    Day {
                        index: 3,
                        exams: [None, None, None, None, None, None],
                    },
                    Day {
                        index: 4,
                        exams: [
                            Some(Exam {
                                subject: AlgebraAndNumberTheory,
                                from: Time(9),
                                to: Time(12),
                                location: PDMI,
                            }),
                            None,
                            Some(Exam {
                                subject: GeometryAndTopology,
                                from: Time(12),
                                to: Time(14),
                                location: PDMI,
                            }),
                            Some(Exam {
                                subject: ComputerScience,
                                from: Time(16),
                                to: Time(18),
                                location: ComputerClass,
                            }),
                            Some(Exam {
                                subject: English,
                                from: Time(12),
                                to: Time(14),
                                location: PUNK,
                            }),
                            None,
                        ],
                    },
                    Day {
                        index: 5,
                        exams: [
                            Some(Exam {
                                subject: AlgebraAndNumberTheory,
                                from: Time(12),
                                to: Time(14),
                                location: PUNK,
                            }),
                            Some(Exam {
                                subject: Calculus,
                                from: Time(12),
                                to: Time(15),
                                location: PUNK,
                            }),
                            Some(Exam {
                                subject: GeometryAndTopology,
                                from: Time(14),
                                to: Time(16),
                                location: PDMI,
                            }),
                            Some(Exam {
                                subject: ComputerScience,
                                from: Time(11),
                                to: Time(12),
                                location: ComputerClass,
                            }),
                            None,
                            None,
                        ],
                    },
                ],
            };

            assert_eq!(timetable, expected);
        }
    }
}
