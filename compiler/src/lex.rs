use crate::lisp::{Atom, Literal, Token, Value};
use crate::scan::Scanner;

#[derive(Debug)]
pub enum LexError {
    FloatParsingFailed,
    InvalidLiteral(char),
    StringNeverClosed,
}

fn _is_space(c: char) -> bool {
    c.is_whitespace() || c == '\n' || c == '\r'
}

fn _skip_space(scanner: &mut Scanner<char>) {
    while scanner.has_next() {
        match scanner.peek() {
            Some(c) if _is_space(c) => {
                scanner.pop();
            }
            _ => {
                break;
            }
        }
    }
}

fn _scan_literal(scanner: &mut Scanner<char>) -> Result<String, LexError> {
    let mut letters = vec![];

    while scanner.has_next() {
        match scanner.peek() {
            Some(c) if _is_valid_literal(c) => {
                scanner.pop();
                letters.push(c);
            }
            Some(c) if _is_space(c) => {
                break;
            }
            Some(c) if c == ')' || c == '(' => {
                break;
            }
            Some(c) => {
                return Err(LexError::InvalidLiteral(c));
            }
            None => {
                break;
            }
        }
    }

    Ok(String::from_iter(letters))
}

fn _is_valid_literal_start(c: char) -> bool {
    c.is_alphabetic() || ['+', '-', '/', '*', '=', '<', '>'].contains(&c)
}

fn _is_valid_literal(c: char) -> bool {
    _is_valid_literal_start(c) || c.is_alphanumeric()
}

fn _scan_until(scanner: &mut Scanner<char>, end: char) -> Result<String, LexError> {
    let mut letters = vec![];
    while scanner.has_next() {
        match scanner.peek() {
            Some(c) if c == end => {
                scanner.pop();
                return Ok(String::from_iter(letters));
            }
            Some(c) => {
                scanner.pop();
                letters.push(c);
            }
            _ => {
                break;
            }
        }
    }

    Err(LexError::StringNeverClosed)
}

fn _scan_number(scanner: &mut Scanner<char>) -> Result<String, LexError> {
    let mut numbers = vec![];

    while scanner.has_next() {
        match scanner.peek() {
            Some(c) if c.is_numeric() || c == '.' => {
                scanner.pop();
                numbers.push(c);
            }
            _ => {
                break;
            }
        }
    }

    Ok(String::from_iter(numbers))
}

fn _lex_number(raw_number: &str, tokens: &mut Vec<Token>) -> Result<(), LexError> {
    if let Ok(value) = raw_number.parse::<f64>() {
        tokens.push(Token::Atom(Atom::Value(Value::Literal(Literal::Number(value)))));
        return Ok(());
    }

    return Err(LexError::FloatParsingFailed);
}

fn _lex_literal(name: &str, tokens: &mut Vec<Token>) {
    match name {
        "t" => {
            tokens.push(Token::Atom(Atom::Value(Value::Literal(Literal::True))));
        }
        "nil" => {
            tokens.push(Token::Atom(Atom::Value(Value::Literal(Literal::Nil))));
        }
        "defun" => {
            tokens.push(Token::Defun);
        }
        "if" => {
            tokens.push(Token::If);
        }
        _ => {
            tokens.push(Token::Atom(Atom::Name(String::from(name))));
        }
    }
}

pub fn lex(source: &str) -> Result<Vec<Token>, LexError> {
    let chars = source.chars().collect::<Vec<char>>();
    let mut tokens = vec![];
    let mut scanner = Scanner::new(&chars);

    while scanner.has_next() {
        _skip_space(&mut scanner);
        match scanner.peek() {
            Some('(') => {
                scanner.pop();
                tokens.push(Token::OpenParen);
            }
            Some(')') => {
                scanner.pop();
                tokens.push(Token::CloseParen);
            }
            Some(c) if c == '\'' || c == '"' => {
                scanner.pop();
                let string = _scan_until(&mut scanner, c)?;
                tokens.push(Token::Atom(Atom::Value(Value::Literal(Literal::String(string)))));
            }
            Some(c) if c.is_numeric() => {
                let raw_number = _scan_number(&mut scanner)?;
                _lex_number(&raw_number, &mut tokens)?;
            }
            Some(c) if _is_valid_literal_start(c) => {
                let literal = _scan_literal(&mut scanner)?;
                _lex_literal(&literal, &mut tokens);
            }
            _ => {
                break;
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use crate::scan::Scanner;

    use super::{_scan_literal, _skip_space, lex};

    #[test]
    fn test_skip_space() {
        let source = "  Test";
        let chars = source.chars().collect::<Vec<char>>();
        let mut scanner = Scanner::new(&chars);

        _skip_space(&mut scanner);
        assert!(scanner.peek().unwrap() == 'T');
    }

    #[test]
    fn test_scan_word() {
        let source = "Test";

        let chars = source.chars().collect::<Vec<char>>();
        let mut scanner = Scanner::new(&chars);

        let word = _scan_literal(&mut scanner).unwrap();
        assert!(word == String::from("Test"));
    }

    #[test]
    fn test_scan_int() {
        let source = "123";

        let chars = source.chars().collect::<Vec<char>>();
        let mut scanner = Scanner::new(&chars);

        let number = super::_scan_number(&mut scanner).unwrap().parse::<i64>();

        assert!(number == Ok(123));
    }

    #[test]
    fn test_scan_float() {
        let source = "123.4";
        let chars = source.chars().collect::<Vec<char>>();
        let mut scanner = Scanner::new(&chars);

        let number = super::_scan_number(&mut scanner).unwrap().parse::<f64>();

        assert!(number == Ok(123.4));
    }
}
