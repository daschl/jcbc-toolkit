use regex::Regex;
use analyze::Analyzer;

pub struct MultiEnvAnalyzer {
    occurences: u8,
    max_envs_found: u8,
}

impl MultiEnvAnalyzer {

    pub fn new() -> MultiEnvAnalyzer {
        MultiEnvAnalyzer { max_envs_found: 0, occurences: 0 }
    }

}

impl Analyzer for MultiEnvAnalyzer {

    fn parse(&mut self, line: &str) {
        let re = Regex::new(r"More than 1 Couchbase Environments found \((\d+)\)").unwrap();

        if re.is_match(line) {
            self.occurences = self.occurences + 1;

            let caps = re.captures(line).unwrap();
            let num_found = caps.at(1).unwrap().parse::<u8>().unwrap();

            if num_found > self.max_envs_found {
                self.max_envs_found = num_found;
            }
        }
    }

    fn print_results(&self) {
        println!("[MultiEnvAnalyzer]: {} Occurences, {} Max Environments Reported",
                 self.occurences,
                 self.max_envs_found);
    }

}
