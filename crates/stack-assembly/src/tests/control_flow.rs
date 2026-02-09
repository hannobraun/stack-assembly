use crate::{Effect, Eval, Script};

#[test]
fn jump() {
    // The `jump` operator takes the index of an operator (usually provided by
    // a reference) as input, then arranges for evaluation to continue at that
    // operator.

    let script = Script::compile("start: 1 yield @start jump");

    let mut eval = Eval::new();

    let (effect, _) = eval.run(&script);
    assert_eq!(effect, Effect::Yield);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[1]);

    eval.clear_effect();

    let (effect, _) = eval.run(&script);
    assert_eq!(effect, Effect::Yield);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[1, 1]);
}

#[test]
fn jump_if_behaves_like_jump_on_nonzero_condition() {
    // The `jump_if` operator is the conditional variant of `jump`. In addition
    // to an operator index, it takes a condition. If that condition is
    // non-zero, `jump_if` behaves like `jump`.

    let script = Script::compile("1 @target jump_if 1 target: 2");

    let mut eval = Eval::new();
    let (effect, _) = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[2]);
}

#[test]
fn jump_if_does_nothing_on_zero_condition() {
    // The `jump_if` operator is the conditional variant of `jump`. In addition
    // to an operator index, it takes a condition. If that condition is zero,
    // `jump_if` does nothing.

    let script = Script::compile("0 @target jump_if 1 target: 2");

    let mut eval = Eval::new();
    let (effect, _) = eval.run(&script);

    assert_eq!(effect, Effect::OutOfOperators);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[1, 2]);
}

#[test]
fn return_() {
    // If the call stack is empty, as is the case when the evaluation starts,
    // the `return` operator triggers an effect.

    let script = Script::compile("return");

    let mut eval = Eval::new();
    let (effect, _) = eval.run(&script);

    assert_eq!(effect, Effect::Return);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[]);
}

#[test]
fn call_return() {
    // The `call` operator takes the index of an operator (usually provided by
    // a reference) as input, arranges for evaluation to continue at that
    // operator, and pushes a return address to the call stack. `return` pops an
    // address from the call stack and arranges for evaluation to continue
    // there.

    let script = Script::compile(
        "
        1
        @2 call
        3
        return

        2:
            2
            return
        ",
    );

    let mut eval = Eval::new();
    let (effect, _) = eval.run(&script);

    assert_eq!(effect, Effect::Return);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[1, 2, 3]);
}

#[test]
fn call_either_jumps_to_first_index_on_non_zero_condition() {
    // `call_either` is the conditional variant of `call`. It takes a condition
    // and the indices of two operators. If the condition is non-zero, it
    // arranges for evaluation to continue at the first operator.

    let script = Script::compile(
        "
        1 @then @else call_either
        return

        then:
            1
            return
        else:
            2
            return
        ",
    );

    let mut eval = Eval::new();
    let (effect, _) = eval.run(&script);

    assert_eq!(effect, Effect::Return);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[1]);
}

#[test]
fn call_either_jumps_to_second_index_on_non_zero_condition() {
    // `call_either` is the conditional variant of `call`. It takes a condition
    // and the indices of two operators. If the condition is non-zero, it
    // arranges for evaluation to continue at the first operator.

    let script = Script::compile(
        "
        0 @then @else call_either
        return

        then:
            1
            return
        else:
            2
            return
        ",
    );

    let mut eval = Eval::new();
    let (effect, _) = eval.run(&script);

    assert_eq!(effect, Effect::Return);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[2]);
}

#[test]
fn invalid_reference_triggers_effect() {
    // A reference that is not paired with a matching label can't return a
    // sensible value and must trigger an effect.

    let script = Script::compile("@invalid");

    let mut eval = Eval::new();
    let (effect, _) = eval.run(&script);

    assert_eq!(effect, Effect::InvalidReference);
    assert_eq!(eval.operand_stack.to_u32_slice(), &[]);
}
