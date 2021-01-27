#[macro_use]
extern crate lazy_static;

use strum::IntoEnumIterator;
use crate::token::TokenType;

mod lexer;
mod token;
mod token_regex;

fn main() {
    for variant in TokenType::iter()
    {
        println!("{:?} = {}", variant, variant.str_repr());
    }
}
