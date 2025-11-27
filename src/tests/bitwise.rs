use crate::{Effect, Eval};

// These tests suffer because we don't support hexadecimal integers yet. We
// should update them, once we do.

#[test]
fn and() {
    // The `and` operator performs the "bitwise and" operation.

    // `61680` = `0xf0f0`, `65280` = `0xff00`
    let mut eval = Eval::start("61680 65280 and");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfTokens));
    assert_eq!(eval.stack.to_u32_slice(), &[0xf000]);
}

#[test]
fn or() {
    // The `or` operator performs the "bitwise or" operation.

    // `61680` = `0xf0f0`, `65280` = `0xff00`
    let mut eval = Eval::start("61680 65280 or");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfTokens));
    assert_eq!(eval.stack.to_u32_slice(), &[0xfff0]);
}
