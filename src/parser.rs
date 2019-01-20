mod arg;
mod consumer;
mod flag;
mod names;
mod parsable;

use std::error;

pub trait Parse {
    fn parse(&mut self, args: Vec<String>) -> Result<(), Box<dyn error::Error>>;
}

pub struct Command<'a> {
    flags: Vec<flag::Flag<'a>>,
    // args: Vec<arg::Arg<'a>>,
    commands: Vec<Box<Command<'a>>>,
}

pub struct MyCommand<'a> {
    a: flag::Flag<'a>,
    b: flag::Flag<'a>,
    c: flag::Flag<'a>,
    x: arg::Arg<'a, String>,
}

impl<'a> Parse for MyCommand<'a> {
    fn parse(&mut self, args: Vec<String>) -> Result<(), Box<dyn error::Error>> {
        unimplemented!();
    }
}
