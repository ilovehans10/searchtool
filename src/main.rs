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

// SelectionView is a singleton that controls the Searcher and its associated data
struct SelectionView {
    selection_searcher: reductivesearch::Searcher,
    screen: Vec<Option<String>>,
    output: RawTerminal<Stdout>,
}

impl SelectionView {
    // Creates a new selection_searcher with an empty screen
    fn new(selection_searcher: reductivesearch::Searcher, output: RawTerminal<Stdout>) -> Self {
        Self {
            selection_searcher,
            screen: vec![Some(String::new()); 5],
            output,
        }
    }

    // printinfo prints all lines of selected items, the users input, and any errors
    fn printinfo(&mut self) {
        // The clearing string is used for clearing the old data out of a line
        let (term_width, _) =
            termion::terminal_size().expect("should be able to get terminal width");
        let clearing_string: String = " ".repeat(term_width.into());

        // This moves the cursor into position at the top left before printing info
        write!(
            &mut self.output,
            "{}{}",
            termion::cursor::Left(term_width),
            termion::cursor::Up(4)
        )
        .expect("should be able to write to stdout");

        for (index, line) in self.screen.iter().enumerate() {
            // This either returns a reference to the string inside the option or a reference to
            // the clearing string
            let line_string = line.as_ref().unwrap_or(&clearing_string);
            match index.cmp(&3) {
                // indexes less than three are the strings filtered by the searcher
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
                // Index three is the search string, but only the clearing string is printed as the
                // search string is the last thing printed
                Ordering::Equal => {
                    write!(
                        self.output,
                        "\n{clearing_string}{}",
                        termion::cursor::Left(term_width)
                    )
                    .expect("should be able to write to stdout");
                }
                // Indexes greater than three are where errors are stored
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
        // The search string is printed last so that the cursor is positioned properly at the end
        // of it
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

            // results is a vec of all filtered strings which is then shortened to just 3 items
            let mut results = self.selection_searcher.search_results();
            results.truncate(3);
            // This loop clears old data out of the list indexes that aren't going to have strings
            for index in results.len()..=2 {
                self.screen[index] = None;
            }
            for (index, line) in results.iter().enumerate() {
                self.screen[index] = Some(line.clone());
            }
            self.printinfo();
            // Flushing the stdout only at the end of the loop makes sure that all information is
            // printed at the same time
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
