use std::collections::HashMap;
use ident::Instance as Ident;
use path::Instance as Path;

mod ident;
mod path;
mod number;

pub struct Module {
    functions: HashMap<Ident, Function>,
    // types: HashMap<Ident, Type>,
    modules: HashMap<Ident, Module>
}

pub enum EnvIdent {
    Sub(Ident),
    Super,
}


pub struct Function {
    is_external: bool,
    expression: Expression
}

pub enum Expression {
    Mul {
        path: Path,
        fields: HashMap<Ident, Expression>
    },
    Sum {
        path: Path,
        tag: Ident,
        body: Box<Expression>,
    },
    Call {
        path: Path,
        input: Box<Expression>,
    },
    Match {
        on: Box<Expression>,
        variants: HashMap<Ident, Expression>,
    },
    Member {
        of: Ident,
        name: Ident,
    },
    Variable {
        name: Ident
    }
}

pub enum ExpressionFromCharsPrefixError {
    CloseBracketExpected,
    UnexpectedPrefix,
    MulField(Box<MulFieldFromCharsPrefixError>),
    SumTagNameExpected,
    SumValue(Box<Self>),
    UnexpectedTypeSuffix
}

impl Expression {
    pub fn from_chars_prefix<'a>(value: &'a [char]) -> Result<(Self, &'a [char]), ExpressionFromCharsPrefixError> {
        type E = ExpressionFromCharsPrefixError;

        if let Some((path, value)) = Path::from_char_prefix(value) {
            match path.case {
                ident::Case::Snake => {
                    if path.parts.is_empty() {
                        let value = strip_space(value);
                        match Self::from_chars_prefix(value) {
                            Ok((input, value)) => {
                                Ok((Self::Call { path, input: Box::new(input) }, value))
                            },

                            Err(e) => if let E::UnexpectedPrefix = e {
                                Ok((Self::Variable { name: path.name }, value))
                            } else {
                                Err(e)
                            }
                        }

                    } else {
                        let value = strip_space(value);
                        
                        match Self::from_chars_prefix(value) {
                            Ok((input, value)) => {
                                Ok((Self::Call { path, input: Box::new(input) }, value))
                            },

                            Err(e) => Err(e)
                        }
                    }
                },
                ident::Case::Pascal => {
                    let value = strip_space(value);
                    if let Some(value) = value.strip_prefix(&[':']) {
                        let value = strip_space(value);
                        let (tag, value) = Ident::from_chars_pascal_case_prefix(value).ok_or(E::SumTagNameExpected)?;
                        let value = strip_space(value);
                        let (body, value) = Self::from_chars_prefix(value).map_err(|e| E::SumValue(Box::new(e)))?;

                        Ok((Self::Sum { path, tag, body: Box::new(body) }, value))

                    } else if let Some(mut value) = value.strip_prefix(&['{']) {
                        let mut fields = HashMap::new();

                        loop {
                            value = strip_space(value);
                            if let Some(next_value) = value.strip_prefix(&['}']) {
                                value = next_value;
                                break;

                            } else {
                                let (name, expression, next_value) = mul_field_from_chars_prefix(value).map_err(|e: MulFieldFromCharsPrefixError| E::MulField(Box::new(e)))?;
                                value = next_value;
                                fields.insert(name, expression);
                            }
                        }

                        Ok((Self::Mul {
                            fields,
                            path
                        }, value))
                    } else {
                        Err(E::UnexpectedTypeSuffix)
                    }
                }
            }
        } else if let Some(value) = value.strip_prefix(&['(']) {
            let (expression, value) = Expression::from_chars_prefix(value)?;
            let value = strip_space(value);
            let value = value.strip_prefix(&[')']).ok_or(E::CloseBracketExpected)?;

            Ok((expression, value))

        } else {
            Err(E::UnexpectedPrefix)?
        }
    }
}

pub enum MulFieldFromCharsPrefixError {
    NameExpected,
    EqualSignExpected,
    ExpressionExpected(ExpressionFromCharsPrefixError),
}

fn mul_field_from_chars_prefix<'a>(value: &'a [char]) -> Result<(Ident, Expression, &'a [char]), MulFieldFromCharsPrefixError> {
    type E = MulFieldFromCharsPrefixError;
    let (name, value) = Ident::from_chars_pascal_case_prefix(value).ok_or(E::NameExpected)?;
    let value = strip_space(value);
    let value = value.strip_prefix(&['=']).ok_or(E::EqualSignExpected)?;
    let value = strip_space(value);
    let (expression, value) = Expression::from_chars_prefix(value).map_err(E::ExpressionExpected)?;

    let value = value.strip_prefix(&[',']).unwrap_or(value);

    Ok((name, expression, value))
}

