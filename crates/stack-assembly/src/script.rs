#[derive(Debug)]
pub struct Script {
    pub operators: Vec<Operator>,
    pub labels: Vec<Label>,
}

impl Script {
    pub fn compile(script: &str) -> Self {
        let mut operators = Vec::new();
        let mut labels = Vec::new();

        for line in script.lines() {
            for token in line.split_whitespace() {
                if token.starts_with("#") {
                    // This is a comment. Ignore the rest of the line.
                    break;
                }

                let operator = if let Some((name, "")) = token.rsplit_once(":")
                {
                    labels.push(Label {
                        name: name.to_string(),
                        operator: OperatorIndex {
                            value: operators.len(),
                        },
                    });
                    continue;
                } else if let Some(("", name)) = token.split_once("@") {
                    Operator::Reference {
                        name: name.to_string(),
                    }
                } else if let Some(("", value)) = token.split_once("0x")
                    && let Ok(value) = i32::from_str_radix(value, 16)
                {
                    Operator::Integer { value }
                } else if let Some(("", value)) = token.split_once("0x")
                    && let Ok(value) = u32::from_str_radix(value, 16)
                {
                    Operator::integer_u32(value)
                } else if let Ok(value) = token.parse::<i32>() {
                    Operator::Integer { value }
                } else if let Ok(value) = token.parse::<u32>() {
                    Operator::integer_u32(value)
                } else {
                    Operator::Identifier {
                        value: token.to_string(),
                    }
                };

                operators.push(operator);
            }
        }

        Self { operators, labels }
    }
}

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

#[derive(Clone, Copy, Debug)]
pub struct OperatorIndex {
    pub value: usize,
}

#[derive(Debug)]
pub struct Label {
    pub name: String,
    pub operator: OperatorIndex,
}
