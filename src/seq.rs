use std::fmt;
use crate::Node;

pub struct Sequence {
    pub(crate) nodes: Vec<Box<dyn Node>>,
    pub(crate) gap_size: usize,
}

impl Sequence {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(mut self, node: Box<dyn Node>) -> Self {
        self.nodes.push(node);
        self
    }

    pub fn gap_size(mut self, size: usize) -> Self {
        self.gap_size = size;
        self
    }
}

impl Default for Sequence {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            gap_size: 2,
        }
    }
}

impl Node for Sequence {
    fn get_width(&self) -> usize {
        let mut width = 0;
        for n in &self.nodes {
            width += n.get_width();
        }

        if self.nodes.len() > 1 {
            width += self.gap_size * (self.nodes.len() - 1);
        }

        width
    }

    fn get_height(&self) -> usize {
        let mut height = 0;
        for n in &self.nodes {
            height = n.get_height().max(height);
        }
        height
    }

    fn as_str(&self) -> String {
        let mut widths = Vec::new();
        let mut nodes_strs = Vec::new();
        let height = self.get_height();

        for n in &self.nodes {
            let w = n.get_width();
            let s = n.as_str();

            widths.push(w);
            nodes_strs.push(s);
        }

        let mut ret = String::new();

        for y in 0..height {
            for i in 0..self.nodes.len() {
                let lines = nodes_strs[i].lines().collect::<Vec<_>>();

                if y < lines.len() {
                    ret += lines[y];
                } else {
                    for _ in 0..widths[i] {
                        ret += " ";
                    }
                }

                // add the seperation between nodes
                if i < self.nodes.len() - 1 {
                    let sep = if y == 1 { "─" } else { " " };
                    for _ in 0..self.gap_size { ret += sep; }
                }
            }

            if y < height - 1 {
                ret += "\n";
            }
        }

        ret
    }
}

impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}