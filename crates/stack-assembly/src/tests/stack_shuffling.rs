use crate::{Effect, Eval};

#[test]
fn copy() {
    // The `copy` operator duplicates any value on the stack, placing a copy at
    // the top.

    let mut eval = Eval::start("3 5 8 1 copy");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.operand_stack.to_u32_slice(), &[3, 5, 8, 5]);
}

#[test]
fn copy_trigger_effect_on_invalid_index() {
    // If an invalid index is passed to `copy`, which does not refer to a value
    // on the stack, this triggers an effect.

    let mut eval = Eval::start("0 copy");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::InvalidStackIndex));
    assert_eq!(eval.operand_stack.to_u32_slice(), &[]);
}

#[test]
fn drop() {
    // The `drop` operator removes any value from the stack.

    let mut eval = Eval::start("3 5 8 1 drop");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.operand_stack.to_u32_slice(), &[3, 8]);
}
