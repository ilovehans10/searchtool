use reductivesearch::reductivesearch;
use std::io::{stdin, stdout, Stdout, Write};
use termion::{
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

struct SelectionView {
    selection_searcher: reductivesearch::Searcher,
    screen: Vec<String>,
    output: RawTerminal<Stdout>,
}

impl SelectionView {
    fn new(selection_searcher: reductivesearch::Searcher, output: RawTerminal<Stdout>) -> Self {
        Self {
            selection_searcher,
            screen: vec![String::new(); 5],
            output,
        }
    }

    fn setup(&mut self) {
        write!(self.output, "\n\n\n\n\n").expect("should be able to write to stdout");
    }

    fn printinfo(&mut self) {
        write!(&mut self.output, "{}", termion::cursor::Up(5))
            .expect("should be able to write to stdout");
        for line in &self.screen {
            write!(
                &mut self.output,
                "{}{line}{}",
                termion::cursor::Down(1),
                termion::cursor::Left(99)
            )
            .expect("should be able to write to stdout");
        }
    }

    fn input_loop(&mut self) {
        let (term_width, _) =
            termion::terminal_size().expect("should be able to get terminal width");
        let clearing_string: String = " ".repeat(term_width.into());
        let stdin = stdin();
        self.screen[1] = String::from("hello");

        for input_key in stdin.keys() {
            self.screen[4] = clearing_string.clone();
            match input_key.expect("should be a standard character") {
                Key::Char('q') => break,
                Key::Char(character) => {
                    match self.selection_searcher.add_search_character(character) {
                        Ok(search_string) => self.screen[3] = search_string,
                        Err(reductivesearch::SearcherError::NoneFound(character)) => {
                            self.screen[4] = format!("Can't add character: '{character}'");
                        }
                        Err(error) => panic!("error encountered: {error}"),
                    }
                }
                Key::Backspace | Key::Delete => self.selection_searcher.remove_search_character(),
                _ => continue,
            }
            self.printinfo();
            self.output.flush().expect("should be able to flush stdout");
        }
    }
}

fn main() {
    let stdout = stdout()
        .into_raw_mode()
        .expect("should be able to put terminal into raw mode");
    let toolsearcher = reductivesearch::Searcher::new(vec![
        String::from("Hev Suit"),
        String::from("Portal Gun"),
        String::from("Grappling Tool"),
        String::from("Elucidator"),
    ]);
    let mut view_handler = SelectionView::new(toolsearcher, stdout);
    view_handler.setup();
    view_handler.input_loop();
}
