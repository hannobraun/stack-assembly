#[derive(Debug)]
pub enum Operator {
    Identifier { value: String },
    Integer { value: i32 },
    Reference { name: String },
}

impl Operator {
    pub fn integer_u32(value: u32) -> Self {
        Self::Integer {
            value: i32::from_le_bytes(value.to_le_bytes()),
        }
    }
}

#[derive(Debug)]
pub struct Label {
    pub name: String,
    pub operator: usize,
}
