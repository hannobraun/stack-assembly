use crate::{Effect, Eval};

#[test]
fn jump() {
    // The `jump` operator takes the index of an operator as input and arranges
    // for evaluation to continue at that operator.

    let mut eval = Eval::start("1 yield 0 jump");

    eval.run();
    assert_eq!(eval.stack, vec![1]);
    assert_eq!(eval.effect, Some(Effect::Yield));

    eval.effect = None;
    eval.run();
    assert_eq!(eval.stack, vec![1, 1]);
    assert_eq!(eval.effect, Some(Effect::Yield));
}
