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
    eval.next_operator += 1;

    eval.run();
    assert_eq!(eval.effect, Some(Effect::Yield));
    assert_eq!(eval.stack.to_u32_slice(), &[1, 1]);
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
