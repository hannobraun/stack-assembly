use crate::Eval;

#[test]
fn evaluate_positive_integers() {
    // Integers are tokens that consist of base-10 digits. Evaluating an integer
    // pushes the value it represents to the stack.

    let mut eval = Eval::start("3 5");
    assert_eq!(eval.stack, vec![]);

    eval.run();
    assert_eq!(eval.stack, vec![3, 5]);
}

#[test]
fn evaluate_negative_integer() {
    // To reflect that the language is untyped and views all values as strings
    // of bits, 32 wide, the stack represents all values as unsigned. But the
    // language itself supports unsigned (two's complement) values.

    let mut eval = Eval::start("-1");
    eval.run();

    assert_eq!(eval.stack, vec![4294967295]);
}
