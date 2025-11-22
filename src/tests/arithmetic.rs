use crate::{Effect, Eval};

#[test]
fn add() {
    // The `+` operator consumes two inputs and pushes their sum.

    let mut eval = Eval::start("1 2 +");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfTokens));
    assert_eq!(eval.stack.values, vec![3]);
}

#[test]
fn add_wraps_on_signed_overflow() {
    // An addition that overflows the range of a signed 32-bit integer wraps.

    let mut eval = Eval::start("2147483647 1 +");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfTokens));
    assert_eq!(eval.stack.values, vec![2147483648]);
}

#[test]
fn add_wraps_on_unsigned_overflow() {
    // An addition that overflows the range of an unsigned 32-bit integer wraps.
    //
    // This test currently needs to represent the largest unsigned integer as
    // `-1`, due to the limitation tracked in this issue:
    // https://github.com/hannobraun/stack-assembly/issues/18

    let mut eval = Eval::start("-1 1 +");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfTokens));
    assert_eq!(eval.stack.values, vec![0]);
}

#[test]
fn subtract() {
    // The `-` operator consumes two inputs and pushes their difference.

    let mut eval = Eval::start("2 1 -");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfTokens));
    assert_eq!(eval.stack.values, vec![1]);
}

#[test]
fn subtract_wraps_on_signed_overflow() {
    // A subtraction that overflows the range of a signed 32-bit integer wraps.

    let mut eval = Eval::start("-2147483648 1 -");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfTokens));
    assert_eq!(eval.stack.values, vec![2147483647]);
}

#[test]
fn subtract_wraps_on_unsigned_overflow() {
    // A subtraction that overflows the range of an unsigned 32-bit integer
    // wraps.

    let mut eval = Eval::start("0 1 -");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfTokens));
    assert_eq!(eval.stack.values, vec![4294967295]);
}

#[test]
fn multiply() {
    // The `*` operator consumes two inputs and pushes their product.

    let mut eval = Eval::start("2 3 *");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfTokens));
    assert_eq!(eval.stack.values, vec![6]);
}

#[test]
fn multiply_wraps_on_signed_overflow() {
    // A multiplication that overflows the range of a signed 32-bit integer
    // wraps.

    let mut eval = Eval::start("2147483647 2 *");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfTokens));
    assert_eq!(eval.stack.values, vec![4294967294]);
}

#[test]
fn multiply_wraps_on_unsigned_overflow() {
    // A multiplication that overflows the range of an unsigned 32-bit integer
    // wraps.
    //
    // This test currently needs to represent the largest unsigned integer as
    // `-1`, due to the limitation tracked in this issue:
    // https://github.com/hannobraun/stack-assembly/issues/18

    let mut eval = Eval::start("-1 2 *");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfTokens));
    assert_eq!(eval.stack.values, vec![4294967294]);
}

#[test]
fn divide() {
    // The `/` operator consumes two inputs and performs integer division,
    // pushing their quotient and the remainder.

    let mut eval = Eval::start("5 2 /");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfTokens));
    assert_eq!(eval.stack.values, vec![2, 1]);
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

    let mut eval = Eval::start("5 -2 /");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfTokens));
    assert_eq!(eval.stack.values, vec![4294967294, 1]);
}

#[test]
fn divide_by_zero_triggers_effect() {
    // A division by zero cannot be reasonably handled and triggers the
    // respective effect.

    let mut eval = Eval::start("1 0 /");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::DivisionByZero));
    assert_eq!(eval.stack.values, vec![]);
}

#[test]
fn divide_triggers_effect_on_overflow() {
    // In contrast to other arithmetic operations, division overflow only in one
    // specific circumstance, and that circumstance is unlikely to be
    // intentional. Therefore it triggers an effect.
    //
    // If it is, the user can work around it quite easily, by watching for the
    // inputs that trigger the overflow and not doing the division then.

    let mut eval = Eval::start("-2147483648 -1 /");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::IntegerOverflow));
    assert_eq!(eval.stack.values, vec![]);
}
