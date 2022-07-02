extern crate utf_railroad;

use utf_railroad::{Diagram, Choice, Terminal, Sequence, Repeat, Optional, NonTerminal};

fn main() {
    let object = Diagram::default()
        .push(Box::new(Terminal::new("{")))
        .push(Box::new(
            Optional::new(Box::new(
                Sequence::default()
                    .push(Box::new(
                        Repeat::new(Box::new(
                            Sequence::default()
                                .push(Box::new(NonTerminal::new("string")))
                                .push(Box::new(Terminal::new(":")))
                                .push(Box::new(NonTerminal::new("value")))
                                .push(Box::new(Terminal::new(",")))
                        ))
                    ))
                    .push(Box::new(
                        Sequence::default()
                            .push(Box::new(NonTerminal::new("string")))
                            .push(Box::new(Terminal::new(":")))
                            .push(Box::new(NonTerminal::new("value")))
                    ))
            ))
        ))
        .push(Box::new(Terminal::new("}")));
    println!("object:\n{}\n", object);

    let array = Diagram::default()
        .push(Box::new(Terminal::new("[")))
        .push(Box::new(
            Optional::new(Box::new(
                Sequence::default()
                    .push(Box::new(
                        Repeat::new(Box::new(
                            Sequence::default()
                                .push(Box::new(NonTerminal::new("value")))
                                .push(Box::new(Terminal::new(",")))
                        ))
                    ))
                    .push(Box::new(NonTerminal::new("value")))
            ))
        ))
        .push(Box::new(Terminal::new("]")));
    println!("array:\n{}\n", array);

    let value = Diagram::default()
        .push(Box::new(
            Choice::default()
                .push(Box::new(NonTerminal::new("string")))
                .push(Box::new(NonTerminal::new("number")))
                .push(Box::new(NonTerminal::new("object")))
                .push(Box::new(NonTerminal::new("array")))
                .push(Box::new(Terminal::new("true")))
                .push(Box::new(Terminal::new("false")))
                .push(Box::new(Terminal::new("null")))
        ));
    println!("value:\n{}\n", value);

    let string = Diagram::default()
        .push(Box::new(Terminal::new("\"")))
        .push(Box::new(
            Repeat::new(Box::new(
                Choice::default()
                    .push(Box::new(Terminal::new("Any unicode character except \" or \\ or control character")))
                    .push(Box::new(
                        Sequence::default()
                            .push(Box::new(Terminal::new("\\")))
                            .push(Box::new(
                                Choice::default()
                                    .push(Box::new(Terminal::new("\"")))
                                    .push(Box::new(Terminal::new("\\")))
                                    .push(Box::new(Terminal::new("/")))
                                    .push(Box::new(Terminal::new("b")))
                                    .push(Box::new(Terminal::new("f")))
                                    .push(Box::new(Terminal::new("n")))
                                    .push(Box::new(Terminal::new("r")))
                                    .push(Box::new(Terminal::new("t")))
                                    .push(Box::new(
                                        Sequence::default()
                                            .push(Box::new(Terminal::new("u")))
                                            .push(Box::new(
                                                Repeat::new(Box::new(Terminal::new("hexadecimal digit")))
                                                    .min(4)
                                                    .max(4)
                                            ))
                                    ))
                            ))
                    ))
                ))
        ))
        .push(Box::new(Terminal::new("\"")));
    println!("string:\n{}\n", string);

    let number = Diagram::default()
        .push(Box::new(Optional::new(Box::new(Terminal::new("-")))))
        .push(Box::new(
            Choice::default()
                .push(Box::new(Terminal::new("0")))
                .push(Box::new(
                    Sequence::default()
                        .push(Box::new(Terminal::new("digit 1-9")))
                        .push(Box::new(
                            Repeat::new(Box::new(Terminal::new("digit")))
                        ))
                ))
        ))
        .push(Box::new(
            Optional::new(Box::new(
                Sequence::default()
                    .push(Box::new(Terminal::new(".")))
                    .push(Box::new(
                        Repeat::new(Box::new(Terminal::new("digit")))
                            .min(1)
                    ))
            ))
        ))
        .push(Box::new(
            Optional::new(Box::new(
                Sequence::default()
                    .push(Box::new(
                        Choice::default()
                            .push(Box::new(Terminal::new("e")))
                            .push(Box::new(Terminal::new("E")))
                    ))
                    .push(Box::new(
                        Optional::new(Box::new(
                            Choice::default()
                                .push(Box::new(Terminal::new("+")))
                                .push(Box::new(Terminal::new("-")))
                        ))
                    ))
                    
                    .push(Box::new(
                        Repeat::new(Box::new(Terminal::new("digit")))
                            .min(1)
                    ))
            ))
        ));
    println!("number:\n{}", number);
}