mod expr;

use combine::parser::char::{spaces, digit, char, letter};
use combine::parser::choice::or;
use combine::parser::range::take_while1;
use combine::{many1, between, sep_by, any, choice, Parser, EasyParser};
use combine::stream::easy;

/*
fn parse_str_list(input: &str) -> Result<Vec<&str>, easy::ParseError<&str>> {
    let tool = take_while1(|c: char| c.is_alphabetic());

    let mut tools = sep_by(tool, range(", "));

    let output = tools.easy_parse(input);
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_str_list() {
        let input = "hello, world, hey12";
        let result = parse_str_list(input);
        assert_eq!(result, Ok(vec!["hello", "world", "hey12"]));
    }
}
*/

fn main() {
    let integer = spaces()
        .with(many1(digit()).map(|string: String| string.parse::<i32>().unwrap()));

    // let string = spaces()
    //     .with(between(char('"'), char('"'), many1(or(letter(), digit()))));

    let mut integer_list = sep_by(
        integer,
        spaces().skip(char(','))
    );

    // let mut string_list = sep_by(
    //     string,
    //     spaces().skip(char(',')),
    // );

    let input = "1234, 45,78";
    let result: Result<(Vec<i32>, &str), easy::ParseError<&str>> =
        integer_list.easy_parse(input);

    // let input: &str = "\"hello\", \"world\", \"hey12\"";
    // let result: Result<(Vec<String>, &str), easy::ParseError<&str>> =
    //     string_list.easy_parse(input);


    // let result: Result<(Vec<i32>, &str), easy::ParseError<&str>> =
    //     string_list.easy_parse(input);

    match result {
        Ok((value, _remaining_input)) => println!("{:?}", value),
        Err(err) => println!("{}", err),
    }
}
