use std::fmt;
use crate::Node;

pub struct Choice {
    pub(crate) nodes: Vec<Box<dyn Node>>,
}

impl Choice {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(mut self, node: Box<dyn Node>) -> Self {
        self.nodes.push(node);
        self
    }
}

impl Default for Choice {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
        }
    }
}

impl Node for Choice {
    fn get_width(&self) -> usize {
        let mut width = 0;
        for n in &self.nodes {
            width = n.get_width().max(width);
        }
        
        width + 2
    }

    fn get_height(&self) -> usize {
        let mut height = 0;
        for n in &self.nodes {
            height += n.get_height();
        }
        height
    }

    fn as_str(&self) -> String {
        let mut max_width = 0;
        for n in &self.nodes {
            max_width = n.get_width().max(max_width);
        }

        let mut ret = String::new();

        for i in 0..self.nodes.len() {
            let s = self.nodes[i].as_str();
            let lines = s.lines().collect::<Vec<_>>();

            for y in 0..lines.len() {
                ret += if i == 0 { 
                    if y == 1 { "┬" } else if y > 1 { "│" } else { " " }
                } else if i < self.nodes.len() - 1 { 
                    if y == 1 { "├" } else { "│" }
                } else { 
                    if y == 1 { "╰" } else if y < 1 { "│" } else { " " }
                };

                let offset = (max_width - lines[y].chars().count()) / 2;
                let sep2 = if y == 1 { "─" } else { " " };
                for _ in 0..offset { ret += sep2; }
                ret += lines[y];
                for _ in 0..(max_width - offset - lines[y].chars().count()) { ret += sep2; }

                ret += if i == 0 { 
                    if y == 1 { "┬" } else if y > 1 { "│" } else { " " }
                } else if i < self.nodes.len() - 1 { 
                    if y == 1 { "┤" } else { "│" }
                } else { 
                    if y == 1 { "╯" } else if y < 1 { "│" } else { " " }
                };
                ret += "\n";
            }
        }

        ret
    }
}

impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}