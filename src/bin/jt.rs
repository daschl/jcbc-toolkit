extern crate jcbc_toolkit;

use std::fs::File;
use std::process;
use std::io::BufReader;
use std::io::BufRead;

use std::env;

use jcbc_toolkit::parse::Parser;
use jcbc_toolkit::analyze::Analyzer;
use jcbc_toolkit::analyze::multi_env::MultiEnvAnalyzer;
use jcbc_toolkit::analyze::open_bucket::OpenBucketAnalyzer;

#[cfg_attr(test, allow(dead_code))]
fn main() {
	let args = env::args().collect::<Vec<String>>();

	if args.len() != 2 {
		println!("I need a filename :(");
		process::exit(-1);
	}

	let file = &args[1];

	let f = match File::open(file) {
		Ok(f) => f,
		Err(e) => {
			println!("{}", e);
			process::exit(-1);
		}
	};

	let reader = BufReader::new(f);

	let mut parser = Parser::new();
	parser.register_analyzer(Box::new(MultiEnvAnalyzer::new()));
	parser.register_analyzer(Box::new(OpenBucketAnalyzer::new()));

	parser.parse(reader.lines());
	parser.print_results();
}