//Syntax Highlighter for something like a text editor,
//Using the synoptic and termion crates


use synoptic::{Token, Highlighter};
use termion::color;

const DEMO: &str = r#"
/* hello bool
*/

const text: &str = "Hi there!"; // Hello!

pub fn main() -> bool {
    println!("Hello"); /*
    Synoptic: A very simple "syntax highlighting" crate for rust
    */ return true;
}
"#;

fn main() {
    // Build highlighter
    let mut rust = Highlighter::new();
    rust.add_keywords(&["fn", "let", "return", "pub", "const", "&self", "self", "mut", "&mut"], "keyword");
    rust.add_keywords(&["bool", "&str", "String", "usize", "Vec", "HashMap"], "type");
    rust.add_keywords(&["true", "false"], "boolean");

    rust.add_regex(r"(?m)(//.*)$", "comment").unwrap();
    rust.add_regex("\".*?\"", "string").unwrap();
    rust.add_regex(r"([a-z_][A-Za-z0-9_]*)\s*\(", "identifier").unwrap();
    rust.add_regex(r"([a-z_][A-Za-z0-9_]*!)\s*", "macro").unwrap();

    rust.add_multiline_regex(r"/\*.*?\*/", "comment").unwrap();

    // Run highlighter
    let highlighting = rust.run(DEMO);
    for tok in highlighting {
        match tok {
            Token::Start(kind) => match kind {
                "comment" => print!("{}", color::Fg(color::Black)),
                "string" => print!("{}", color::Fg(color::Green)),
                "keyword" => print!("{}", color::Fg(color::Blue)),
                "type" => print!("{}", color::Fg(color::LightMagenta)),
                "boolean" => print!("{}", color::Fg(color::LightGreen)),
                "identifier" => print!("{}", color::Fg(color::Yellow)),
                "macro" => print!("{}", color::Fg(color::Magenta)),
                _ => (),
            }
            Token::Text(txt) => print!("{}", txt),
            Token::End(_) => print!("{}", color::Fg(color::Reset)),
        }
    }
    println!("");
}
