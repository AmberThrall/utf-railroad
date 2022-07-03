extern crate utf_railroad;

use utf_railroad::{Diagram, Terminal, NonTerminal, Optional, Repeat, Sequence};

fn main() {
    let diagram = Diagram::default()
        .push(Box::new(Terminal::new("CREATE")))
        .push(Box::new(Terminal::new("VIRTUAL")))
        .push(Box::new(Terminal::new("TABLE")))
        .push(Box::new(
            Optional::new(Box::new(
                Sequence::default()
                    .push(Box::new(Terminal::new("IF")))
                    .push(Box::new(Terminal::new("NOT")))
                    .push(Box::new(Terminal::new("EXISTS")))
            ))
        ))
        .next_row()
        .push(Box::new(
            Optional::new(Box::new(
                Sequence::default()
                    .push(Box::new(NonTerminal::new("schema-name")))
                    .push(Box::new(Terminal::new(".")))
            ))
        ))
        .push(Box::new(NonTerminal::new("table-name")))
        .next_row()
        .push(Box::new(Terminal::new("USING")))
        .push(Box::new(NonTerminal::new("module-name")))
        .push(Box::new(
            Optional::new(Box::new(
                Sequence::default()
                    .push(Box::new(Terminal::new("(")))
                    .push(Box::new(
                        Repeat::new(Box::new(
                            NonTerminal::new("module-argument")
                        ))
                    ))
                    .push(Box::new(Terminal::new(")")))
            ))
        ));
    println!("{}", diagram);
}