use crate::Eval;

#[test]
fn add() {
    // The `+` operator consumes two inputs and pushes their sum.

    let mut eval = Eval::start("1 2 +");
    eval.run();

    assert_eq!(eval.effect, None);
    assert_eq!(eval.stack, vec![3]);
}
