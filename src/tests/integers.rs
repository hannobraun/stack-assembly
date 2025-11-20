use crate::{Effect, Eval};

#[test]
fn evaluate_positive_integers() {
    // Integers are tokens that consist of base-10 digits. Evaluating an integer
    // pushes the value it represents to the stack.

    let mut eval = Eval::start("3 5");
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
    // If a token could theoretically be an integer, but is too large to be a
    // signed (two's complement) 32-bit one, we treat it as an unknown
    // identifier.
    //
    // It would be more appropriate to trigger an "integer overflow" effect in
    // this case, but that would complicate the implementation. For now, that
    // would be the wrong trade-off, so this weirder but easier behavior has to
    // do.
    //
    // In principle, since the language is untyped, we could support integers
    // that cover the full range of both signed and unsigned 32-bit values. That
    // would just mean that numbers larger than `2^31-1` would end up with bit
    // patterns that also represent negative numbers (and vice versa).
    //
    // But again, to keep the initial implementation simple, we only support the
    // range of signed values for now.

    let mut eval = Eval::start("2147483647 2147483648");

    eval.step();
    assert_eq!(eval.stack, vec![2147483647]);
    assert_eq!(eval.effect, None);

    eval.step();
    assert_eq!(eval.stack, vec![2147483647]);
    assert_eq!(eval.effect, Some(Effect::UnknownIdentifier));
}
