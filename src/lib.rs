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
        /* An asterisk-based comment here is required to trigger the fmt bug.

        A single-line double-slash-prefixed comment is not enough. A
        single-line double-slash-prefixed comment is not enough either.
        Side note: If you have a combination of two, or all three of the
        above mentioned types of comments here, some of them disappear
        after `cargo fmt`.

        The above comment applied with a field definition following this
        comment, too.
        */
    }
}
