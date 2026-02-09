use crate::{Effect, Eval, Script};

#[test]
fn empty_script_triggers_out_of_tokens() {
    // Running an empty script directly triggers the "out of operators" effect.

    let script = Script::compile("");

    let mut eval = Eval::new();
    let (effect, _) = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[]);
}

#[test]
fn yield_operator_triggers_the_respective_effect() {
    // `yield` exists to moderate the communication between the evaluating
    // script and the host. It triggers an effect, that the host may interpret
    // in whatever way it deems appropriate.

    let script = Script::compile("yield");

    let mut eval = Eval::new();
    let (effect, _) = eval.run(&script);

    assert_eq!(effect, Effect::Yield);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[]);
}

#[test]
fn active_effect_prevents_evaluation_from_advancing() {
    // An active effect prevents the evaluation from advancing.

    let script = Script::compile("yield 1");

    let mut eval = Eval::new();

    let (effect, _) = eval.run(&script);
    assert_eq!(effect, Effect::Yield);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[]);

    let (effect, _) = eval.run(&script);
    assert_eq!(effect, Effect::Yield);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[]);
}

#[test]
fn stack_underflow_triggers_effect() {
    // Popping a value from an empty stack is a stack underflow and triggers an
    // effect.

    let script = Script::compile("1 +");

    let mut eval = Eval::new();
    let (effect, _) = eval.run(&script);

    assert_eq!(effect, Effect::OperandStackUnderflow);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[]);
}
