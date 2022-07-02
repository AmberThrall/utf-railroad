use std::fmt;
use crate::Node;

pub struct NonTerminal {
    pub(crate) contents: String,
}

impl NonTerminal {
    pub fn new(contents: &str) -> Self {
        Self { 
            contents: String::from(contents),
        }
    }
}

impl Node for NonTerminal {
    fn get_width(&self) -> usize {
        let mut width = 0;
        for line in self.contents.lines() {
            width = line.len().max(width);
        }
        width + 4
    }

    fn get_height(&self) -> usize {
        self.contents.lines().collect::<Vec<_>>().len() + 2
    }

    fn as_str(&self) -> String {
        let width = self.get_width() - 4;
        let mut ret = String::new();

        // top
        ret += "┏";
        for _ in 0..width+2 { ret += "━"; }
        ret += "┓\n";

        // contents
        for line in self.contents.lines() {
            let offset = (width - line.len())/2;
            ret += "┃ ";
            for _ in 0..offset { ret += " "; };
            ret += line;
            for _ in 0..(width - offset - line.len()) {ret += " ";}
            ret += " ┃\n";
        }

        // bottom
        ret += "┗";
        for _ in 0..width+2 { ret += "━"; }
        ret += "┛";

        ret
    }
}

impl fmt::Display for NonTerminal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}