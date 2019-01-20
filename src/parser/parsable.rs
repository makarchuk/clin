use std::error;

pub trait Parsable {
    fn short_names<'a>(&'a self) -> Vec<&'a str>;
    fn long_names<'a>(&'a self) -> Vec<&'a str>;
    fn accept_arg(&self) -> bool {
        false
    }
    fn feed_arg<'a>(
        &mut self,
        name: &'a str,
        input: Option<&'a str>,
    ) -> Result<(), Box<dyn error::Error>>;
}
