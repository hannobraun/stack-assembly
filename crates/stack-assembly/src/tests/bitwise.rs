use crate::{Effect, Eval, Script};

// Some of these tests suffer because we don't support integers that are larger
// than `i32::MAX` yet. We should update them, once we do.

#[test]
fn and() {
    // The `and` operator performs the "bitwise and" operation.

    let script = Script::compile("0xf0f0 0xff00 and");

    let mut eval = Eval::new();
    let (effect, _) = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[0xf000]);
}

#[test]
fn or() {
    // The `or` operator performs the "bitwise or" operation.

    let script = Script::compile("0xf0f0 0xff00 or");

    let mut eval = Eval::new();
    let (effect, _) = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[0xfff0]);
}

#[test]
fn xor() {
    // The `xor` operator performs the "bitwise exclusive-or" operation.

    let script = Script::compile("0xf0f0 0xff00 xor");

    let mut eval = Eval::new();
    let (effect, _) = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[0x0ff0]);
}

#[test]
fn count_ones() {
    // The `count_ones` operator outputs the number of `1` bits in its input.

    let script = Script::compile("0xf0f0 count_ones");

    let mut eval = Eval::new();
    let (effect, _) = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[8]);
}

#[test]
fn leading_zeros() {
    // The `leading_zeros` operator outputs the number of leading zero bits in
    // its input.

    let script = Script::compile("0x0f0f0f0f leading_zeros");

    let mut eval = Eval::new();
    let (effect, _) = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[4]);
}

#[test]
fn trailing_zeros() {
    // The `trailing_zeros` operator outputs the number of trailing zero bits in
    // its input.

    let script = Script::compile("0xf0f0f0f0 trailing_zeros");

    let mut eval = Eval::new();
    let (effect, _) = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[4]);
}

#[test]
fn rotate_left() {
    // The `rotate_left` operator rotates the bits of its first input to the
    // left, by the number of positions defined by its second input.

    let script = Script::compile("0xf0000000 4 rotate_left");

    let mut eval = Eval::new();
    let (effect, _) = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[0x0000000f]);
}

#[test]
fn rotate_right() {
    // The `rotate_right` operator rotates the bits of its first input to the
    // right, by the number of positions defined by its second input.

    let script = Script::compile("0x0000000f 4 rotate_right");

    let mut eval = Eval::new();
    let (effect, _) = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[0xf0000000]);
}

#[test]
fn shift_left() {
    // The `shift_left` operator shifts the bits of its first input to the left,
    // by the number of positions defined by its second input. Since this is a
    // shift to the left, there is no meaningful distinction between arithmetic
    // and logical shift.

    let script = Script::compile("0xff000000 4 shift_left");

    let mut eval = Eval::new();
    let (effect, _) = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[0xf0000000]);
}

#[test]
fn shift_right_unsigned() {
    // The `shift_right` operator shifts the bits of its first input to the
    // right, by the number of positions defined by its second input. With an
    // unsigned input, there is no meaningful distinction between arithmetic and
    // logical shift.

    let script = Script::compile("0x000000ff 4 shift_right");

    let mut eval = Eval::new();
    let (effect, _) = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[0x0000000f]);
}

#[test]
fn shift_right_signed() {
    // The `shift_right` operator shifts the bits of its first input to the
    // right, by the number of positions defined by its second input. This is an
    // arithmetic shift, meaning the sign of the input is preserved.

    let script = Script::compile("0xf00000ff 4 shift_right");

    let mut eval = Eval::new();
    let (effect, _) = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[0xff00000f]);
}
