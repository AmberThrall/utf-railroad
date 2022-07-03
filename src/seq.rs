use std::fmt;
use crate::Node;

pub struct Sequence {
    pub(crate) nodes: Vec<Vec<Box<dyn Node>>>,
    pub(crate) gap_size: usize,
}

impl Sequence {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(mut self, node: Box<dyn Node>) -> Self {
        let id = self.nodes.len() - 1;
        self.nodes[id].push(node);
        self
    }

    pub fn gap_size(mut self, size: usize) -> Self {
        self.gap_size = size;
        self
    }

    pub fn next_row(mut self) -> Self {
        self.nodes.push(Vec::new());
        self
    }

    pub fn num_rows(&self) -> usize {
        self.nodes.len()
    }
}

impl Default for Sequence {
    fn default() -> Self {
        Self {
            nodes: vec![Vec::new()],
            gap_size: 2,
        }
    }
}

impl Node for Sequence {
    fn get_width(&self) -> usize {
        let mut width = 0;
        for row in &self.nodes {
            let mut row_width = 0;
            for n in row {
                row_width += n.get_width();
            }

            if row.len() > 1 {
                row_width += self.gap_size * (row.len() - 1);
            }

            width = row_width.max(width);
        }

        if self.nodes.len() > 1 { width += 3; }

        width
    }

    fn get_height(&self) -> usize {
        let mut height = 0;
        for row in &self.nodes {
            let mut row_height = 0;
            for n in row {
                row_height = n.get_height().max(row_height);
            }

            height += row_height;
        }

        if self.nodes.len() > 1 {
            height += self.nodes.len();
        }

        height
    }

    fn as_str(&self) -> String {
        let mut final_s = String::new();
        let max_width = self.get_width() - if self.nodes.len() > 1 { 3 } else { 0 };

        for row_n in 0..self.nodes.len() {
            let row = &self.nodes[row_n];
            let mut row_width = 0;
            let mut widths = Vec::new();
            let mut nodes_strs = Vec::new();
            let mut height = 0;

            for n in row {
                let w = n.get_width();
                let s = n.as_str();

                height = n.get_height().max(height);
                widths.push(w);
                nodes_strs.push(s);
                row_width += w;
            }

            if row.len() > 1 {
                row_width += self.gap_size * (row.len() - 1);
            }

            let mut ret = String::new();

            for y in 0..height {
                if self.nodes.len() > 1 {
                    ret += if row_n == 0 {
                        if y == 1 { "─" } else { " " }
                    } else {
                        if y == 1 { "└" } else if y == 0 { "│" } else { " " }
                    };
                }

                for i in 0..row.len() {
                    let lines = nodes_strs[i].lines().collect::<Vec<_>>();

                    if y < lines.len() {
                        ret += lines[y];
                    } else {
                        for _ in 0..widths[i] {
                            ret += " ";
                        }
                    }

                    // add the seperation between nodes
                    if i < row.len() - 1 {
                        let sep = if y == 1 { "─" } else { " " };
                        for _ in 0..self.gap_size { ret += sep; }
                    }
                }

                for _ in row_width..max_width { ret += if y == 1 { "─" } else { " " }; }

                if self.nodes.len() > 1 { 
                    if row_n < self.nodes.len() - 1 {
                        ret += if y == 0 { " " } else if y == 1 { "┐" } else { "│" };
                    }

                    ret += if row_n == 0 {
                        if y == 1 { "┌" } else if y == 0 { " " } else { "│" }
                    } else if row_n < self.nodes.len() - 1 {
                        "│"
                    } else {
                        if y == 0 { " │" } else if y == 1 { "─┘" } else { "  " }
                    };
                }

                if y < height - 1 { ret += "\n"; }
            }

            // migrate to next row
            final_s += &ret;

            if self.nodes.len() > 1 && row_n < self.nodes.len() - 1 {
                final_s += "\n┌";
                for _ in 0..max_width { final_s += "─"; }
                final_s += "┘│\n";
            }
        }

        final_s
    }
}

impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}