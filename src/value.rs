use crate::object::Obj;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Value {
    #[default]
    Nil,
    Bool(bool),
    Number(f64),
    Obj(Obj),
}

impl Value {
    pub fn is_number(&self) -> bool {
        match self {
            Value::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Value::Obj(obj) => obj.is_string(),
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
