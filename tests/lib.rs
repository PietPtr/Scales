#[cfg(test)]
mod tests {
    use scales::{
        a, aes, ais, b, bes, c, ces, cis, d, des, dis, e, ees, eis, f, fis, g, ges, gis,
        intervals::{ImperfectInterval::*, Interval::*, PerfectInterval::*},
        notes::Notes,
        scales::{spell, Major},
    };

    #[test]
    fn test_note_steps() {
        let note = c!(4);
        assert_eq!(note.next(2), d!(4));
        assert_eq!(note.prev(1), b!(3));
    }

    #[test]
    fn test_diatonic() {
        let note = c!(4);

        let c = note.leap(Perfect(Unison));
        assert_eq!(c, c!(4));
        let d = note.leap(Major(Second));
        assert_eq!(d, d!(4));
        let e = note.leap(Major(Third));
        assert_eq!(e, e!(4));
        let f = note.leap(Perfect(Fourth));
        assert_eq!(f, f!(4));
        let g = note.leap(Perfect(Fifth));
        assert_eq!(g, g!(4));
        let a = note.leap(Major(Sixth));
        assert_eq!(a, a!(4));
        let b = note.leap(Major(Seventh));
        assert_eq!(b, b!(4));

        let c = note.fall(Perfect(Unison));
        assert_eq!(c, c!(4));
        let b = note.fall(Minor(Second));
        assert_eq!(b, b!(3));
        let a = note.fall(Minor(Third));
        assert_eq!(a, a!(3));
        let g = note.fall(Perfect(Fourth));
        assert_eq!(g, g!(3));
        let f = note.fall(Perfect(Fifth));
        assert_eq!(f, f!(3));
        let e = note.fall(Minor(Sixth));
        assert_eq!(e, e!(3));
        let d = note.fall(Minor(Seventh));
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
            spell(Major { root: ees!(4) }),
            Notes(vec![ees!(4), f!(4), g!(4), aes!(4), bes!(4), c!(5), d!(5)])
        );
    }

    #[test]
    fn circle_of_fifths() {
        // TODO: Octave differentation unnecessary here
        let _all_keys = [
            [ges!(4), aes!(4), bes!(4), ces!(4), des!(4), ees!(4), f!(4)],
            [des!(4), ees!(4), f!(4), ges!(4), aes!(4), bes!(4), c!(4)],
            [aes!(4), bes!(4), c!(4), des!(4), ees!(4), f!(4), g!(4)],
            [ees!(4), f!(4), g!(4), aes!(4), bes!(4), c!(4), d!(4)],
            [bes!(4), c!(4), d!(4), ees!(4), f!(4), g!(4), a!(4)],
            [f!(4), g!(4), a!(4), bes!(4), c!(4), d!(4), e!(4)],
            [c!(4), d!(4), e!(4), f!(4), g!(4), a!(4), b!(4)],
            [g!(4), a!(4), b!(4), c!(4), d!(4), e!(4), fis!(4)],
            [d!(4), e!(4), fis!(4), g!(4), a!(4), b!(4), cis!(4)],
            [a!(4), b!(4), cis!(4), d!(4), e!(4), fis!(4), gis!(4)],
            [e!(4), fis!(4), gis!(4), a!(4), b!(4), cis!(4), dis!(4)],
            [b!(4), cis!(4), dis!(4), e!(4), fis!(4), gis!(4), ais!(4)],
            [fis!(4), gis!(4), ais!(4), b!(4), cis!(4), dis!(4), eis!(4)],
        ];
        let mut root = ces!(4);

        for _ in 0..=12 {
            let notes = spell(Major { root });
            println!("{} Major: {}", root, notes);
            root = root.leap(Perfect(Fifth));
        }
    }
}
