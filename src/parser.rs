mod chars {
    pub const ALPHA: &str = "qazwsxedcrfvtgbyhnujmikolpQAZWSXEDCRFVTGBYHNUJMIKOLP";
    pub const NUMERIC: &str = "1234567890";
    pub const LINEAR: &str = "'\"";
    pub const OPERATOR: &str = "`~!@#$%^&*-_=+|\\;:,<.>/?";
    pub const OPEN_SCOPE: &str = "([{";
    pub const CLOSE_SCOPE: &str = ")]}";
    pub const WHITESPACE: &str = " \n\r";
}

#[derive(Debug)]
enum CharType {
    Alpha {
        value: char
    },
    Numeric {
        value: char
    },
    Operator {
        value: char,
    },
    Linear {
        variant: LiteralVariant,
    },
    OpenScope {
        variant: ScopeVariant,
    },
    CloseScope {
        variant: ScopeVariant,
    },
    Whitespace,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralVariant {
    Double,
    Single
}

#[derive(Clone, Debug, PartialEq)]
pub enum ScopeVariant {
    Squared,
    Curly,
    Rounded,
}

#[derive(Clone, Debug)]
pub enum Token {
    Literal {
        variant: LiteralVariant,
        value: String
    },

    Number {
        value: String
    },

    Id {
        value: String
    },

    Operator {
        value: String
    },

    OpenScope {
        variant: ScopeVariant
    },

    CloseScope {
        variant: ScopeVariant
    },
}

fn get_char_type(character: char) -> CharType {
    if chars::ALPHA.contains(character) {
        CharType::Alpha {
            value: character
        }
    } else if chars::NUMERIC.contains(character) {
        CharType::Numeric {
            value: character
        }
    } else if chars::OPERATOR.contains(character){
        CharType::Operator {
            value: character
        }
    } else if chars::LINEAR.contains(character) {
        CharType::Linear {
            variant: match character {
                '\'' => LiteralVariant::Single,
                '"' => LiteralVariant::Double,
                _ => panic!("invalid linear char")
            }
        }
    } else if chars::OPEN_SCOPE.contains(character) {
        CharType::OpenScope {
            variant: match character {
                '(' => ScopeVariant::Rounded,
                '[' => ScopeVariant::Squared,
                '{' => ScopeVariant::Curly,
                _ => panic!("invalid scope char")
            }
        }
    } else if chars::CLOSE_SCOPE.contains(character) {
        CharType::CloseScope {
            variant: match character {
                ')' => ScopeVariant::Rounded,
                ']' => ScopeVariant::Squared,
                '}' => ScopeVariant::Curly,
                _ => panic!("invalid close scope char")
            }
        }
    } else if chars::WHITESPACE.contains(character) {
        CharType::Whitespace
    } else {
        panic!("Invalid character");
    }
}

pub fn tokenize(input: String) -> Vec<Token> {
    let mut output: Vec<Token> = vec![];
    let mut token_option: Option<Token> = None;
    let mut character_id = 0;

    let input_characters: Vec<char> = input.chars().collect();

    loop {
        macro_rules! end_token {
            ($token:ident) => {
                output.push($token.clone());
                token_option = None;
                character_id -= 1;
            };

            ($token:ident, $go_back:expr) => {
                output.push($token.clone());
                token_option = None;
                if $go_back {
                    character_id -= 1;
                }
            };
        }

        if character_id >= input_characters.len() {
            break;
        }

        let character = input_characters[character_id];
        character_id += 1;
        
        let char_type = get_char_type(character);
        
        match &mut token_option {
            Some(token) => {
                match token {
                    Token::Id { value } => {
                        match char_type {
                            CharType::Alpha {value: character} => {
                                value.push(character);
                            },

                            CharType::Numeric {value: character} => {
                                value.push(character);
                            },

                            _ => {
                                end_token!(token);
                            }
                        }
                    },

                    Token::Number { value } => {
                        match char_type {
                            CharType::Numeric {value: character} => {
                                value.push(character);
                            },

                            CharType::Operator {value: character} => {
                                if character == '.' {
                                    value.push(character);

                                } else {
                                    end_token!(token);
                                }
                            },

                            CharType::Linear {variant: _} => {
                                end_token!(token);
                            },

                            _ => {
                                end_token!(token);
                            }
                        }
                    },

                    Token::OpenScope { variant: _ } => {
                        end_token!(token);
                    },

                    Token::CloseScope { variant: _ } => {
                        end_token!(token);
                    },

                    Token::Literal { value, variant: open_variant } => {
                        match char_type {
                            CharType::Linear {variant: close_variant} if &close_variant == open_variant => {
                                end_token!(token, false);
                            },

                            _ => {
                                value.push(character);
                            }
                        }
                    },

                    Token::Operator { value } => {
                        match char_type {
                            CharType::Operator {value: character} => {
                                value.push(character);
                            },

                            _ => {
                                end_token!(token);
                            }
                        }
                    }
                }
            },

            None => {
                match char_type {
                    CharType::Alpha {value: character} => {
                        token_option = Some(Token::Id {
                            value: String::from(character)
                        });
                    },
        
                    CharType::Numeric {value: character} => {
                        token_option = Some(Token::Number {
                            value: String::from(character)
                        });
                    },
        
                    CharType::Operator {value: character} => {
                        token_option = Some(Token::Operator {
                            value: String::from(character)
                        });
                    },

                    CharType::Linear {variant} => {
                        token_option = Some(Token::Literal {
                            value: String::default(),
                            variant
                        });
                    },

                    CharType::OpenScope {variant} => {
                        token_option = Some(Token::OpenScope {
                            variant
                        });
                    },

                    CharType::CloseScope {variant} => {
                        token_option = Some(Token::CloseScope {
                            variant
                        });
                    },

                    _ => {}
                }
            }
        }
    }

    match token_option {
        Some(token) => output.push(token),
        _ => {}
    };

    output
}