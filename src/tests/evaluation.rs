use crate::{Effect, Eval};

#[test]
fn active_effect_prevents_evaluation_from_advancing() {
    // An active effect prevents the evaluation from advancing.

    let mut eval = Eval::start("1");
    eval.effect = Some(Effect::IntegerOverflow);

    eval.run();
    assert_eq!(eval.stack, vec![]);
    assert_eq!(eval.effect, Some(Effect::IntegerOverflow));
}
