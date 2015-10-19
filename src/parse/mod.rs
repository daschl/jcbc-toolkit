use std::io::Lines;
use std::io::Result;

pub struct Parser;

impl Parser {

	pub fn new() -> Parser {
		Parser
	}

	pub fn parse<I>(&self, iter: I) where I: Iterator<Item=Result<String>> {
		for i in iter {
			println!("{}", i.ok().unwrap());
		}
	}

}