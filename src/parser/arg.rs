use crate::parser::names;
use crate::parser::parsable;
use std::error;
use std::fmt;
use std::str::FromStr;

pub struct Arg<'a, T>
where
    T: FromStr,
{
    names: names::Names<'a>,
    value: Option<T>,
    default: Option<T>,
}

impl<'a, T> Arg<'a, T>
where
    T: FromStr,
{
    pub fn new(names: names::Names<'a>, default: T) -> Self {
        Self {
            value: None,
            default: Some(default),
            names,
        }
    }
}

impl<'a, T> parsable::Parsable for Arg<'a, T>
where
    T: FromStr,
{
    fn short_names<'b>(&'b self) -> Vec<&'b str> {
        self.names.short_names.clone()
    }

    fn long_names<'b>(&'b self) -> Vec<&'b str> {
        self.names.long_names.clone()
    }

    fn accept_arg<'b>(&'b self) -> bool {
        true
    }

    fn feed_arg<'b>(
        &mut self,
        name: &'b str,
        input: Option<&'b str>,
    ) -> Result<(), Box<dyn error::Error>> {
        match input {
            Some(v) => match v.parse::<T>() {
                Ok(val) => {
                    self.value = Some(val);
                    Ok(())
                }
                Err(_) => Err(Box::new(FailedToParseArgumentError::new(v))),
            },
            None => Err(Box::new(FailedToParseArgumentError::new(""))),
        }
    }
}

#[derive(Debug)]
struct FailedToParseArgumentError {
    arg: String,
}

impl FailedToParseArgumentError {
    fn new(input: &str) -> Self {
        Self {
            arg: input.to_owned(),
        }
    }
}

impl fmt::Display for FailedToParseArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse argument {}", self.arg)
    }
}

impl error::Error for FailedToParseArgumentError {}
