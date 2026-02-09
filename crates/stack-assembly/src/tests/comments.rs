use crate::{Effect, Eval, Script};

#[test]
fn full_line_comment() {
    // A line that starts with `#` is a comment and gets ignored.

    let script = Script::compile(
        "\
        # 3 5 8
        ",
    );

    let mut eval = Eval::new();
    let (effect, _) = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[]);
}

#[test]
fn end_of_line_comment() {
    // If a `#` appears somewhere within a line, everything from there on gets
    // ignored.

    let script = Script::compile("3 # 5 8");

    let mut eval = Eval::new();
    let (effect, _) = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[3]);
}

#[test]
fn comment_without_whitespace() {
    // Any `#` introduces a comment, even if not delineated by whitespace.

    let script = Script::compile("3 #5 8");

    let mut eval = Eval::new();
    let (effect, _) = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[3]);
}
