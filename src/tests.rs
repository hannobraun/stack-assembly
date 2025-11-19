use crate::{Effect, Eval};

#[test]
fn evaluate_positive_integers() {
    // Integers are tokens that consist of base-10 digits. Evaluating an integer
    // pushes the value it represents to the stack.

    let mut eval = Eval::start("3 5");
    assert_eq!(eval.stack, vec![]);
    assert_eq!(eval.effect, None);

    eval.run();
    assert_eq!(eval.stack, vec![3, 5]);
    assert_eq!(eval.effect, None);
}

#[test]
fn evaluate_negative_integer() {
    // To reflect that the language is untyped and views all values as strings
    // of bits, 32 wide, the stack represents all values as unsigned. But the
    // language itself supports unsigned (two's complement) values.

    let mut eval = Eval::start("-1");
    eval.run();

    assert_eq!(eval.stack, vec![4294967295]);
    assert_eq!(eval.effect, None);
}

#[test]
fn trigger_effect_on_integer_overflow() {
    // If an integer is too large to fit in a signed (two's complement) 32-bit
    // value, evaluating it triggers the corresponding effect.
    //
    // In principle, since the language is untyped, we could support integers
    // that cover the full range of both signed and unsigned 32-bit values. That
    // would just mean that numbers larger than `2^31-1` would end up with bit
    // patterns that also represent negative numbers (and vice versa).
    //
    // But to keep the initial implementation simple, we only support the range
    // of signed values for now.

    let mut eval = Eval::start("2147483647 2147483648");

    eval.step();
    assert_eq!(eval.stack, vec![2147483647]);
    assert_eq!(eval.effect, None);

    eval.step();
    assert_eq!(eval.stack, vec![2147483647]);
    assert_eq!(eval.effect, Some(Effect::IntegerOverflow));
}
