use chumsky::prelude::*;

#[derive(Debug)]
pub struct Values;

pub fn parser<'a>() -> impl Parser<'a, &'a str, Values, extra::Err<Simple<'a, char>>> {
    let comments = just("//").ignored().repeated();

    any()
        .ignored()
        .padded_by(comments.repeated())
        .repeated()
        .collect::<Vec<_>>()
        .map(|_| Values)
}

fn main() {
    let src = "Would you rather have a CurseForge or a hex editor?";

    println!("{src}");

    println!("Creating parser...");
    let parser = parser().then_ignore(end());
    println!("Parsing file...");
    let result = parser.parse(src);
    println!("Done!");

    if let Some(value) = result.output() {
        println!("{value:?}");
    }
    for error in result.errors() {
        println!("{error}");
    }
}
