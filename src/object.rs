#[derive(Debug, Clone, PartialEq)]
pub enum Obj {
    String(Vec<char>),
}

pub fn copy_string(chars: &[char]) -> Obj {
    let mut heap_chars = chars.to_vec();
    heap_chars.push('\0');

    Obj::String(heap_chars)
}

impl Obj {
    pub fn is_string(&self) -> bool {
        match self {
            Obj::String(_) => true,
            _ => false,
        }
    }
}
