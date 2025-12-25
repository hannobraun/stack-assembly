use crate::{Effect, Eval};

#[test]
fn assert_consumes_input() {
    // `assert` consumes one input. If that input is non-zero, it does nothing
    // else.

    let mut eval = Eval::start("1 assert");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.stack.to_i32_slice(), &[]);
}

#[test]
fn assert_triggers_effect() {
    // `assert` triggers an effect, if its input is zero.

    let mut eval = Eval::start("0 assert");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::AssertionFailed));
    assert_eq!(eval.stack.to_i32_slice(), &[]);
}
