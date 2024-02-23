use reductivesearch::reductivesearch;
use std::{
    cmp::Ordering,
    io::{stdin, stdout, Stdout, Write},
};
use termion::{
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

struct SelectionView {
    selection_searcher: reductivesearch::Searcher,
    screen: Vec<Option<String>>,
    output: RawTerminal<Stdout>,
}

impl SelectionView {
    fn new(selection_searcher: reductivesearch::Searcher, output: RawTerminal<Stdout>) -> Self {
        Self {
            selection_searcher,
            screen: vec![Some(String::new()); 5],
            output,
        }
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
            let line_string  = line.as_ref().unwrap_or(&clearing_string);
            match index.cmp(&3) {
                Ordering::Less => {
                    write!(
                        &mut self.output,
                        "\n{clearing_string}{}{}{line_string}{}{}",
                        termion::cursor::Left(term_width),
                        termion::color::Fg(termion::color::Blue),
                        termion::cursor::Left(term_width),
                        termion::color::Fg(termion::color::Reset)
                    )
                    .expect("should be able to write to stdout");
                }
                Ordering::Equal => {
                    write!(
                        self.output,
                        "\n{clearing_string}{}",
                        termion::cursor::Left(term_width)
                    )
                    .expect("should be able to write to stdout");
                }
                Ordering::Greater => {
                    write!(
                        &mut self.output,
                        "\n{clearing_string}{}{}{line_string}{}{}",
                        termion::cursor::Left(term_width),
                        termion::color::Fg(termion::color::Red),
                        termion::cursor::Left(term_width),
                        termion::color::Fg(termion::color::Reset)
                    )
                    .expect("should be able to write to stdout");
                }
            }
        }
        write!(
            &mut self.output,
            "{}{clearing_string}{}{}",
            termion::cursor::Up(1),
            termion::cursor::Left(term_width),
            self.screen[3].clone().expect("filter should be a string")
        )
        .expect("should be able to write to stdout");
    }

    fn input_loop(&mut self) {
        let (term_width, _) =
            termion::terminal_size().expect("should be able to get terminal width");
        let stdin = stdin();

        self.printinfo();
        self.output.flush().expect("should be able to flush stdout");

        for input_key in stdin.keys() {
            self.screen[4] = None;
            match input_key.expect("should be a standard character") {
                Key::Char('\n') => {
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
                            self.screen[3] = Some(search_string);
                        }
                        Err(reductivesearch::SearcherError::NoneFound(character)) => {
                            self.screen[4] = Some(format!("Can't add character: '{character}'"));
                        }
                        Err(error) => panic!("error encountered: {error}"),
                    }
                }
                Key::Backspace | Key::Delete => {
                    self.screen[3] = Some(self.selection_searcher.remove_search_character());
                }
                _ => continue,
            }
            let mut results = self.selection_searcher.search_results();
            results.truncate(3);
            for index in results.len()..=2 {
                self.screen[index] = None;
            }
            for (index, line) in results.iter().enumerate() {
                self.screen[index] = Some(line.clone());
            }
            self.printinfo();
            self.output.flush().expect("should be able to flush stdout");
        }
    }
}

fn main() {
    println!("Enter will quit the program.\n\n\n");
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
    view_handler.input_loop();
}
