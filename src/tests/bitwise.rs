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

#[test]
fn xor() {
    // The `xor` operator performs the "bitwise exclusive-or" operation.

    // `61680` = `0xf0f0`, `65280` = `0xff00`
    let mut eval = Eval::start("61680 65280 xor");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfTokens));
    assert_eq!(eval.stack.to_u32_slice(), &[0x0ff0]);
}

#[test]
fn count_ones() {
    // The `count_ones` operator outputs the number of `1` bits in its input.

    // `61680` = `0xf0f0`
    let mut eval = Eval::start("61680 count_ones");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfTokens));
    assert_eq!(eval.stack.to_u32_slice(), &[8]);
}

#[test]
fn leading_zeros() {
    // The `leading_zeros` operator outputs the number of leading zero bits in
    // its input.

    // `252645135` = `0x0f0f0f0f`
    let mut eval = Eval::start("252645135 leading_zeros");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfTokens));
    assert_eq!(eval.stack.to_u32_slice(), &[4]);
}

#[test]
fn trailing_zeros() {
    // The `trailing_zeros` operator outputs the number of trailing zero bits in
    // its input.

    // `-252645136` = `0xf0f0f0f0`
    let mut eval = Eval::start("-252645136 trailing_zeros");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfTokens));
    assert_eq!(eval.stack.to_u32_slice(), &[4]);
}

#[test]
fn rotate_left() {
    // The `rotate_left` operator rotates the bits of its first input to the
    // left, by the number of positions defined by its second input.

    // `4026531840` = `0xf0000000`
    let mut eval = Eval::start("-268435456 4 rotate_left");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfTokens));
    assert_eq!(eval.stack.to_u32_slice(), &[0x0000000f]);
}

#[test]
fn rotate_right() {
    // The `rotate_right` operator rotates the bits of its first input to the
    // right, by the number of positions defined by its second input.

    // `15` = `0x0000000f`
    let mut eval = Eval::start("15 4 rotate_right");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfTokens));
    assert_eq!(eval.stack.to_u32_slice(), &[0xf0000000]);
}

#[test]
fn shift_left() {
    // The `shift_left` operator shifts the bits of its first input to the left,
    // by the number of positions defined by its second input. Since this is a
    // shift to the left, there is no meaningful distinction between arithmetic
    // and logical shift.

    // `-16777216` = `0xff000000`
    let mut eval = Eval::start("-16777216 4 shift_left");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfTokens));
    assert_eq!(eval.stack.to_u32_slice(), &[0xf0000000]);
}

#[test]
fn shift_right_unsigned() {
    // The `shift_right` operator shifts the bits of its first input to the
    // right, by the number of positions defined by its second input. With an
    // unsigned input, there is no meaningful distinction between arithmetic and
    // logical shift.

    // `255` = `0x000000ff`
    let mut eval = Eval::start("255 4 shift_right");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfTokens));
    assert_eq!(eval.stack.to_u32_slice(), &[0x0000000f]);
}

#[test]
fn shift_right_signed() {
    // The `shift_right` operator shifts the bits of its first input to the
    // right, by the number of positions defined by its second input. This is an
    // arithmetic shift, meaning the sign of the input is preserved.

    // `-268435201` = `0xf00000ff`
    let mut eval = Eval::start("-268435201 4 shift_right");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfTokens));
    assert_eq!(eval.stack.to_u32_slice(), &[0xff00000f]);
}
