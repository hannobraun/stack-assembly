use crate::{Effect, Eval};

#[test]
fn evaluate_positive_integers() {
    // Integers are tokens that consist of base-10 digits. Evaluating an integer
    // pushes the value it represents to the stack.

    let mut eval = Eval::start("3 5");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.stack.to_i32_slice(), &[3, 5]);
}

#[test]
fn evaluate_negative_integer() {
    // Negative integers are also supported.

    let mut eval = Eval::start("-1");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.stack.to_i32_slice(), &[-1]);
}

#[test]
fn evaluate_hexadecimal_integer() {
    // Hexadecimal integer notation is supported.

    let mut eval = Eval::start("0xf0f0");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.stack.to_i32_slice(), &[0xf0f0]);
}

#[test]
fn evaluate_full_range_of_unsigned_decimal_integers() {
    // Decimal integers that are too large to fit into signed (two's complement)
    // 32-bit values are still supported, as long as they fit into an unsigned
    // 32-bit value.

    let mut eval = Eval::start("2147483648");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.stack.to_u32_slice(), &[2147483648]);
}

#[test]
fn evaluate_full_range_of_unsigned_hexadecimal_integers() {
    // Hexadecimal integers that are too large to fit into signed (two's
    // complement) 32-bit values are still supported, as long as they fit into
    // an unsigned 32-bit value.

    let mut eval = Eval::start("0x80000000");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.stack.to_u32_slice(), &[0x80000000]);
}

#[test]
fn trigger_effect_on_integer_overflow() {
    // If a token could theoretically be an integer, but is too large to be a
    // signed (two's complement) 32-bit one, we treat it as an unknown
    // identifier.
    //
    // Long-term, this is undesired behavior, which is tracked in the following
    // issue:
    // https://github.com/hannobraun/stack-assembly/issues/18

    let mut eval = Eval::start("4294967295 4294967296");

    eval.step();
    assert_eq!(eval.effect, None);
    assert_eq!(eval.stack.to_u32_slice(), &[4294967295]);

    eval.step();
    assert_eq!(eval.effect, Some(Effect::UnknownIdentifier));
    assert_eq!(eval.stack.to_u32_slice(), &[4294967295]);
}
