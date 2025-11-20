use crate::Eval;

#[test]
fn add() {
    // The `+` operator consumes two inputs and pushes their sum.

    let mut eval = Eval::start("1 2 +");
    eval.run();

    assert_eq!(eval.effect, None);
    assert_eq!(eval.stack, vec![3]);
}

#[test]
fn add_wraps_on_signed_overflow() {
    // An addition that overflows the range of a signed 32-bit integer wraps.

    let mut eval = Eval::start("2147483647 1 +");
    eval.run();

    assert_eq!(eval.effect, None);
    assert_eq!(eval.stack, vec![2147483648]);
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
    assert_eq!(eval.stack, vec![0]);
}
