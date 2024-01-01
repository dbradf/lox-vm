#[derive(Debug, Clone, Default, PartialEq)]
pub enum Value {
    Bool(bool),

    #[default]
    Nil,
    Number(f64),
}

impl Value {
    pub fn is_number(&self) -> bool {
        match self {
            Value::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_falsy(&self) -> bool {
        match self {
            Value::Nil => true,
            Value::Bool(b) => !b,
            _ => false,
        }
    }

    pub fn equals(&self, other: &Self) -> bool {
        self == other
    }
}
