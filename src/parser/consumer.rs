use crate::parser::parsable::Parsable;
use std::error;
use std::fmt;

#[derive(Debug)]
struct Tokenizer<'a> {
    input: Vec<&'a str>,
    position: usize,
    leftover: Leftover,
}

//Leftover is a remaining part of a single token
//For example in a command like `tail -fn10`
//after consumming "f" "n10" will become a Leftover::SigleDash
//Leftover::DoubleDash is always a value
//As in `docker logs --tail=100`. Leftover will be `=100`
#[derive(Debug)]
enum Leftover {
    SingleDash(String),
    DoubleDash(String),
    Empty,
}

impl<'a> Tokenizer<'a> {
    fn new(input: Vec<&'a str>) -> Self {
        Self {
            leftover: Leftover::Empty,
            position: 0,
            input,
        }
    }

    fn is_over(&self) -> bool {
        self.position == self.input.len() && self.leftover.is_empty()
    }

    fn read_token(&mut self) -> String {
        let token = self.input[self.position];
        self.position += 1;
        token.to_owned()
    }

    fn get_name(&mut self) -> Result<String, Box<dyn error::Error>> {
        if self.leftover.is_empty() {
            let token = self.read_token();
            match &token {
                t if t.starts_with("--") => {
                    let t = &t[2..];
                    let chunks = t.splitn(2, "=").collect::<Vec<_>>();
                    if chunks.len() == 2 {
                        let mut leftover_buffer = String::with_capacity(chunks[1].len() + 1);
                        leftover_buffer.push('=');
                        leftover_buffer.push_str(chunks[1]);
                        self.leftover = Leftover::DoubleDash(leftover_buffer);
                    }
                    Ok(chunks[0].to_owned())
                }
                t if t.starts_with("-") => {
                    let t = &t[1..];
                    let mut chars = t.chars();
                    match chars.next() {
                        None => Err(Box::new(ParsingError {
                            msg: "No charachters after `-` while searching for value".to_owned(),
                        })),
                        Some(v) => {
                            let return_val = v.to_owned();
                            let leftover_buffer = chars.collect::<String>();
                            if leftover_buffer.len() > 0 {
                                self.leftover = Leftover::SingleDash(leftover_buffer);
                            }
                            Ok(return_val.to_string())
                        }
                    }
                }
                _ => Err(Box::new(ParsingError {
                    msg: format!("Name should start with - or --. Got {}", token),
                })),
            }
        } else {
            self.leftover.read_name()
        }
    }

    fn get_value(&mut self) -> Result<String, Box<dyn error::Error>> {
        if self.leftover.is_empty() {
            Ok(self.read_token())
        } else {
            Ok(self.leftover.read_value())
        }
    }
}

impl Leftover {
    fn is_empty(&self) -> bool {
        match &self {
            Leftover::Empty => true,
            _ => false,
        }
    }

    fn read_name(&mut self) -> Result<String, Box<dyn error::Error>> {
        match &self {
            Leftover::Empty => panic!("Leftover is empty while trying to be read"),
            Leftover::DoubleDash(val) => Err(Box::new(ParsingError {
                msg: format!("Buffer contains value {} while searching for a name", val),
            })),
            Leftover::SingleDash(v) => {
                let mut chars = v.chars();
                let next_name = chars
                    .next()
                    .expect("Leftover::SingleDash was unexpectedly empty!");
                let remaining = chars.collect::<String>();
                if remaining.len() > 0 {
                    *self = Leftover::SingleDash(remaining)
                } else {
                    *self = Leftover::Empty
                }
                Ok(next_name.to_string())
            }
        }
    }

    fn read_value(&mut self) -> String {
        match &self {
            Leftover::Empty => panic!("Leftover is empty while trying to be read"),
            Leftover::DoubleDash(v) => {
                let mut chars = v.chars();
                let next_ch = chars
                    .next()
                    .expect("Leftover::Double was unexpectedly empty!");
                if next_ch != '=' {
                    panic!("Leftover::DoubleDash was not read propperly the last time")
                }
                let value = chars.collect::<String>();
                *self = Leftover::Empty;
                value
            }
            Leftover::SingleDash(v) => {
                let value = v.to_owned();
                *self = Leftover::Empty;
                value
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_factory() {
        let tok = Tokenizer::new(vec!["-abcd", "-vvvv", "VALUE"]);
        assert_eq!(tok.input, vec!["-abcd", "-vvvv", "VALUE"]);
        assert!(tok.leftover.is_empty());
        assert_eq!(tok.position, 0);
    }

    #[test]
    fn test_read_single_arguments() {
        let mut tok = Tokenizer::new(vec!["-abcd", "-vvvv", "VALUE"]);
        assert_eq!(tok.get_name().unwrap(), "a".to_owned());
        assert_eq!(tok.get_name().unwrap(), "b".to_owned());
        assert_eq!(tok.get_value().unwrap(), "cd".to_owned());
        vec!["v"; 4].iter().for_each(|name| {
            assert_eq!(tok.is_over(), false);
            assert_eq!(tok.get_name().unwrap(), name.to_owned());
        });
        assert_eq!(tok.get_value().unwrap(), "VALUE".to_owned());
        assert!(tok.is_over())
    }

    #[test]
    fn test_read_complex_attributes() {
        let mut tok = Tokenizer::new(vec![
            "--name",
            "--another_name",
            "VALUE HERE",
            "--this=that",
        ]);
        assert_eq!(tok.get_name().unwrap(), "name".to_owned());
        assert_eq!(tok.get_name().unwrap(), "another_name".to_owned());
        assert_eq!(tok.is_over(), false);
        assert_eq!(tok.get_value().unwrap(), "VALUE HERE".to_owned());
        assert_eq!(tok.get_name().unwrap(), "this".to_owned());
        assert_eq!(tok.get_value().unwrap(), "that".to_owned());
        assert!(tok.is_over())
    }

    #[test]
    fn test_read_name_from_value() {
        let mut tok = Tokenizer::new(vec!["--aaaa=bbbb"]);
        assert_eq!(tok.get_name().unwrap(), "aaaa".to_owned());
        assert!(tok.get_name().is_err());
    }

    #[test]
    fn test_no_actual_name() {
        let mut tok = Tokenizer::new(vec!["-"]);
        assert!(tok.get_name().is_err());
    }

    #[test]
    fn test_no_invalid_name() {
        let mut tok = Tokenizer::new(vec!["name"]);
        assert!(tok.get_name().is_err());
    }

}

#[derive(Debug)]
struct ParsingError {
    msg: String,
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParsingErrorOccured: {}", self.msg)
    }
}

impl error::Error for ParsingError {}

fn consume(arguments: Vec<Box<dyn Parsable>>, args: Vec<&str>) {}
