use crate::{Effect, Eval};

#[test]
fn copy() {
    // The `copy` operator duplicates any value on the stack, placing a copy at
    // the top.

    let mut eval = Eval::start("3 5 8 1 copy");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfTokens));
    assert_eq!(eval.stack.to_u32_slice(), &[3, 5, 8, 5]);
}
