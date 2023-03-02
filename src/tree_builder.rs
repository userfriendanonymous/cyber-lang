use crate::parser::{Token, ScopeVariant, LiteralVariant};

#[derive(Clone, Debug)]
struct Operator {
    pub value: String
}

#[derive(Clone, Debug)]
pub enum Node {
    Number {
        value: String
    },

    Literal {
        variant: LiteralVariant,
        value: String
    },

    Id {
        value: String
    },

    Scope {
        variant: ScopeVariant,
        left: Box<Node>,
        operator: Operator,
        right: Box<Node>
    },

    None
}

fn token_to_node(token: &Token) -> Node {
    match token {
        Token::Id { value } => Node::Id { value: value.clone() },
        Token::Literal { variant, value } => Node::Literal { variant: variant.clone(), value: value.clone() },
        Token::Number { value } => Node::Number { value: value.clone() },
        _ => Node::None
    }
}

pub fn build(tokens: &[Token]) -> Node {
    let mut node: Node = Node::None;

    let mut token_id = tokens.len();

    loop {
        if token_id <= 0 {
            break;
        }
        token_id -= 1;
        let token = &tokens[token_id];

        match token {
            Token::Operator { value } => {

                node = Node::Scope {
                    variant: ScopeVariant::Rounded,
                    left: Box::new(node.clone()),
                    right: Box::new(build(&tokens[0..token_id])),
                    operator: Operator {
                        value: value.clone()
                    }
                };
                token_id = 0;
            },

            Token::OpenScope { variant } => {
                panic!("unclosed scope!");
            },

            Token::CloseScope { variant } => {
                let close_scope_id = token_id;
                let mut scope_count = 1;

                loop {
                    token_id -= 1;
                    let token = &tokens[token_id];
                    
                    match token {
                        Token::CloseScope { variant } => {
                            scope_count += 1;
                        },

                        Token::OpenScope { variant: open_variant } => {
                            scope_count -= 1;
                            if scope_count == 0 {
                                if open_variant == variant {
                                    break
                                } else {
                                    panic!("Invalid scope closure!");
                                }
                            }
                        }, 

                        _ => {}
                    }
                }

                node = build(&tokens[token_id..close_scope_id]);
            },

            _ => {
                if let Node::None = node {
                    node = token_to_node(token);
                } else {
                    panic!("two or more non-operator tokens at the same place!");
                }
            }
        }
    }

    node
}