pub enum Pattern {
    Equals(Expression),
    Tag(Ident, Box<Self>),
    Fields(HashMap<Ident, Self>),
    Any(Ident),
}

pub enum PatternFromCharsPrefixError {
    UnknownPrefix,
}

impl Pattern {
    pub fn try_from_chars_prefix<'a>(value: &'a [char]) -> Result<(Self, &'a [char]), PatternFromCharsPrefixError> {
        type E = PatternFromCharsPrefixError;

        if let Some((name, value)) = Ident::from_chars_pascal_case_prefix(value) {
            match Self::try_from_chars_prefix(value) {
                Ok((body, value)) => {
                    Ok((Self::Tag(name, Box::new(body)), value))
                },
                Err(err) => if let PatternFromCharsPrefixError::UnknownPrefix = err {
                    Ok((Self::Any(name), value))
                } else {
                    Err(err)
                }
            }

        } else if let Some(value) = value.strip_prefix(&['{']) {
            
        } else {
            Err(E::UnknownPrefix)
        }
    }
}

pub enum Init {
    Struct {
        path: Path,
        fields: HashMap<Ident, Expression>,
    },
}

pub enum PathPart {
    Super,
    Module(Ident)
}

type PathBase = Vec<PathPart>;

fn normalize_path_base(value: PathBase) -> Vec<Ident> {
    let types: HashMap<usize, Type> = HashMap::new();
    let modules: HashMap<usize, Module> = HashMap::new();
    let functions: HashMap<usize, Function> = HashMap::new();

    let mut parts = Vec::new();
    for part in value {
        match part {
            PathPart::Super => { parts.pop(); },
            PathPart::Module(name) => parts.push(name)
        }
    }
    parts
}

pub enum Type {
    Path(Path),

}

pub enum TypeItem {
    Struct(Struct)
}

fn strip_space<'a>(value: &'a [char]) -> &'a [char] {
    let mut idx = 0;
    for &item in value.iter() {
        if " \n".contains(item) {
            idx += 1;
        } else {
            break
        }
    }

    &value[idx..]
}

pub enum Item {
    Function(Function),
    Module(Module),
}

pub enum ItemFromCharsPrefixError {
    UnknownPrefix,
    FnNameExpected,
    OpenCurlyBraceExpected,
    CloseCurlyBraceExpected,
    FnExpression(ExpressionFromCharsPrefixError)
}

impl Item {
    pub fn try_from_chars_prefix<'a>(value: &'a [char]) -> Result<(Ident, Self, &'a [char]), ItemFromCharsPrefixError> {
        type E = ItemFromCharsPrefixError;

        if let Some(value) = value.strip_prefix(&['f', 'n']) {
            let value = strip_space(value);
            let (name, value) = ident::Instance::from_chars_snake_case_prefix(value).ok_or(E::FnNameExpected)?;
            let value = strip_space(value);
            let value = value.strip_prefix(&['{']).ok_or(E::OpenCurlyBraceExpected)?;
            let value = strip_space(value);

            let (expression, value) = Expression::from_chars_prefix(value).map_err(E::FnExpression)?;
            let value = strip_space(value);
            let value = value.strip_prefix(&['}']).ok_or(E::CloseCurlyBraceExpected)?;

            Ok((name, Self::Function(Function {
                expression,
                is_external: false,
            }), value))

        } else if let Some(value) = value.strip_prefix(&['m', 'o', 'd']) {
            let value = strip_space(value);
            let (name, value) = ident::Instance::from_chars_snake_case_prefix(value).ok_or(E::FnNameExpected)?;
            let value = strip_space(value);
            let mut value = value.strip_prefix(&['{']).ok_or(E::OpenCurlyBraceExpected)?;

            let mut functions = HashMap::new();
            let mut modules = HashMap::new();

            loop {
                value = strip_space(value);

                if let Some(next_value) = value.strip_prefix(&['}']) {
                    value = next_value;
                    break;

                } else {
                    let (name, item, next_value) = Self::try_from_chars_prefix(value)?;
                    value = next_value;

                    match item {
                        Item::Function(function) => {
                            functions.insert(name, function);
                        }
                        Item::Module(module) => {
                            modules.insert(name, module);
                        }
                    }
                }
            }

            Ok((name, Self::Module(Module {
                functions,
                modules,
            }), value))

        } else {
            Err(E::UnknownPrefix)
        }
    }
}

pub struct Struct {
    fields: HashMap<Ident, Type>
}
