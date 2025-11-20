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
