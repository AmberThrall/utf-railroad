use std::fmt;
use crate::Node;

pub struct Repeat {
    pub node: Box<dyn Node>,
    pub min: usize,
    pub max: Option<usize>,
}

impl Repeat {
    pub fn new(node: Box<dyn Node>) -> Self {
        Self {
            node,
            min: 0,
            max: None,
        }
    }

    pub fn min(mut self, min: usize) -> Self {
        self.min = min;
        self
    }

    pub fn max(mut self, max: usize) -> Self {
        self.max = Some(max);
        self
    }

    fn caption(&self) -> String {
        if let Some(max) = self.max {
            if max == self.min {
                format!("{} time{}", self.min, if self.min == 1 { "" } else { "s" })
            } else {
                format!("{}-{} times", self.min, max)
            }
        } else {
            format!("{}+ times", self.min)
        }
    }
}

impl Node for Repeat {
    fn get_width(&self) -> usize {
        self.node.get_width().max(self.caption().len()) + 2
    }

    fn get_height(&self) -> usize {
        self.node.get_height() + 1
    }

    fn as_str(&self) -> String {
        let mut ret = String::new();
        let width = self.get_width();

        let s = self.node.as_str();
        let lines = s.lines().collect::<Vec<_>>();
        for y in 0..lines.len() {
            let sep = if y == 0 { " " } 
                else if y == 1 { "┬" }
                else { "│" };
            ret += sep;

            let offset = (width - 2 - lines[y].chars().count()) / 2;
            for _ in 0..offset { ret += if y == 1 { "─" } else { " " }; }
            ret += lines[y];
            for _ in 0..(width - 2 - lines[y].chars().count() - offset) { ret += if y == 1 { "─" } else { " " }; }

            ret += sep;
            ret += "\n";
        }

        let caption = self.caption();
        let offset = (width - 2 - caption.len()) / 2;

        ret += "╰";
        for _ in 0..offset { ret += "─" }
        ret += &caption;
        for _ in 0..(width - 2 - caption.len() - offset) { ret += "─" }
        ret += "╯";

        ret
    }
}

impl fmt::Display for Repeat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}