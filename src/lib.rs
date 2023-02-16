#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

macro_rules! num_slots {
    ($c:expr) => {
        $c + 1
    };
}

pub fn f() {
    #[allow(dead_code)]
    #[allow(unused_braces)]
    struct Inner<const C: usize>
    where
        [(); { num_slots!(C) }]:, {
        /* An asterisk-based, or a double-slash-prefixed, comment here is
           required to trigger the fmt bug.

        A single-line triple-slash-prefixed comment (with a field following it) is not enough - it will not trigger the fmt bug.

        Side note: If you have a combination of two, or all three of the
        above mentioned types of comments here, some of them disappear
        after `cargo fmt`.

        The bug gets triggered even if a field definition following the
        (asterisk-based, or a double-slash-prefixed) comment, too.
        */
    }
}
