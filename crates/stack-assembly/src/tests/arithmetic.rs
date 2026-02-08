use crate::{Effect, Eval, Script};

#[test]
fn add() {
    // The `+` operator consumes two inputs and pushes their sum.

    let script = Script::compile("1 2 +");

    let mut eval = Eval::new();
    eval.run(&script);

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.operand_stack.to_i32_slice(), &[3]);
}

#[test]
fn add_wraps_on_signed_overflow() {
    // An addition wraps, if it overflows the range of a signed 32-bit integer.

    let script = Script::compile("2147483647 1 +");

    let mut eval = Eval::new();
    eval.run(&script);

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.operand_stack.to_i32_slice(), &[-2147483648]);
}

#[test]
fn add_wraps_on_unsigned_overflow() {
    // An addition wraps, if it overflows the range of an unsigned 32-bit
    // integer.
    //
    // This test currently needs to represent the largest unsigned integer as
    // `-1`, due to the limitation tracked in this issue:
    // https://github.com/hannobraun/stack-assembly/issues/18

    let script = Script::compile("-1 1 +");

    let mut eval = Eval::new();
    eval.run(&script);

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.operand_stack.to_i32_slice(), &[0]);
}

#[test]
fn subtract() {
    // The `-` operator consumes two inputs and pushes their difference.

    let script = Script::compile("2 1 -");

    let mut eval = Eval::new();
    eval.run(&script);

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.operand_stack.to_i32_slice(), &[1]);
}

#[test]
fn subtract_wraps_on_signed_overflow() {
    // A subtraction wraps, if it overflows the range of a signed 32-bit
    // integer.

    let script = Script::compile("-2147483648 1 -");

    let mut eval = Eval::new();
    eval.run(&script);

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.operand_stack.to_i32_slice(), &[2147483647]);
}

#[test]
fn subtract_wraps_on_unsigned_overflow() {
    // A subtraction wraps, if it overflows the range of an unsigned 32-bit
    // integer.

    let script = Script::compile("0 1 -");

    let mut eval = Eval::new();
    eval.run(&script);

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.operand_stack.to_i32_slice(), &[-1]);
}

#[test]
fn multiply() {
    // The `*` operator consumes two inputs and pushes their product.

    let script = Script::compile("2 3 *");

    let mut eval = Eval::new();
    eval.run(&script);

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.operand_stack.to_i32_slice(), &[6]);
}

#[test]
fn multiply_wraps_on_signed_overflow() {
    // A multiplication wraps, if it overflows the range of a signed 32-bit
    // integer.

    let script = Script::compile("2147483647 2 *");

    let mut eval = Eval::new();
    eval.run(&script);

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.operand_stack.to_i32_slice(), &[-2]);
}

#[test]
fn multiply_wraps_on_unsigned_overflow() {
    // A multiplication wraps, if it overflows the range of an unsigned 32-bit
    // integer.
    //
    // This test currently needs to represent the largest unsigned integer as
    // `-1`, due to the limitation tracked in this issue:
    // https://github.com/hannobraun/stack-assembly/issues/18

    let script = Script::compile("-1 2 *");

    let mut eval = Eval::new();
    eval.run(&script);

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.operand_stack.to_i32_slice(), &[-2]);
}

#[test]
fn divide() {
    // The `/` operator consumes two inputs and performs integer division,
    // pushing their quotient and the remainder.

    let script = Script::compile("5 2 /");

    let mut eval = Eval::new();
    eval.run(&script);

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.operand_stack.to_i32_slice(), &[2, 1]);
}

#[test]
fn divide_treats_its_inputs_as_signed() {
    // For most arithmetic operations, it makes no difference whether their
    // inputs are considered to be unsigned or signed in two's complement
    // representation. In terms of what happens with the bits, both are mostly
    // treated the same.
    //
    // Not so for division. Here, either case requires a dedicated algorithm
    // that won't work for the other. The `/` operator treats its inputs as
    // signed.

    let script = Script::compile("5 -2 /");

    let mut eval = Eval::new();
    eval.run(&script);

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.operand_stack.to_i32_slice(), &[-2, 1]);
}

#[test]
fn divide_by_zero_triggers_effect() {
    // A division by zero cannot be reasonably handled and triggers the
    // respective effect.

    let script = Script::compile("1 0 /");

    let mut eval = Eval::new();
    eval.run(&script);

    assert_eq!(eval.effect, Some(Effect::DivisionByZero));
    assert_eq!(eval.operand_stack.to_i32_slice(), &[]);
}

#[test]
fn divide_triggers_effect_on_overflow() {
    // In contrast to other arithmetic operations, division overflows only in
    // one specific circumstance, and that circumstance is unlikely to be
    // intentional. Therefore it triggers an effect instead of wrapping.
    //
    // If it is intentional, the user can work around it quite easily, by
    // watching for the inputs that trigger the overflow and not doing the
    // division then.

    let script = Script::compile("-2147483648 -1 /");

    let mut eval = Eval::new();
    eval.run(&script);

    assert_eq!(eval.effect, Some(Effect::IntegerOverflow));
    assert_eq!(eval.operand_stack.to_i32_slice(), &[]);
}
