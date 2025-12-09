use crate::{Effect, Eval, Value};

#[test]
fn read() {
    // `read` reads a word from memory at the given address, pushing it to the
    // stack. It does not modify the memory.

    let mut eval = Eval::start("1 read 1 read");
    eval.memory.values[1] = Value::from(3);
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.stack.to_u32_slice(), &[3, 3]);
}

#[test]
fn read_triggers_effect_on_out_of_bounds_access() {
    // If the address passed to `read` is out of bounds, that triggers the
    // respective effect.

    let mut eval = Eval::start("1025 read");
    assert!(
        eval.memory.values.len() < 1025,
        "Test can't work, because it makes wrong assumption about memory size.",
    );

    eval.run();
    assert_eq!(eval.effect, Some(Effect::InvalidAddress));
    assert_eq!(eval.stack.to_u32_slice(), &[]);
}

#[test]
fn write() {
    // `write` writes a word to memory at the given address.

    let mut eval = Eval::start("1 3 write");
    eval.run();

    assert_eq!(eval.effect, Some(Effect::OutOfOperators));
    assert_eq!(eval.stack.to_u32_slice(), &[]);
    assert_eq!(eval.memory.values[1], Value::from(3));
}

#[test]
fn write_triggers_effect_on_out_of_bounds_access() {
    // If the address passed to `write` is out of bounds, that triggers the
    // respective effect.

    let mut eval = Eval::start("1025 3 write");
    assert!(
        eval.memory.values.len() < 1025,
        "Test can't work, because it makes wrong assumption about memory size.",
    );

    eval.run();
    assert_eq!(eval.effect, Some(Effect::InvalidAddress));
    assert_eq!(eval.stack.to_u32_slice(), &[]);
}
