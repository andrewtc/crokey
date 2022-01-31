[![MIT][s2]][l2] [![Latest Version][s1]][l1] [![docs][s3]][l3] [![Chat on Miaou][s4]][l4]

[s1]: https://img.shields.io/crates/v/crokey.svg
[l1]: https://crates.io/crates/crokey

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: LICENSE

[s3]: https://docs.rs/crokey/badge.svg
[l3]: https://docs.rs/crokey/

[s4]: https://miaou.dystroy.org/static/shields/room.svg
[l4]: https://miaou.dystroy.org/3490?crokey

# Crokey

Crokey helps incorporate configurable keybindings in [crossterm](https://github.com/crossterm-rs/crossterm)
based terminal applications by providing functions
- parsing key combinations from strings
- describing key combinations in strings
- parsing key combinations at compile time

## Parse a string

Those strings are usually provided by a configuration file.

```rust
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
assert_eq!(
    crokey::parse("alt-enter").unwrap(),
    KeyEvent::new(KeyCode::Enter, KeyModifiers::ALT),
);
assert_eq!(
    crokey::parse("shift-F6").unwrap(),
    KeyEvent::new(KeyCode::F(6), KeyModifiers::SHIFT),
);
```

## Use key event "literals" thanks to procedural macros

Those key events are parsed at compile time and have zero runtime cost.

They're efficient and convenient for matching events or defining hardcoded keybindings.

```rust
match key_event {
    key!(ctrl-c) => {
        println!("Arg! You savagely killed me with a {}", fmt.to_string(key_event).red());
        break;
    }
    key!(ctrl-q) => {
        println!("You typed {} which gracefully quits", fmt.to_string(key_event).green());
        break;
    }
    _ => {
        println!("You typed {}", fmt.to_string(key_event).blue());
    }
}
```
Complete example in `/examples/print_key`:

![print_key](doc/print_key.png)

## Display a string with a configurable format

```rust
use crokey::*;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

// The default format
let format = KeyEventFormat::default();
assert_eq!(format.to_string(key!(shift-a)), "Shift-a");
assert_eq!(format.to_string(key!(ctrl-c)), "Ctrl-c");

// A more compact format
let format = KeyEventFormat::default()
    .with_implicit_shift()
    .with_control("^");
assert_eq!(format.to_string(key!(shift-a)), "A");
assert_eq!(format.to_string(key!(ctrl-c)), "^c");
```

Crokey is used in [broot](https://github.com/Canop/broot) and [SafeCloset](https://github.com/Canop/safecloset).
