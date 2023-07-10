
use std::collections::HashMap;
use super::parser;

pub struct Scope {
    data: Box<[Value]>,
    expression: Expression,
}

pub enum Expression {
    Match {
        on: usize,
        patterns: Box<[Pattern]>,

    }
}

pub enum Pattern {
    Eq(Value),
    Variant {
        name: String,
        value: Box<Pattern>,
    },
    Fields(HashMap<String, Pattern>),

}

pub enum Value {
    Sum {
        id: usize,
        variant: String,
        value: Box<Value>,
    },

    Mul {
        id: usize,
        fields: HashMap<String, Value>,
    },
}
