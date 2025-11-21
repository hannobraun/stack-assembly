use crate::Eval;

#[test]
fn add() {
    // The `+` operator consumes two inputs and pushes their sum.

    let mut eval = Eval::start("1 2 +");
    eval.run();

    assert_eq!(eval.effect, None);
    assert_eq!(eval.stack.values, vec![3]);
}

#[test]
fn add_wraps_on_signed_overflow() {
    // An addition that overflows the range of a signed 32-bit integer wraps.

    let mut eval = Eval::start("2147483647 1 +");
    eval.run();

    assert_eq!(eval.effect, None);
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

    assert_eq!(eval.effect, None);
    assert_eq!(eval.stack.values, vec![0]);
}

#[test]
fn subtract() {
    // The `-` operator consumes two inputs and pushes their difference.

    let mut eval = Eval::start("2 1 -");
    eval.run();

    assert_eq!(eval.effect, None);
    assert_eq!(eval.stack.values, vec![1]);
}

#[test]
fn subtract_wraps_on_signed_overflow() {
    // A subtraction that overflows the range of a signed 32-bit integer wraps.

    let mut eval = Eval::start("-2147483648 1 -");
    eval.run();

    assert_eq!(eval.effect, None);
    assert_eq!(eval.stack.values, vec![2147483647]);
}

#[test]
fn subtract_wraps_on_unsigned_overflow() {
    // A subtraction that overflows the range of an unsigned 32-bit integer
    // wraps.

    let mut eval = Eval::start("0 1 -");
    eval.run();

    assert_eq!(eval.effect, None);
    assert_eq!(eval.stack.values, vec![4294967295]);
}

#[test]
fn multiply() {
    // The `*` operator consumes two inputs and pushes their product.

    let mut eval = Eval::start("2 3 *");
    eval.run();

    assert_eq!(eval.effect, None);
    assert_eq!(eval.stack.values, vec![6]);
}

#[test]
fn multiply_wraps_on_signed_overflow() {
    // A multiplication that overflows the range of a signed 32-bit integer
    // wraps.

    let mut eval = Eval::start("2147483647 2 *");
    eval.run();

    assert_eq!(eval.effect, None);
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

    assert_eq!(eval.effect, None);
    assert_eq!(eval.stack.values, vec![4294967294]);
}

#[test]
fn divide() {
    // The `/` operator consumes two inputs and performs integer division,
    // pushing their quotient and the remainder.

    let mut eval = Eval::start("5 2 /");
    eval.run();

    assert_eq!(eval.effect, None);
    assert_eq!(eval.stack.values, vec![2, 1]);
}
