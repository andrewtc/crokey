//! Crokey helps incorporate configurable keybindings in [crossterm](https://github.com/crossterm-rs/crossterm)
//! based terminal applications by providing functions
//! - parsing key combinations from strings
//! - describing key combinations in strings

use {
    crossterm::event::{KeyCode::*, KeyEvent, KeyModifiers},
    std::fmt,
};

#[derive(Debug)]
pub struct ParseKeyError {
    /// the string which couldn't be parsed
    pub raw: String,
}

impl ParseKeyError {
    pub fn new<S: Into<String>>(s: S) -> Self {
        Self { raw: s.into() }
    }
}

impl fmt::Display for ParseKeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} can't be parsed as a key", self.raw)
    }
}

impl std::error::Error for ParseKeyError {}

/// parse a string as a keyboard key combination definition.
///
/// About the case:
/// The char we receive as code from crossterm is usually lowercase
/// but uppercase when it was typed with shift (i.e. we receive
/// "g" for a lowercase, and "shift-G" for an uppercase)
pub fn parse(raw: &str) -> Result<KeyEvent, ParseKeyError> {
    let mut modifiers = KeyModifiers::empty();
    let raw = raw.to_ascii_lowercase();
    let mut raw: &str = raw.as_ref();
    loop {
        if let Some(end) = raw.strip_prefix("ctrl-") {
            raw = end;
            modifiers.insert(KeyModifiers::CONTROL);
        } else if let Some(end) = raw.strip_prefix("alt-") {
            raw = end;
            modifiers.insert(KeyModifiers::ALT);
        } else if let Some(end) = raw.strip_prefix("shift-") {
            raw = end;
            modifiers.insert(KeyModifiers::SHIFT);
        } else {
            break;
        }
    }
    let code = match raw {
        "esc" => Esc,
        "enter" => Enter,
        "left" => Left,
        "right" => Right,
        "up" => Up,
        "down" => Down,
        "home" => Home,
        "end" => End,
        "pageup" => PageUp,
        "pagedown" => PageDown,
        "backtab" => {
            // Crossterm always sends SHIFT with backtab
            modifiers.insert(KeyModifiers::SHIFT);
            BackTab
        }
        "backspace" => Backspace,
        "del" => Delete,
        "delete" => Delete,
        "insert" => Insert,
        "ins" => Insert,
        "f1" => F(1),
        "f2" => F(2),
        "f3" => F(3),
        "f4" => F(4),
        "f5" => F(5),
        "f6" => F(6),
        "f7" => F(7),
        "f8" => F(8),
        "f9" => F(9),
        "f10" => F(10),
        "f11" => F(11),
        "f12" => F(12),
        "space" => Char(' '),
        "hyphen" => Char('-'),
        "minus" => Char('-'),
        "tab" => Tab,
        c if c.len() == 1 => {
            let mut c = c.chars().next().unwrap();
            if modifiers.contains(KeyModifiers::SHIFT) {
                c = c.to_ascii_uppercase();
            }
            Char(c)
        }
        _ => {
            return Err(ParseKeyError::new(raw));
        }
    };
    Ok(KeyEvent::new(code, modifiers))
}

#[test]
fn check_key_parsing() {
    use crate::*;
    fn check_ok(raw: &str, key: KeyEvent) {
        let parsed = parse(raw);
        assert!(parsed.is_ok(), "failed to parse {:?} as key", raw);
        assert_eq!(parsed.unwrap(), key);
    }
    assert!(parse("").is_err());
    check_ok("left", key!(left));
    check_ok("RIGHT", key!(right));
    check_ok("Home", key!(HOME));
    check_ok(
        "backtab",
        KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT),
    );
    check_ok("f1", KeyEvent::from(F(1)));
    check_ok("F2", KeyEvent::from(F(2)));
    check_ok("Enter", KeyEvent::from(Enter));
    check_ok("alt-enter", KeyEvent::new(Enter, KeyModifiers::ALT));
    check_ok("insert", KeyEvent::from(Insert));
    check_ok("ctrl-q", KeyEvent::new(Char('q'), KeyModifiers::CONTROL));
    check_ok("shift-q", KeyEvent::new(Char('Q'), KeyModifiers::SHIFT));
    check_ok("ctrl-Q", KeyEvent::new(Char('q'), KeyModifiers::CONTROL));
    check_ok("shift-Q", KeyEvent::new(Char('Q'), KeyModifiers::SHIFT));
    check_ok(
        "ctrl-shift-Q",
        KeyEvent::new(Char('Q'), KeyModifiers::SHIFT | KeyModifiers::CONTROL),
    );
    check_ok("-", KeyEvent::new(Char('-'), KeyModifiers::NONE));
    check_ok("Hyphen", KeyEvent::new(Char('-'), KeyModifiers::NONE));
    check_ok("alt--", KeyEvent::new(Char('-'), KeyModifiers::ALT));
    check_ok("alt-hyphen", KeyEvent::new(Char('-'), KeyModifiers::ALT));
    check_ok("alt-hyphen", KeyEvent::new(Char('-'), KeyModifiers::ALT));
    check_ok(
        "ctrl-Shift-alt-space",
        KeyEvent::new(
            Char(' '),
            KeyModifiers::ALT | KeyModifiers::SHIFT | KeyModifiers::ALT | KeyModifiers::CONTROL,
        ),
    );
    check_ok(
        "ctrl-shift-alt--",
        KeyEvent::new(
            Char('-'),
            KeyModifiers::ALT | KeyModifiers::SHIFT | KeyModifiers::ALT | KeyModifiers::CONTROL,
        ),
    );
}
