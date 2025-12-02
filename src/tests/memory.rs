use crate::{Effect, Eval, Value};

#[test]
fn read() {
    // `read` reads a word from memory at the given address, pushing it to the
    // stack. It does not modify the memory.

    let mut eval = Eval::start("1 read 1 read");
    eval.memory[1] = Value::from(3);
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfTokens));
    assert_eq!(eval.stack.to_u32_slice(), &[3, 3]);
}
