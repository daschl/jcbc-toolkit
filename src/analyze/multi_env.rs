use regex::Regex;
use analyze::Analyzer;

pub struct MultiEnvAnalyzer {
    occurences: u8,
    max_envs_found: u8,
    matcher_regex: Regex
}

impl MultiEnvAnalyzer {

    pub fn new() -> MultiEnvAnalyzer {
        MultiEnvAnalyzer { 
            max_envs_found: 0, 
            occurences: 0, 
            matcher_regex: Regex::new(r"More than 1 Couchbase Environments found \((\d+)\)").unwrap() 
        }
    }

}

impl Analyzer for MultiEnvAnalyzer {

    fn parse(&mut self, line: &str) {

        if self.matcher_regex.is_match(line) {
            self.occurences = self.occurences + 1;

            let caps = self.matcher_regex.captures(line).unwrap();
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

#[cfg(test)]
mod tests {

    use super::*;
    use analyze::Analyzer;

    #[test]
    fn should_match_line() {
        let mut analyzer = MultiEnvAnalyzer::new();

        assert_eq!(0, analyzer.max_envs_found);
        assert_eq!(0, analyzer.occurences);

        let not_matching_line = "This is some log line which should not match.";
        analyzer.parse(not_matching_line);

        assert_eq!(0, analyzer.max_envs_found);
        assert_eq!(0, analyzer.occurences);

        let matching_line = "2015-10-27 19:37:23 WARN  CoreEnvironment:204 - More than 1 \
                             Couchbase Environments found (2), this can have severe impact on \
                             performance and stability. Reuse environments!";
        analyzer.parse(matching_line);

        assert_eq!(2, analyzer.max_envs_found);
        assert_eq!(1, analyzer.occurences);

        let another_matching_line = "2015-10-27 21:37:23 WARN  CoreEnvironment:204 - More than 1 \
                                     Couchbase Environments found (3), this can have severe \
                                     impact on performance and stability. Reuse environments!";
        analyzer.parse(another_matching_line);

        assert_eq!(3, analyzer.max_envs_found);
        assert_eq!(2, analyzer.occurences);
    }

}
