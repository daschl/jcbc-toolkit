pub mod multi_env;

pub trait Analyzer {
    fn parse(&mut self, line: &str);
    fn print_results(&self);
}
