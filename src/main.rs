use reductivesearch::reductivesearch;
use std::io::{stdin, stdout, Write};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

fn main() {
    let mut stdout = stdout()
        .into_raw_mode()
        .expect("should be able to put terminal into raw mode");
    let stdin = stdin();
    let mut toolsearcher = reductivesearch::Searcher::new(vec![
        String::from("Hev Suit"),
        String::from("Portal Gun"),
        String::from("Grappling Tool"),
        String::from("Elucidator"),
    ]);

    write!(stdout, "\n\n\n\n\n").expect("should be able to make newlines");

    for input_key in stdin.keys() {
        match input_key.expect("should be a standard character") {
            Key::Char('q') => break,
            Key::Char(character) => {
                match toolsearcher.add_search_character(character) {
                    Ok(search_string) => write!(stdout, "{}{search_string}", termion::cursor::Left(99)),
                    Err(_) => write!(stdout, "can't add that"),
                }
                .expect("should be able to write");
            }
            Key::Backspace | Key::Delete => toolsearcher.remove_search_character(),
            _ => continue,
        }
        stdout.flush().expect("should be able to flush stdout");
    }
    write!(stdout, "Searcher: {:?}", toolsearcher.search_results()).unwrap();
}
