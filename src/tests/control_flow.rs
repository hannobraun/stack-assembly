use crate::{Effect, Eval};

#[test]
fn jump() {
    // The `jump` operator takes the index of an operator as input and arranges
    // for evaluation to continue at that operator.

    let mut eval = Eval::start("1 yield 0 jump");

    eval.run();
    assert_eq!(eval.effect, Some(Effect::Yield));
    assert_eq!(eval.stack.to_u32_slice(), &[1]);

    eval.effect = None;
    eval.next_token += 1;

    eval.run();
    assert_eq!(eval.effect, Some(Effect::Yield));
    assert_eq!(eval.stack.to_u32_slice(), &[1, 1]);
}
