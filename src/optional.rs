use std::fmt;
use crate::Node;

pub struct Optional(Box<dyn Node>);

impl Optional {
    pub fn new(node: Box<dyn Node>) -> Self {
        Self(node)
    }
}

impl Node for Optional {
    fn get_width(&self) -> usize {
        self.0.get_width() + 2
    }

    fn get_height(&self) -> usize {
        self.0.get_height() + 1
    }

    fn as_str(&self) -> String {
        let mut ret = String::new();

        let s = self.0.as_str();
        let lines = s.lines().collect::<Vec<_>>();
        for y in 0..lines.len() {
            let sep = if y == 0 { " " } 
                else if y == 1 { "┬" }
                else { "│" };
            ret += sep;

            ret += lines[y];

            ret += sep;
            ret += "\n";
        }

        ret += "╰";
        for _ in 0..self.get_width()-2 { ret += "─" }
        ret += "╯";
        ret
    }
}

impl fmt::Display for Optional {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}