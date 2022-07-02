extern crate utf_railroad;

use utf_railroad::{Diagram, Terminal, NonTerminal};

fn main() {
    let diagram = Diagram::default()
        .push(Box::new(Terminal::new("Hello")))
        .push(Box::new(NonTerminal::new("World!")))
        .gap_size(2);

    println!("{}", diagram);
}