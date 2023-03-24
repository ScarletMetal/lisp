use lisp::{Atom, Literal, Token};

#[derive(Debug)]
pub enum LexError {
    FloatParsingFailed,
    InvalidLiteral(char),
    StringNeverClosed,
    ParsingEmptyString,
}

type LexResult<'a> = Result<(&'a [char], Token), LexError>;

fn _is_space(c: &char) -> bool {
    c.is_whitespace() || *c == '\n' || *c == '\r'
}

fn _is_name_start_char(c: &char) -> bool {
    c.is_alphabetic() || ['+', '-', '/', '*', '=', '<', '>'].contains(&c)
}

fn _is_name_char(c: &char) -> bool {
    _is_name_start_char(c) || c.is_alphanumeric()
}

fn _lex_name_string(source: &[char]) -> Result<(&[char], String), LexError> {
    let mut chars = vec![];
    let mut tmp = source;

    while tmp.len() > 0 {
        match tmp.first() {
            Some(c) if _is_name_char(c) => {
                chars.push(*c);
                tmp = &tmp[1..]
            }
            _ => {
                break;
            }
        }
    }

    Ok((tmp, String::from_iter(chars)))
}

fn _lex_name(source: &[char]) -> LexResult {
    let (rest, raw) = _lex_name_string(source)?;

    match raw.as_str() {
        "t" => Ok((rest, Token::Atom(Atom::Literal(Literal::True)))),
        "nil" => Ok((rest, Token::Atom(Atom::Literal(Literal::Nil)))),
        name => Ok((rest, Token::Atom(Atom::Name(String::from(name)))))
    }
}

fn _lex_number(source: &[char]) -> LexResult {
    let mut digits = vec![];
    let mut tmp = source;

    while tmp.len() > 0 {
        match tmp.first() {
            Some(digit) if digit.is_numeric() || *digit == '.' => {
                digits.push(digit);
                tmp = &tmp[1..];
            }
            _ => {
                break;
            }
        }
    }

    match String::from_iter(digits).parse::<f64>() {
        Ok(float) => Ok((tmp, Token::Atom(Atom::Literal(Literal::Number(float))))),
        Err(_) => Err(LexError::FloatParsingFailed)
    }
}

fn _lex_string(source: &[char]) -> LexResult {
    let mut chars = vec![];
    let mut tmp = source;

    while tmp.len() > 0 {
        match tmp.first() {
            Some('"') => {
                tmp = &tmp[1..];
                break;
            }
            Some(c) => {
                chars.push(*c);
                tmp = &tmp[1..]
            }
            None => {
                return Err(LexError::StringNeverClosed)
            }
        }
    }

    let string = String::from_iter(chars);
    Ok(
        (tmp, Token::Atom(Atom::Literal(Literal::String(string))))
    )
}

fn _lex_one(source: &[char]) -> LexResult {
    match source.first() {
        Some('(') => Ok((&source[1..], Token::OpenParen)),
        Some(')') => Ok((&source[1..], Token::CloseParen)),
        Some('"') => _lex_string(&source[1..]),
        Some(c) if _is_name_start_char(c) => _lex_name(source),
        Some(c) if c.is_numeric() => _lex_number(source),
        Some(c) => Err(LexError::InvalidLiteral(*c)),
        None => Err(LexError::ParsingEmptyString)
    }
}

fn _skip_space(source: &[char]) -> &[char] {
    let mut tmp = source;
    while tmp.len() > 0 {
        match tmp.first() {
            Some(c) if _is_space(c) => {
                tmp = &tmp[1..];
            }
            _ => {
                break;
            }
        }
    }

    tmp
}

pub fn lex(source: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = vec![];
    let chars = source.chars().collect::<Vec<char>>();
    let mut tmp = &chars[..];

    while tmp.len() > 0 {
        tmp = _skip_space(tmp);
        if tmp.len() > 0 {
            let (rest, token) = _lex_one(tmp)?;
            tmp = rest;
            tokens.push(token);
        }
    }

    Ok(tokens)
}
