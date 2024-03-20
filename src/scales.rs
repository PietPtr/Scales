use crate::{
    intervals::{AnyInterval, ImperfectInterval, Interval, PerfectInterval},
    notes::{Note, Notes},
};

use ImperfectInterval::*;
use Interval::*;
use PerfectInterval::*;

pub trait Scale {
    fn intervals() -> Vec<Interval>;
    fn root(&self) -> Note;
}

pub fn spell<S: Scale>(scale: S) -> Notes {
    let mut notes = vec![];

    let mut intervals: Vec<Interval> = S::intervals();
    intervals.sort_by_key(|a| a.size());

    for interval in intervals {
        notes.push(scale.root().leap(interval));
    }

    Notes(notes)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Qualities {
    Major,
    Minor,
    Diminshed,
    Sus4,
    Sus2,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Tensions {
    Seventh,
    Ninth,
    Eleventh,
    Thirteenth,
}

pub fn replace_in_scale(mut scale: Vec<Interval>, src: Interval, dst: Interval) -> Vec<Interval> {
    if let Some(fourth_idx) = scale.iter().position(|&p| p == src) {
        scale[fourth_idx] = dst;
    }
    scale
}

pub struct Lydian {
    pub root: Note,
}

impl Scale for Lydian {
    fn intervals() -> Vec<Interval> {
        vec![
            Perfect(Unison),
            Major(Second),
            Major(Third),
            Augmented(AnyInterval::Perfect(Fourth)),
            Perfect(Fifth),
            Major(Sixth),
            Major(Seventh),
        ]
    }

    fn root(&self) -> Note {
        self.root
    }
}

pub struct Ionian {
    pub root: Note,
}

pub type Major = Ionian;

impl Scale for Ionian {
    fn intervals() -> Vec<Interval> {
        replace_in_scale(
            Lydian::intervals(),
            Augmented(AnyInterval::Perfect(Fourth)),
            Perfect(Fourth),
        )
    }

    fn root(&self) -> Note {
        self.root
    }
}

pub struct Mixolydian {
    pub root: Note,
}

impl Scale for Mixolydian {
    fn intervals() -> Vec<Interval> {
        replace_in_scale(Ionian::intervals(), Major(Seventh), Minor(Seventh))
    }

    fn root(&self) -> Note {
        self.root
    }
}

pub struct Dorian {
    pub root: Note,
}

impl Scale for Dorian {
    fn intervals() -> Vec<Interval> {
        replace_in_scale(Mixolydian::intervals(), Major(Third), Minor(Third))
    }

    fn root(&self) -> Note {
        self.root
    }
}

pub struct Aeolian {
    pub root: Note,
}

pub type Minor = Aeolian;

impl Scale for Aeolian {
    fn intervals() -> Vec<Interval> {
        replace_in_scale(Dorian::intervals(), Major(Sixth), Minor(Sixth))
    }

    fn root(&self) -> Note {
        self.root
    }
}

pub struct Phrygian {
    pub root: Note,
}

impl Scale for Phrygian {
    fn intervals() -> Vec<Interval> {
        replace_in_scale(Aeolian::intervals(), Major(Second), Minor(Second))
    }

    fn root(&self) -> Note {
        self.root
    }
}

pub struct Locrian {
    pub root: Note,
}

impl Scale for Locrian {
    fn intervals() -> Vec<Interval> {
        replace_in_scale(
            Phrygian::intervals(),
            Perfect(Fifth),
            Diminshed(AnyInterval::Perfect(Fifth)),
        )
    }

    fn root(&self) -> Note {
        self.root
    }
}
