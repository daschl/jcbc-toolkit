extern crate jcbc_toolkit;

use std::fs::File;
use std::process;
use std::io::BufReader;
use std::io::BufRead;

use jcbc_toolkit::parse::Parser;

fn main() {

	let file = "foo.txt";

	let f = match File::open(file) {
		Ok(f) => f,
		Err(e) => {
			println!("{}", e);
			process::exit(-1);
		}
	};

	let reader = BufReader::new(f);

	let parser = Parser::new();
	parser.parse(reader.lines());

}