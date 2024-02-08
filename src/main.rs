use reductivesearch::reductivesearch;
use termion::input;

fn main() {
    let mut toolsearcher = reductivesearch::Searcher::new(vec![String::from("Hev Suit"), String::from("Portal Gun"), String::from("Grappling Tool"), String::from("Elucidator") ]);
    toolsearcher.add_search_character('e').expect("should be able to search with an e");
    dbg!("Searcher: {}", toolsearcher.search_results());
}
