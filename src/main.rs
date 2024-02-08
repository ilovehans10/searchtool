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
    toolsearcher
        .add_search_character('e')
        .expect("should be able to search with an e");
    for input_key in stdin.keys() {
        match input_key.expect("should be a standard character") {
            Key::Char('q') => break,
            Key::Char(character) => write!(stdout, "{character}").unwrap(),
            Key::Backspace | Key::Delete => toolsearcher.remove_search_character(),
                //toolsearcher.add_search_character(character),
            _ => continue,
        }
        stdout.flush().unwrap();
    }
    write!(stdout, "Searcher: {:?}", toolsearcher.search_results()).unwrap();
}
