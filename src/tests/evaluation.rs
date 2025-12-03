use crate::{Effect, Eval};

#[test]
fn empty_script_triggers_out_of_tokens() {
    // Running an empty script does nothing.

    let mut eval = Eval::start("");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.stack.to_u32_slice(), &[]);
}

#[test]
fn starting_evaluation_does_not_evaluate_any_operators() {
    // Starting the evaluation readies it, but does not yet evaluate any
    // operators.

    let eval = Eval::start("yield");
    assert_eq!(eval.effect, None);
    assert_eq!(eval.stack.to_u32_slice(), &[]);
}

#[test]
fn yield_operator_triggers_the_respective_effect() {
    // `yield` exists to moderate the communication between the evaluating
    // script and the host. It triggers an effect, that the host may interpret
    // in whatever way it deems appropriate.

    let mut eval = Eval::start("yield");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::Yield));
    assert_eq!(eval.stack.to_u32_slice(), &[]);
}

#[test]
fn active_effect_prevents_evaluation_from_advancing() {
    // An active effect prevents the evaluation from advancing.

    let mut eval = Eval::start("1");
    eval.effect = Some(Effect::Yield);

    eval.run();
    assert_eq!(eval.effect, Some(Effect::Yield));
    assert_eq!(eval.stack.to_u32_slice(), &[]);
}

#[test]
fn stack_underflow_triggers_effect() {
    // Popping a value from an empty stack is a stack underflow and triggers an
    // effect.

    let mut eval = Eval::start("1 +");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::StackUnderflow));
    assert_eq!(eval.stack.to_u32_slice(), &[]);
}
