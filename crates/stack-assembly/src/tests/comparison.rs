use crate::{Effect, Eval, Script};

#[test]
fn smaller_outputs_one_if_smaller() {
    // The `<` operator outputs `1`, if its first input is smaller than the
    // second, treating both inputs as signed.

    let script = Script::compile("-1 0 <");

    let mut eval = Eval::new();
    let effect = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[1]);
}

#[test]
fn smaller_outputs_zero_if_equal() {
    // The `<` operator outputs `0`, if its two inputs are equal.

    let script = Script::compile("0 0 <");

    let mut eval = Eval::new();
    let effect = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[0]);
}

#[test]
fn smaller_outputs_zero_if_larger() {
    // The `<` operator outputs `0`, if its first outputs is larger then the
    // second, treating both inputs as signed.

    let script = Script::compile("0 -1 <");

    let mut eval = Eval::new();
    let effect = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[0]);
}

#[test]
fn smaller_equals_outputs_one_if_smaller() {
    // The `<=` operator outputs `1`, if its first input is smaller than the
    // second, treating both inputs as signed.

    let script = Script::compile("-1 0 <=");

    let mut eval = Eval::new();
    let effect = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[1]);
}

#[test]
fn smaller_equals_outputs_one_if_equal() {
    // The `<=` operator outputs `1`, if its two inputs are equal.

    let script = Script::compile("0 0 <=");

    let mut eval = Eval::new();
    let effect = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[1]);
}

#[test]
fn smaller_equals_outputs_zero_if_larger() {
    // The `<=` operator outputs `0`, if its first outputs is larger then the
    // second, treating both inputs as signed.

    let script = Script::compile("0 -1 <=");

    let mut eval = Eval::new();
    let effect = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[0]);
}

#[test]
fn equals_outputs_one_if_equal() {
    // The `=` operator outputs `1`, if its two inputs are equal.

    let script = Script::compile("3 3 =");

    let mut eval = Eval::new();
    let effect = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[1]);
}

#[test]
fn equals_outputs_zero_if_not_equal() {
    // The `=` operator outputs `0`, if its two inputs are not equal.

    let script = Script::compile("3 5 =");

    let mut eval = Eval::new();
    let effect = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[0]);
}

#[test]
fn larger_outputs_zero_if_smaller() {
    // The `>` operator outputs `0`, if its first input is smaller than the
    // second, treating both inputs as signed.

    let script = Script::compile("-1 0 >");

    let mut eval = Eval::new();
    let effect = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[0]);
}

#[test]
fn larger_outputs_zero_if_equal() {
    // The `>` operator outputs `0`, if its two inputs are equal.

    let script = Script::compile("0 0 >");

    let mut eval = Eval::new();
    let effect = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[0]);
}

#[test]
fn larger_outputs_one_if_larger() {
    // The `>` operator outputs `1`, if its first outputs is larger then the
    // second, treating both inputs as signed.

    let script = Script::compile("0 -1 >");

    let mut eval = Eval::new();
    let effect = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[1]);
}

#[test]
fn larger_equals_outputs_zero_if_smaller() {
    // The `>=` operator outputs `0`, if its first input is smaller than the
    // second, treating both inputs as signed.

    let script = Script::compile("-1 0 >=");

    let mut eval = Eval::new();
    let effect = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[0]);
}

#[test]
fn larger_equals_outputs_one_if_equal() {
    // The `>=` operator outputs `1`, if its two inputs are equal.

    let script = Script::compile("0 0 >=");

    let mut eval = Eval::new();
    let effect = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[1]);
}

#[test]
fn larger_equals_outputs_one_if_larger() {
    // The `>=` operator outputs `1`, if its first outputs is larger then the
    // second, treating both inputs as signed.

    let script = Script::compile("0 -1 >=");

    let mut eval = Eval::new();
    let effect = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[1]);
}
