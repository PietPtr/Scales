#[cfg(test)]
mod tests {
    use scales::{
        a, aes, b, bes, c, d, dis, e, es, f, g,
        intervals::Interval,
        notes::Notes,
        scales::{spell, Major},
    };

    #[test]
    fn test_note_steps() {
        let note = c!(4);
        assert_eq!(note.next(2), d!(4));
        assert_eq!(note.prev(1), b!(4));
    }

    #[test]
    fn test_diatonic() {
        let note = c!(4);

        let c = note.leap(Interval::Unison);
        assert_eq!(c, c!(4));
        let d = note.leap(Interval::MajorSecond);
        assert_eq!(d, d!(4));
        let e = note.leap(Interval::MajorThird);
        assert_eq!(e, e!(4));
        let f = note.leap(Interval::PerfectFourth);
        assert_eq!(f, f!(4));
        let g = note.leap(Interval::PerfectFifth);
        assert_eq!(g, g!(4));
        let a = note.leap(Interval::MajorSixth);
        assert_eq!(a, a!(4));
        let b = note.leap(Interval::MajorSeventh);
        assert_eq!(b, b!(4));

        let c = note.fall(Interval::Unison);
        assert_eq!(c, c!(4));
        let b = note.fall(Interval::MinorSecond);
        assert_eq!(b, b!(3));
        let a = note.fall(Interval::MinorThird);
        assert_eq!(a, a!(3));
        let g = note.fall(Interval::PerfectFourth);
        assert_eq!(g, g!(3));
        let f = note.fall(Interval::PerfectFifth);
        assert_eq!(f, f!(3));
        let e = note.fall(Interval::MinorSixth);
        assert_eq!(e, e!(3));
        let d = note.fall(Interval::MinorSeventh);
        assert_eq!(d, d!(3));
    }

    #[test]
    fn test_spell() {
        assert_eq!(
            spell(Major { root: c!(4) }),
            Notes(vec![c!(4), d!(4), e!(4), f!(4), g!(4), a!(4), b!(4)])
        );

        println!("D# Major: {}", spell(Major { root: dis!(4) }));
        assert_eq!(
            spell(Major { root: es!(4) }),
            Notes(vec![es!(4), f!(4), g!(4), aes!(4), bes!(4), c!(5), d!(5)])
        );
    }
}
