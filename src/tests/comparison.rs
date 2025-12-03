use crate::{Effect, Eval};

#[test]
fn smaller_outputs_one_if_smaller() {
    // The `<` operator output `1`, if its first input is smaller than the
    // second, treating both inputs as signed.

    let mut eval = Eval::start("-1 0 <");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.stack.to_u32_slice(), &[1]);
}

#[test]
fn smaller_outputs_zero_if_equal() {
    // The `<` operator output `0`, if its two inputs are equal.

    let mut eval = Eval::start("0 0 <");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.stack.to_u32_slice(), &[0]);
}

#[test]
fn smaller_outputs_zero_if_larger() {
    // The `<` operator output `0`, if its first outputs is larger then the
    // second, treating both inputs as signed.

    let mut eval = Eval::start("0 -1 <");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.stack.to_u32_slice(), &[0]);
}

#[test]
fn smaller_equals_outputs_one_if_smaller() {
    // The `<=` operator output `1`, if its first input is smaller than the
    // second, treating both inputs as signed.

    let mut eval = Eval::start("-1 0 <=");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.stack.to_u32_slice(), &[1]);
}

#[test]
fn smaller_equals_outputs_one_if_equal() {
    // The `<=` operator output `1`, if its two inputs are equal.

    let mut eval = Eval::start("0 0 <=");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.stack.to_u32_slice(), &[1]);
}

#[test]
fn smaller_equals_outputs_zero_if_larger() {
    // The `<=` operator output `0`, if its first outputs is larger then the
    // second, treating both inputs as signed.

    let mut eval = Eval::start("0 -1 <=");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.stack.to_u32_slice(), &[0]);
}

#[test]
fn equals_outputs_one_if_equal() {
    // The `=` operator outputs `1`, if its two inputs are equal.

    let mut eval = Eval::start("3 3 =");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.stack.to_u32_slice(), &[1]);
}

#[test]
fn equals_outputs_zero_if_not_equal() {
    // The `=` operator outputs `0`, if its two inputs are not equal.

    let mut eval = Eval::start("3 5 =");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.stack.to_u32_slice(), &[0]);
}

#[test]
fn larger_outputs_zero_if_smaller() {
    // The `>` operator output `0`, if its first input is smaller than the
    // second, treating both inputs as signed.

    let mut eval = Eval::start("-1 0 >");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.stack.to_u32_slice(), &[0]);
}

#[test]
fn larger_outputs_zero_if_equal() {
    // The `>` operator output `0`, if its two inputs are equal.

    let mut eval = Eval::start("0 0 >");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.stack.to_u32_slice(), &[0]);
}

#[test]
fn larger_outputs_one_if_larger() {
    // The `>` operator output `1`, if its first outputs is larger then the
    // second, treating both inputs as signed.

    let mut eval = Eval::start("0 -1 >");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.stack.to_u32_slice(), &[1]);
}

#[test]
fn larger_equals_outputs_zero_if_smaller() {
    // The `>=` operator output `0`, if its first input is smaller than the
    // second, treating both inputs as signed.

    let mut eval = Eval::start("-1 0 >=");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.stack.to_u32_slice(), &[0]);
}

#[test]
fn larger_equals_outputs_one_if_equal() {
    // The `>=` operator output `1`, if its two inputs are equal.

    let mut eval = Eval::start("0 0 >=");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.stack.to_u32_slice(), &[1]);
}

#[test]
fn larger_equals_outputs_one_if_larger() {
    // The `>=` operator output `1`, if its first outputs is larger then the
    // second, treating both inputs as signed.

    let mut eval = Eval::start("0 -1 >=");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.stack.to_u32_slice(), &[1]);
}
