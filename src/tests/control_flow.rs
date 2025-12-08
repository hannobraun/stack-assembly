use crate::{Effect, Eval};

#[test]
fn jump() {
    // The `jump` operator takes the index of an operator (usually provided by
    // a reference) as input, then arranges for evaluation to continue at that
    // operator.

    let mut eval = Eval::start("start: 1 yield @start jump");

    eval.run();
    assert_eq!(eval.effect, Some(Effect::Yield));
    assert_eq!(eval.stack.to_u32_slice(), &[1]);

    eval.effect = None;

    eval.run();
    assert_eq!(eval.effect, Some(Effect::Yield));
    assert_eq!(eval.stack.to_u32_slice(), &[1, 1]);
}

#[test]
fn jump_if_behaves_like_jump_on_nonzero_condition() {
    // The `jump_if` operator is the conditional variant of `jump`. In addition
    // to an operator index, it takes a condition that. If that condition is
    // non-zero, `jump_if` behaves like `jump`.

    let mut eval = Eval::start("1 @target jump_if 1 target: 2");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.stack.to_u32_slice(), &[2]);
}

#[test]
fn jump_if_does_nothing_on_zero_condition() {
    // The `jump_if` operator is the conditional variant of `jump`. In addition
    // to an operator index, it takes a condition that. If that condition is
    // zero, `jump_if` does nothing.

    let mut eval = Eval::start("0 @target jump_if 1 target: 2");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.stack.to_u32_slice(), &[1, 2]);
}

#[test]
fn invalid_reference_triggers_effect() {
    // A reference that is not paired with a matching label can't return a
    // sensible value and must trigger an effect.

    let mut eval = Eval::start("@invalid");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::InvalidReference));
    assert_eq!(eval.stack.to_u32_slice(), &[]);
}
