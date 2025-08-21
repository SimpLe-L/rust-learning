use nom::{
    IResult, Parser,
    branch::alt,
    bytes::tag,
    character::complete::{char as char_parser, multispace0, none_of},
    combinator::{map, opt, value},
    multi::{many0, separated_list0},
    number::complete::recognize_float,
    sequence::{delimited, separated_pair},
};
use serde_json::Value;

fn main() {
    let input = r#"
    {
        "name": "John Doe",
        "age": 30,
        "address": {
            "street": "123 Main St",
            "city": "Anytown",
            "state": "CA"
        },
        "phone": [
            "+1-555-1234",
            "+1-555-5678"
        ]
    }"#;

    let result = parse_parimary(input);
    assert!(result.is_ok());
    let (_, value) = result.unwrap();

    println!("解析结果: {:#?}", value);
}

fn parse_null(input: &str) -> IResult<&str, Value> {
    value(
        Value::Null,
        delimited(multispace0, tag("null"), multispace0),
    )
    .parse(input)
}

fn parse_boolean(input: &str) -> IResult<&str, Value> {
    delimited(
        multispace0,
        alt((
            value(Value::Bool(false), tag("false")),
            value(Value::Bool(true), tag("true")),
        )),
        multispace0,
    )
    .parse(input)
}

fn parse_number(input: &str) -> IResult<&str, Value> {
    let (input, number) = delimited(multispace0, recognize_float, multispace0).parse(input)?;

    Ok((input, Value::Number(number.parse().unwrap())))
}

fn parse_string(input: &str) -> IResult<&str, Value> {
    map(
        delimited(multispace0, recognize_string, multispace0),
        |string| Value::String(string),
    )
    .parse(input)
}

fn recognize_string(input: &str) -> IResult<&str, String> {
    map(
        delimited(char_parser('"'), many0(none_of("\"")), char_parser('"')),
        |chars| chars.into_iter().collect(),
    )
    .parse(input)
}

fn parse_array(input: &str) -> IResult<&str, Value> {
    delimited(
        (char_parser('['), opt(multispace0)),
        map(
            separated_list0(
                delimited(multispace0, char_parser(','), multispace0),
                parse_parimary,
            ),
            |values| Value::Array(values),
        ),
        (char_parser(']'), opt(multispace0)),
    )
    .parse(input)
}

fn parse_object(input: &str) -> IResult<&str, Value> {
    delimited(
        multispace0,
        delimited(
            (char_parser('{'), opt(multispace0)),
            map(
                separated_list0(
                    delimited(multispace0, char_parser(','), multispace0),
                    separated_pair(
                        recognize_string,
                        delimited(multispace0, char_parser(':'), multispace0),
                        parse_parimary,
                    ),
                ),
                |fields| Value::Object(fields.into_iter().collect()),
            ),
            (char_parser('}'), opt(multispace0)),
        ),
        multispace0,
    )
    .parse(input)
}

fn parse_parimary(input: &str) -> IResult<&str, Value> {
    alt((
        parse_null,
        parse_boolean,
        parse_number,
        parse_string,
        parse_array,
        parse_object,
    ))
    .parse(input)
}
