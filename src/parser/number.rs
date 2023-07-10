
pub struct Instance {
    first: Digit,
    others: Box<[Digit]>
}

impl Instance {
    pub fn from_chars_prefix<'a>(value: &'a [char]) -> Option<(Self, &'a [char])> {
        let (&first, mut value) = value.split_first()?;
        let first = Digit::from_char(first)?;
        let mut others = Vec::new();

        while let Some((&item, next_value)) = value.split_first() {
            value = next_value;
            if let Some(digit) = Digit::from_char(item) {
                others.push(digit);
            } else {
                break;
            }
        }

        Some((Self {
            first,
            others: others.into()
        }, value))
    }
}

pub enum Digit {
    Zero,
    One, Two, Three,
    Four, Five, Six,
    Seven, Eight, Nine
}

impl Digit {
    pub fn from_char(value: char) -> Option<Digit> {
        Some(match value {
            '0' => Self::Zero,
            '1' => Self::One,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight, 
            '9' => Self::Nine,
            _ => None?
        })
    }
}
