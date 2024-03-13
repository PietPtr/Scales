#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Interval {
    Unison,
    MinorSecond,
    MajorSecond,
    MinorThird,
    MajorThird,
    PerfectFourth,
    AugmentedFourth, // TODO: all the other funky interval names
    Tritone,         // TODO: or change to only numbers and qualities?
    DiminshedFifth,
    PerfectFifth,
    MinorSixth,
    MajorSixth,
    MinorSeventh,
    MajorSeventh,
    Octave,
}

impl Interval {
    pub fn size(&self) -> u32 {
        use Interval::*;
        match *self {
            Unison => 0,
            MinorSecond => 1,
            MajorSecond => 2,
            MinorThird => 3,
            MajorThird => 4,
            PerfectFourth => 5,
            AugmentedFourth => 6,
            Tritone => 6,
            DiminshedFifth => 6,
            PerfectFifth => 7,
            MinorSixth => 8,
            MajorSixth => 9,
            MinorSeventh => 10,
            MajorSeventh => 11,
            Octave => 12,
        }
    }

    pub fn diatonic_steps(&self) -> u32 {
        use Interval::*;
        match *self {
            Unison => 0,
            MinorSecond => 1,
            MajorSecond => 1,
            MinorThird => 2,
            MajorThird => 2,
            PerfectFourth => 3,
            AugmentedFourth => 3,
            Tritone => unimplemented!(),
            DiminshedFifth => 4,
            PerfectFifth => 4,
            MinorSixth => 5,
            MajorSixth => 5,
            MinorSeventh => 6,
            MajorSeventh => 6,
            Octave => 7,
        }
    }
}
