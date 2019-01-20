pub struct Names<'a> {
    pub short_names: Vec<&'a str>,
    pub long_names: Vec<&'a str>,
}

impl<'a> Names<'a> {
    pub fn empty() -> Self {
        Self {
            short_names: vec![],
            long_names: vec![],
        }
    }

    pub fn new(names: Vec<&'a str>) -> Self {
        Self {
            short_names: names
                .iter()
                .filter(|name| name.len() == 1)
                .map(|n| *n)
                .collect(),
            long_names: names
                .iter()
                .filter(|name| name.len() != 1)
                .map(|n| *n)
                .collect(),
        }
    }
}
