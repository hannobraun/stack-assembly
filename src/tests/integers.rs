use crate::{Effect, Eval};

#[test]
fn evaluate_positive_integers() {
    // Integers are tokens that consist of base-10 digits. Evaluating an integer
    // pushes the value it represents to the stack.

    let mut eval = Eval::start("3 5");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.stack.to_u32_slice(), &[3, 5]);
}

#[test]
fn evaluate_negative_integer() {
    // To reflect that the language is untyped and views all values as strings
    // of bits, 32 wide, the stack represents all values as unsigned. But the
    // language itself supports unsigned (two's complement) values.

    let mut eval = Eval::start("-1");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.stack.to_u32_slice(), &[4294967295]);
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

    let mut eval = Eval::start("2147483647 2147483648");

    eval.step();
    assert_eq!(eval.effect, None);
    assert_eq!(eval.stack.to_u32_slice(), &[2147483647]);

    eval.step();
    assert_eq!(eval.effect, Some(Effect::UnknownIdentifier));
    assert_eq!(eval.stack.to_u32_slice(), &[2147483647]);
}
