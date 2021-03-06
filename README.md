# UTF Railroad

Generate text railroad diagrams in rust.

# Usage

```rust
use utf_railroad::{Diagram, Terminal, NonTerminal};

fn main() {
    let diagram = Diagram::default()
        .push(Box::new(Terminal::new("Hello")))
        .push(Box::new(NonTerminal::new("World!")))
        .gap_size(2);

    println!("{}", diagram);
}
```

Output:
```
   ╭───────╮  ┏━━━━━━━━┓   
╟──│ Hello │──┃ World! ┃──╢
   ╰───────╯  ┗━━━━━━━━┛   
```

## Examples

JSON Number:
```
    ╭───╮              ╭───╮               ╭───╮   ╭───────╮      ╭───╮     ╭───╮     ╭───────╮     
╟──┬│ - │┬──┬──────────│ 0 │───────────┬──┬│ . │──┬│ digit │┬┬──┬┬│ e │┬──┬┬│ + │┬┬──┬│ digit │┬┬──╢
   │╰───╯│  │          ╰───╯           │  │╰───╯  │╰───────╯││  ││╰───╯│  ││╰───╯││  │╰───────╯││   
   ╰─────╯  │╭───────────╮   ╭───────╮ │  │       ╰1+ times─╯│  ││╭───╮│  ││╭───╮││  ╰1+ times─╯│   
            ╰│ digit 1-9 │──┬│ digit │┬╯  ╰──────────────────╯  │╰│ E │╯  │╰│ - │╯│             │   
             ╰───────────╯  │╰───────╯│                         │ ╰───╯   │ ╰───╯ │             │   
                            ╰0+ times─╯                         │         ╰───────╯             │   
                                                                ╰───────────────────────────────╯   
```

Stacked diagrams:
```
    ╭────────╮  ╭─────────╮  ╭───────╮   ╭────╮  ╭─────╮  ╭────────╮      
╟───│ CREATE │──│ VIRTUAL │──│ TABLE │──┬│ IF │──│ NOT │──│ EXISTS │┬┐┌──╢
    ╰────────╯  ╰─────────╯  ╰───────╯  │╰────╯  ╰─────╯  ╰────────╯│││   
                                        ╰───────────────────────────╯││   
   ┌─────────────────────────────────────────────────────────────────┘│   
   │ ┏━━━━━━━━━━━━━┓  ╭───╮   ┏━━━━━━━━━━━━┓                          │   
   └┬┃ schema-name ┃──│ . │┬──┃ table-name ┃─────────────────────────┐│   
    │┗━━━━━━━━━━━━━┛  ╰───╯│  ┗━━━━━━━━━━━━┛                         ││   
    ╰──────────────────────╯                                         ││   
   ┌─────────────────────────────────────────────────────────────────┘│   
   │╭───────╮  ┏━━━━━━━━━━━━━┓   ╭───╮   ┏━━━━━━━━━━━━━━━━━┓   ╭───╮  │   
   └│ USING │──┃ module-name ┃──┬│ ( │──┬┃ module-argument ┃┬──│ ) │┬─┘   
    ╰───────╯  ┗━━━━━━━━━━━━━┛  │╰───╯  │┗━━━━━━━━━━━━━━━━━┛│  ╰───╯│     
                                │       ╰─────0+ times──────╯       │     
                                ╰───────────────────────────────────╯     
```

JSON String:
```
   ╭───╮    ╭──────────────────────────────────────────────────────────╮    ╭───╮   
╟──│ " │──┬┬│ Any unicode character except " or \ or control character │┬┬──│ " │──╢
   ╰───╯  ││╰──────────────────────────────────────────────────────────╯││  ╰───╯   
          ││          ╭───╮               ╭───╮                         ││          
          │╰──────────│ \ │──┬────────────│ " │─────────────┬───────────╯│          
          │           ╰───╯  │            ╰───╯             │            │          
          │                  │            ╭───╮             │            │          
          │                  ├────────────│ \ │─────────────┤            │          
          │                  │            ╰───╯             │            │          
          │                  │            ╭───╮             │            │          
          │                  ├────────────│ / │─────────────┤            │          
          │                  │            ╰───╯             │            │          
          │                  │            ╭───╮             │            │          
          │                  ├────────────│ b │─────────────┤            │          
          │                  │            ╰───╯             │            │          
          │                  │            ╭───╮             │            │          
          │                  ├────────────│ f │─────────────┤            │          
          │                  │            ╰───╯             │            │          
          │                  │            ╭───╮             │            │          
          │                  ├────────────│ n │─────────────┤            │          
          │                  │            ╰───╯             │            │          
          │                  │            ╭───╮             │            │          
          │                  ├────────────│ r │─────────────┤            │          
          │                  │            ╰───╯             │            │          
          │                  │            ╭───╮             │            │          
          │                  ├────────────│ t │─────────────┤            │          
          │                  │            ╰───╯             │            │          
          │                  │╭───╮   ╭───────────────────╮ │            │          
          │                  ╰│ u │──┬│ hexadecimal digit │┬╯            │          
          │                   ╰───╯  │╰───────────────────╯│             │          
          │                          ╰───────4 times───────╯             │          
          ╰───────────────────────────0+ times───────────────────────────╯  
```