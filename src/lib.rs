mod node;
mod terminal;
mod nonterminal;
mod seq;
mod choice;
mod optional;
mod repeat;
mod diagram;

pub use node::Node;
pub use terminal::Terminal;
pub use nonterminal::NonTerminal;
pub use seq::Sequence;
pub use choice::Choice;
pub use optional::Optional;
pub use repeat::Repeat;
pub use diagram::Diagram;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
