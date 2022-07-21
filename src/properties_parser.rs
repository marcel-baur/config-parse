//! Parser by: https://github.com/ZacBlanco
use crate::config::Configuration;
use crate::model::Record;
use crate::writer::write;
use std::fs;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_till};
use nom::combinator::{complete, eof, opt, value};

use nom::character::complete::{none_of, one_of};
use nom::multi::{many0, many1, many_till, separated_list0, separated_list1};

use nom::IResult;

fn consume_whitespaces(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _) = many0(one_of(" \t\u{c}"))(input)?;
    Ok((input, ()))
}

fn consume_eol(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _) = alt((complete(tag("\r\n")), tag("\r"), tag("\n")))(input)?;
    Ok((input, ()))
}

fn consume_eol_or_eof(input: &[u8]) -> IResult<&[u8], ()> {
    alt((value((), eof), consume_eol))(input)
}

fn blank_line(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _) = consume_whitespaces(input)?;
    consume_eol_or_eof(input)
}

fn comment_line(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _) = consume_whitespaces(input)?;
    let (input, _) = one_of("#!")(input)?;
    let (input, _) = take_till(eol)(input)?;
    consume_eol_or_eof(input)
}

/// Returns whether or not a byte (as a character) represents a EOL character
/// (line feed `\r` or newline `\n`)
fn eol(c: u8) -> bool {
    c as char == '\r' || c as char == '\n'
}

/// Consumes a single line escape and any whitespaces after it
fn consume_line(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _) = tag(r"\")(input)?;
    let (input, _) = consume_eol(input)?;
    let (input, _) = consume_whitespaces(input)?;
    Ok((input, ()))
}

/// Consumes a set of alternating lines and whiespaces. Stopping once there is
/// no more alternating
fn consume_whitespaces_and_lines(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _) =
        separated_list0(many1(consume_line), consume_whitespaces)(input)?;
    Ok((input, ()))
}

/// Consumes a character that exists in a key
fn char_in_key(input: &[u8]) -> IResult<&[u8], char> {
    none_of(":=\n\r \t\u{c}\\")(input)
}

/// Consumes a character which exists in a value
fn char_in_value(input: &[u8]) -> IResult<&[u8], char> {
    none_of("\n\r\\")(input)
}

/// matches a single character and returns its escaped equivalent e.g. `'t' ->
/// '\t'`
fn escaped_char_to_char(v: char) -> char {
    match v {
        't' => '\t',
        'n' => '\n',
        'f' => '\u{c}',
        'r' => '\r',
        '\\' => '\\',
        _ => v,
    }
}

/// consumes an escaped character in a key or value
fn escape_in_key_or_value(input: &[u8]) -> IResult<&[u8], char> {
    let (input, _) = tag(r"\")(input)?;
    let (input, c) = none_of("u\r\n")(input)?;
    Ok((input, escaped_char_to_char(c)))
}

/// consumes a character in a key
fn one_char_in_key(input: &[u8]) -> IResult<&[u8], char> {
    alt((escape_in_key_or_value, char_in_key))(input)
}

/// consumes a character in a value
fn one_char_in_value(input: &[u8]) -> IResult<&[u8], char> {
    alt((escape_in_key_or_value, char_in_value))(input)
}

/// Consumes and returns a `String` representing the key to a property.
fn consume_key(input: &[u8]) -> IResult<&[u8], String> {
    // use many1(consume_line) because many0 always returns true and causes a
    // separated list error.
    let (input, chars) =
        separated_list1(many1(consume_line), many1(one_char_in_key))(input)?;
    Ok((input, chars.into_iter().flatten().collect::<String>()))
}

/// Consumes and returns a `String` representing the value of a property.
fn consume_value(input: &[u8]) -> IResult<&[u8], String> {
    // use many1(consume_line) because many0 always returns true and causes a
    // separated list error.
    let (input, chars) =
        separated_list0(many1(consume_line), many0(one_char_in_value))(input)?;
    Ok((input, chars.into_iter().flatten().collect::<String>()))
}

/// Consumes an entire line (or set of lines) representing a key-value property
fn kv_line(input: &[u8]) -> IResult<&[u8], Record> {
    let (input, _) = consume_whitespaces_and_lines(input)?;
    let (input, key) = consume_key(input)?;
    let (input, _) = consume_whitespaces_and_lines(input)?;
    let (input, _) = opt(complete(one_of(":=")))(input)?;
    let (input, _) = consume_whitespaces_and_lines(input)?;
    let (input, value) = consume_value(input)?;
    let (input, _) = consume_eol_or_eof(input)?;
    Ok((input, Record { key, value }))
}

type ParsedProps<'a> = (Vec<Option<Record>>, &'a [u8]);

/// The full parser which consumes comments, blanks, and Property lines.
fn _fparser(input: &[u8]) -> IResult<&[u8], ParsedProps> {
    many_till(
        alt((
            value(None, complete(comment_line)),
            value(None, complete(blank_line)),
            opt(complete(kv_line)),
        )),
        eof,
    )(input)
}

/// Public parser function
fn parser(input: &[u8]) -> IResult<&[u8], Vec<Record>> {
    let (input, props) = _fparser(input)?;
    let v = props.0.into_iter().flatten().collect();
    Ok((input, v))
}

pub fn parse(configuration: Configuration) {
    for file in configuration.files {
        let file_write = file.clone();
        let contents = fs::read(file).expect("Failed to read file!");
        match parser(&contents) {
            Ok((_input, properties)) => {
                println!("{:?}", properties);

                match write(properties, file_write) {
                    Ok(()) => {}
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
            Err(_) => {
                println!("ERROR");
            }
        }
    }
}
