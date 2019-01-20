use crate::parser::names;
use crate::parser::parsable;
use std::error;
use std::fmt;

pub struct Flag<'a> {
    positive_names: names::Names<'a>,
    negative_names: names::Names<'a>,
    default: bool,
    value: Option<bool>,
}

impl<'a> Flag<'a> {
    pub fn new(positive: names::Names<'a>, default: bool) -> Self {
        Self {
            positive_names: positive,
            negative_names: names::Names::empty(),
            value: None,
            default,
        }
    }

    pub fn value(&self) -> bool {
        match self.value {
            Some(v) => v,
            None => self.default,
        }
    }
}

impl<'a> parsable::Parsable for Flag<'a> {
    fn short_names<'b>(&'b self) -> Vec<&'b str> {
        let mut names_copy = self.positive_names.short_names.clone();
        names_copy.append(&mut self.negative_names.short_names.clone());
        names_copy
    }

    fn long_names<'b>(&'b self) -> Vec<&'b str> {
        let mut names_copy = self.positive_names.long_names.clone();
        names_copy.append(&mut self.negative_names.long_names.clone());
        names_copy
    }

    fn feed_arg<'b>(
        &mut self,
        name: &'b str,
        input: Option<&'b str>,
    ) -> Result<(), Box<dyn error::Error>> {
        match input {
            Some(_) => panic!("Failure while parsing flag {}", name),
            None => {
                if let Some(_) = self.value {
                    return Err(Box::new(FlagIsSetTwiceError {}));
                }
                if name.len() == 1 {
                    if self.positive_names.short_names.contains(&name) {
                        self.value = Some(true)
                    } else {
                        if self.negative_names.short_names.contains(&name) {
                            self.value = Some(false)
                        } else {
                            panic!("Parsing failure occured for name: {}", name)
                        }
                    }
                } else {
                    if self.positive_names.long_names.contains(&name) {
                        self.value = Some(true)
                    } else {
                        if self.negative_names.long_names.contains(&name) {
                            self.value = Some(false)
                        } else {
                            panic!("Parsing failure occured for name: {}", name)
                        }
                    }
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug)]
struct FlagIsSetTwiceError {}

impl fmt::Display for FlagIsSetTwiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Argument provided for flag provided for flag")
    }
}

impl error::Error for FlagIsSetTwiceError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parsable::Parsable;

    fn create_flag<'a>() -> Flag<'a> {
        Flag {
            positive_names: names::Names::new(vec!["t", "true", "totally"]),
            negative_names: names::Names::new(vec!["f", "false", "no", "n"]),
            value: None,
            default: false,
        }
    }

    #[test]
    fn test_names() {
        let f = create_flag();
        assert_eq!(f.long_names(), vec!["true", "totally", "false", "no"]);
        assert_eq!(f.short_names(), vec!["t", "f", "n"]);
    }

    #[test]
    fn test_default() {
        let f = create_flag();
        assert_eq!(f.value(), false)
    }

    #[test]
    fn test_receives_value() {
        vec!["f", "n", "no"].iter().for_each(|name| {
            let mut f = create_flag();
            f.default = true;
            f.feed_arg(name, None).unwrap();
            assert_eq!(f.value(), false)
        });
        vec!["t", "true", "totally"].iter().for_each(|name| {
            let mut f = create_flag();
            f.feed_arg(name, None).unwrap();
            assert_eq!(f.value(), true)
        })
    }

    #[test]
    #[should_panic]
    fn test_doesnt_accept_arguments() {
        create_flag().feed_arg("no", Some("panic!"));
    }

    #[test]
    #[should_panic]
    fn test_panics_if_called_with_wrong_name() {
        create_flag().feed_arg("maybe", None);
    }

    #[test]
    fn test_fails_if_called_twice() {
        let mut f = create_flag();
        f.feed_arg("true", None).unwrap();
        match f.feed_arg("false", None) {
            Ok(_) => panic!("Error should've accured"),
            Err(_) => (),
        }
    }

}
