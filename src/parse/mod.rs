use std::io::Result;

use analyze::Analyzer;

pub struct Parser {
    analyzers: Vec<Box<Analyzer>>,
}

impl Parser {

    pub fn new() -> Parser {
        Parser { analyzers: Vec::new() }
    }

    pub fn register_analyzer(&mut self, analyzer: Box<Analyzer>) {
        self.analyzers.push(analyzer);
    }

    pub fn parse<I>(&mut self, iter: I)
        where I: Iterator<Item = Result<String>>
    {
        for i in iter {
            let line = i.ok().unwrap();

            for a in &mut self.analyzers {
                a.parse(&line);
            }
        }
    }

    pub fn print_results(&self) {
        for a in &self.analyzers {
            a.print_results();
        }
    }

}
