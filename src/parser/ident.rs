
#[derive(PartialEq, Eq, Hash)]
pub struct Instance {
    first: Part,
    others: Box<[Part]>
}

impl Instance {
    pub fn from_chars_snake_case_prefix<'a>(mut value: &'a [char]) -> Option<(Self, &'a [char])> {
        let (first, mut value) = Part::from_chars_lowercase_prefix(value)?;
        let mut others = Vec::new();

        while let Some((&item, next_value)) = value.split_first() {
            value = next_value;
            if item != '_' { None? }

            let (part, next_value) = Part::from_chars_lowercase_prefix(value)?;
            value = next_value;
            others.push(part);
        }

        Some((Self {
            first,
            others: others.into()
        }, value))
    }

    pub fn from_chars_pascal_case_prefix<'a>(mut value: &'a [char]) -> Option<(Self, &'a [char])> {
        let (first, mut value) = Part::from_chars_pascal_case_prefix(value)?;
        let mut others = Vec::new();

        while let Some((part, next_value)) = Part::from_chars_pascal_case_prefix(value) {
            value = next_value;
            others.push(part);
        }

        Some((Self {
            first,
            others: others.into()
        }, value))
    }

    pub fn from_chars_prefix<'a>(value: &'a [char]) -> Option<(Self, Case, &'a [char])> {
        if let Some((instance, value)) = Self::from_chars_pascal_case_prefix(value) {
            Some((instance, Case::Snake, value))
        } else if let Some((instance, value)) = Self::from_chars_pascal_case_prefix(value) {
            Some((instance, Case::Pascal, value))
        } else {
            None
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct Part {
    first: Char,
    others: Box<[Char]>,
}

impl Part {
    pub fn from_chars_lowercase_prefix<'a>(value: &'a [char]) -> Option<(Self, &'a [char])> {
        let (&item, mut value) = value.split_first()?;
        let first = Char::from_lowercase_char(item)?;
        let mut others = Vec::new();

        while let Some(item) = value.get(0) {
            if let Some(part) = Char::from_lowercase_char(*item) {
                others.push(part);
                value = &value[1..];
            } else {
                break
            }
        }

        Some((Self {
            first,
            others: others.into()
        }, value))
    }

    pub fn from_chars_pascal_case_prefix<'a>(mut value: &'a [char]) -> Option<(Self, &'a [char])> {
        let (&item, mut value) = value.split_first()?;
        if !item.is_ascii_uppercase() { None? }
        let first = Char::from_char(item)?;
        let mut others = Vec::new();

        while let Some(item) = value.get(0) {
            if let Some(part) = Char::from_lowercase_char(*item) {
                others.push(part);
                value = &value[1..];
            } else {
                break
            }
        }

        Some((Self {
            first,
            others: others.into()
        }, value))
    }
}

pub enum Case {
    Snake,
    Pascal,
}

impl Case {
    pub fn is_snake(&self) -> bool {
        matches!(self, Self::Snake)
    }

    pub fn is_pascal(&self) -> bool {
        matches!(self, Self::Pascal)
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum Char {
    Q, W, E, R, T, Y, U, I, O, P,
    A, S, D, F, G, H, J, K, L,
    Z, X, C, V, B, N, M,
    One, Two, Three,
}

impl Char {
    pub fn from_lowercase_char(value: char) -> Option<Self> {
        Some(match value {
            'q' => Self::Q,
            'w' => Self::W,
            'e' => Self::E,
            'r' => Self::R,
            't' => Self::T,
            'y' => Self::Y,
            'u' => Self::U,
            'i' => Self::I,
            'o' => Self::O,
            'p' => Self::P,
            '1' => Self::One,
            '2' => Self::Two,
            '3' => Self::Three,
            _ => None?
        })
    }

    pub fn from_char(value: char) -> Option<Self> {
        Self::from_lowercase_char(value.to_ascii_lowercase())
    }
}
