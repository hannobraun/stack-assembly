use crate::Eval;

#[test]
fn evaluate_integers() {
    // Integers are tokens that consist of base-10 digits. Evaluating an integer
    // pushes the value it represents to the stack.

    let mut eval = Eval::start("3 5");
    assert_eq!(eval.stack, vec![]);

    eval.run();
    assert_eq!(eval.stack, vec![3, 5]);
}
