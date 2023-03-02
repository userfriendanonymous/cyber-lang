
mod parser;
mod tree_builder;
fn main() {
    let tokens = parser::tokenize(String::from("5+5 = 10 ` 'hello world'"));
    dbg!(&tokens);
    let result = tree_builder::build(&tokens[..]);
    dbg!(result);
}