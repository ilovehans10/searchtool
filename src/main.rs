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
        let (term_width, _) =
            termion::terminal_size().expect("should be able to get terminal width");
        let clearing_string: String = " ".repeat(term_width.into());
        write!(
            &mut self.output,
            "{}{}",
            termion::cursor::Left(term_width),
            termion::cursor::Up(4)
        )
        .expect("should be able to write to stdout");

        for (index, line) in self.screen.iter().enumerate() {
            if index == 3 {
                write!(
                    self.output,
                    "{}{clearing_string}{}",
                    termion::cursor::Down(1),
                    termion::cursor::Left(term_width)
                )
                .expect("should be able to write to stdout");
            } else {
                write!(
                    &mut self.output,
                    "{}{clearing_string}{}{line}{}",
                    termion::cursor::Down(1),
                    termion::cursor::Left(term_width),
                    termion::cursor::Left(term_width)
                )
                .expect("should be able to write to stdout");
            }
        }
        write!(
            &mut self.output,
            "{}{clearing_string}{}{}",
            termion::cursor::Up(1),
            termion::cursor::Left(term_width),
            self.screen[3]
        )
        .expect("should be able to write to stdout");
    }

    fn input_loop(&mut self) {
        let (term_width, _) =
            termion::terminal_size().expect("should be able to get terminal width");
        let clearing_string: String = " ".repeat(term_width.into());
        let stdin = stdin();

        for input_key in stdin.keys() {
            self.screen[4] = clearing_string.clone();
            match input_key.expect("should be a standard character") {
                Key::Char('q') => {
                    writeln!(
                        &mut self.output,
                        "{}{}\n{:?}{}",
                        termion::cursor::Down(1),
                        termion::cursor::Left(term_width),
                        self.selection_searcher.search_results(),
                        termion::cursor::Left(term_width)
                    )
                    .expect("should be able to write to stdout");
                    break;
                }
                Key::Char(character) => {
                    match self.selection_searcher.add_search_character(character) {
                        Ok(search_string) => {
                            self.screen[3] = search_string;
                        }
                        Err(reductivesearch::SearcherError::NoneFound(character)) => {
                            self.screen[4] = format!("Can't add character: '{character}'");
                        }
                        Err(error) => panic!("error encountered: {error}"),
                    }
                }
                Key::Backspace | Key::Delete => {
                    self.screen[3] = self.selection_searcher.remove_search_character();
                }
                _ => continue,
            }
            let mut results = self.selection_searcher.search_results();
            results.truncate(3);
            for (index, line) in results.iter().enumerate() {
                self.screen[index] = line.clone();
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
        String::from("Long Fall Boots"),
        String::from("Web Slinger"),
        String::from("Modular Web Slinger"),
        String::from("Hither Thither Staff"),
    ]);
    let mut view_handler = SelectionView::new(toolsearcher, stdout);
    view_handler.setup();
    view_handler.input_loop();
}
