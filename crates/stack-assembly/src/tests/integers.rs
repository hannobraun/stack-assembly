use crate::{Effect, Eval, Script};

#[test]
fn evaluate_positive_integers() {
    // Integers are tokens that consist of base-10 digits. Evaluating an integer
    // pushes the value it represents to the stack.

    let script = Script::compile("3 5");

    let mut eval = Eval::new();
    let effect = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_i32_slice(), &[3, 5]);
}

#[test]
fn evaluate_negative_integer() {
    // Negative integers are also supported.

    let script = Script::compile("-1");

    let mut eval = Eval::new();
    let effect = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_i32_slice(), &[-1]);
}

#[test]
fn evaluate_hexadecimal_integer() {
    // Hexadecimal integer notation is supported.

    let script = Script::compile("0xf0f0");

    let mut eval = Eval::new();
    let effect = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_i32_slice(), &[0xf0f0]);
}

#[test]
fn evaluate_full_range_of_unsigned_decimal_integers() {
    // Decimal integers that are too large to fit into signed (two's complement)
    // 32-bit values are still supported, as long as they fit into an unsigned
    // 32-bit value.

    let script = Script::compile("2147483648");

    let mut eval = Eval::new();
    let effect = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[2147483648]);
}

#[test]
fn evaluate_full_range_of_unsigned_hexadecimal_integers() {
    // Hexadecimal integers that are too large to fit into signed (two's
    // complement) 32-bit values are still supported, as long as they fit into
    // an unsigned 32-bit value.

    let script = Script::compile("0x80000000");

    let mut eval = Eval::new();
    let effect = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[0x80000000]);
}

#[test]
fn trigger_effect_on_integer_overflow() {
    // If a token could theoretically be an integer, but the value it represents
    // is too large to fit in a 32-bit word, we treat it as an unknown
    // identifier.
    //
    // Long-term, it would make more sense to trigger an "integer overflow"
    // effect instead. This is tracked in the following issue:
    // https://github.com/hannobraun/stack-assembly/issues/22

    let script = Script::compile("4294967295 4294967296");

    let mut eval = Eval::new();

    let effect = eval.step(&script);
    assert_eq!(effect, None);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[4294967295]);

    let effect = eval.step(&script);
    assert_eq!(effect, Some(Effect::UnknownIdentifier));
    assert_eq!(eval.operand_stack.to_u32_slice(), &[4294967295]);
}
