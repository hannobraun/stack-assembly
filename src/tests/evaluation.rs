use crate::{Effect, Eval};

#[test]
fn yield_operator_triggers_the_respective_effect() {
    // `yield` exists to moderate the communication between the evaluating
    // script and the host. It triggers an effect, that the host may interpret
    // in whatever way it deems appropriate.

    let mut eval = Eval::start("yield");
    eval.run();

    assert_eq!(eval.stack, vec![]);
    assert_eq!(eval.effect, Some(Effect::Yield));
}

#[test]
fn active_effect_prevents_evaluation_from_advancing() {
    // An active effect prevents the evaluation from advancing.

    let mut eval = Eval::start("1");
    eval.effect = Some(Effect::Yield);

    eval.run();
    assert_eq!(eval.stack, vec![]);
    assert_eq!(eval.effect, Some(Effect::Yield));
}
