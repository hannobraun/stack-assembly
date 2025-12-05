use crate::{Effect, Eval};

#[test]
fn full_line_comment() {
    // A line that starts with `#` is a comment and gets ignored.

    let script = "\
        # 3 5 8
    ";

    let mut eval = Eval::start(script);
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.stack.to_u32_slice(), &[]);
}

#[test]
fn end_of_line_comment() {
    // If a `#` appears somewhere within a line, everything from there on gets
    // ignored.

    let mut eval = Eval::start("3 # 5 8");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.stack.to_u32_slice(), &[3]);
}
