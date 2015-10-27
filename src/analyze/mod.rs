pub mod multi_env;
pub mod open_bucket;

pub trait Analyzer {
    fn parse(&mut self, line: &str);
    fn print_results(&self);
}
