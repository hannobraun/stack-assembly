use crate::{Effect, Eval, Script};

#[test]
fn assert_consumes_input() {
    // `assert` consumes one input. If that input is non-zero, it does nothing
    // else.

    let script = Script::compile("1 assert");

    let mut eval = Eval::new();
    let (effect, _) = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_i32_slice(), &[]);
}

#[test]
fn assert_triggers_effect() {
    // `assert` triggers an effect, if its input is zero.

    let script = Script::compile("0 assert");

    let mut eval = Eval::new();
    let (effect, _) = eval.run(&script);

    assert_eq!(effect, Effect::AssertionFailed);
    assert_eq!(eval.operand_stack.to_i32_slice(), &[]);
}
