use std::fmt;
use crate::{Node, Sequence};

pub struct Diagram(Sequence);

impl Diagram {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(self, node: Box<dyn Node>) -> Self {
        Self(self.0.push(node))
    }

    pub fn gap_size(self, size: usize) -> Self {
        Self(self.0.gap_size(size))
    }

    pub fn next_row(self) -> Self {
        Self(self.0.next_row())
    }

    pub fn num_rows(&self) -> usize {
        self.0.num_rows()
    }
}

impl Default for Diagram {
    fn default() -> Self {
        Self(Sequence::default())
    }
}

impl Node for Diagram {
    fn get_width(&self) -> usize {
        self.0.get_width() + self.0.gap_size * 2 + 2
    }

    fn get_height(&self) -> usize {
        self.0.get_height()
    }

    fn as_str(&self) -> String {
        let mut ret = String::new();

        let s = self.0.as_str();
        let lines = s.lines().collect::<Vec<_>>();
        for y in 0..lines.len() {
            if y == 1 {
                ret += "╟";
                for _ in 0..self.0.gap_size { ret += "─"; }
            } else {
                ret += " ";
                for _ in 0..self.0.gap_size { ret += " "; }
            }

            ret += lines[y];

            if y == 1 {
                for _ in 0..self.0.gap_size { ret += "─"; }
                ret += "╢";
            } else {
                for _ in 0..self.0.gap_size { ret += " "; }
                ret += " ";
            }

            if y < lines.len() - 1 { ret += "\n" }
        }

        ret
    }
}

impl fmt::Display for Diagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}