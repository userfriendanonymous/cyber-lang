use super::ident::{self, Instance as Ident};

pub struct Instance {
    pub parts: Box<[Ident]>,
    pub name: Ident,
    pub case: ident::Case,
}

impl Instance {
    pub fn from_char_prefix<'a>(mut value: &'a [char]) -> Option<(Self, &'a [char])> {
        let mut parts = Vec::new();
        let mut case = ident::Case::Snake;

        loop {
            if let Some((part, next_value)) = Ident::from_chars_snake_case_prefix(value) {
                value = next_value;
                parts.push(part);

            } else if let Some((part, next_value)) = Ident::from_chars_pascal_case_prefix(value) {
                value = next_value;
                parts.push(part);
                case = ident::Case::Pascal;

            } else {
                break;
            }
            
            if let Some(next_value) = value.strip_prefix(&[':', ':']) {
                value = next_value;
            } else {
                break;
            }
        }

        let name = parts.pop()?;

        Some((Self {
            case,
            name,
            parts: parts.into(),
        }, value))
    }
}
