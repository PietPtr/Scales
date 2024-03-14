use std::cmp::max;

///             RAISE       LOWER
/// Minor       Major       Diminshed
/// Perfect     Augmented   Diminished
/// Major       Augmented   Minor
///
///         DIM     MIN     PER     MAJ     AUG
/// uni     x       x       1       -       2
/// 2nd     0?      1       -       2       3
/// 3rd     2       3       -       4       5
/// 4th     4       -       5       -       6
/// 5th     5       -       7       -       8
/// 6th     7       8       -       9       10
/// 7th     9       10      -       11      12
///
/// 8th     11      -       12      -       13
/// 9th     12      13      -       14      15
/// etc?

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Interval {
    Diminshed(AnyInterval),
    Minor(ImperfectInterval),
    Perfect(PerfectInterval),
    Major(ImperfectInterval),
    Augmented(AnyInterval),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PerfectInterval {
    Unison,
    Fourth,
    Fifth,
    Octave,
}

pub trait Diatonic {
    fn diatonic_steps(&self) -> u32;
}

impl PerfectInterval {
    pub fn size(&self) -> u32 {
        use PerfectInterval::*;
        match *self {
            Unison => 0,
            Fourth => 5,
            Fifth => 7,
            Octave => 12,
        }
    }
}

impl Diatonic for PerfectInterval {
    fn diatonic_steps(&self) -> u32 {
        use PerfectInterval::*;
        match *self {
            Unison => 0,
            Fourth => 3,
            Fifth => 4,
            Octave => 7,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ImperfectInterval {
    Second,
    Third,
    Sixth,
    Seventh,
}

impl ImperfectInterval {
    pub fn minor_size(&self) -> u32 {
        use ImperfectInterval::*;
        match *self {
            Second => 1,
            Third => 3,
            Sixth => 8,
            Seventh => 10,
        }
    }
    pub fn major_size(&self) -> u32 {
        self.minor_size() + 1
    }
}

impl Diatonic for ImperfectInterval {
    fn diatonic_steps(&self) -> u32 {
        use ImperfectInterval::*;
        match *self {
            Second => 1,
            Third => 2,
            Sixth => 5,
            Seventh => 6,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AnyInterval {
    Perfect(PerfectInterval),
    Imperfect(ImperfectInterval),
}

impl Diatonic for AnyInterval {
    fn diatonic_steps(&self) -> u32 {
        use AnyInterval::*;
        match *self {
            Perfect(interval) => interval.diatonic_steps(),
            Imperfect(interval) => interval.diatonic_steps(),
        }
    }
}

impl Interval {
    pub fn size(&self) -> u32 {
        use Interval::*;
        match *self {
            Diminshed(interval) => match interval {
                AnyInterval::Perfect(interval) => max(0, interval.size() - 1),
                AnyInterval::Imperfect(interval) => interval.minor_size() - 1,
            },
            Minor(interval) => interval.minor_size(),
            Perfect(interval) => interval.size(),
            Major(interval) => interval.major_size(),
            Augmented(interval) => match interval {
                AnyInterval::Perfect(interval) => interval.size() + 1,
                AnyInterval::Imperfect(interval) => interval.major_size() + 1,
            },
        }
    }
}

impl Diatonic for Interval {
    fn diatonic_steps(&self) -> u32 {
        use Interval::*;
        match *self {
            Diminshed(interval) => interval.diatonic_steps(),
            Minor(interval) => interval.diatonic_steps(),
            Perfect(interval) => interval.diatonic_steps(),
            Major(interval) => interval.diatonic_steps(),
            Augmented(interval) => interval.diatonic_steps(),
        }
    }
}
