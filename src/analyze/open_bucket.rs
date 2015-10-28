use std::collections::HashMap;
use regex::Regex;
use analyze::Analyzer;

pub struct OpenBucketAnalyzer {
    found_buckets: HashMap<String, u32>,
    matcher_regex: Regex
}

impl OpenBucketAnalyzer {

    pub fn new() -> OpenBucketAnalyzer {
        OpenBucketAnalyzer { 
            found_buckets: HashMap::new(), 
            matcher_regex: Regex::new(r"Opened bucket ([0-9A-Za-z_-]+)").unwrap() 
        }
    }

}

impl Analyzer for OpenBucketAnalyzer {

    fn parse(&mut self, line: &str) {
        if self.matcher_regex.is_match(line) {

            let caps = self.matcher_regex.captures(line).unwrap();
            let bucket_name = caps.at(1).unwrap();

            let counter = self.found_buckets.entry(bucket_name.to_string()).or_insert(0);
            *counter += 1;
        }
    }

    fn print_results(&self) {
        println!("[OpenBucketAnalyzer]: TODO - print buckets and their open attempts");
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use analyze::Analyzer;

    #[test]
    fn should_match_line() {
        let mut analyzer = OpenBucketAnalyzer::new();

        assert_eq!(0, analyzer.found_buckets.len());

        let not_matching_line = "This is some log line which should not match.";
        analyzer.parse(not_matching_line);

        assert_eq!(0, analyzer.found_buckets.len());

        let matching_line = "2015-10-27 19:37:23 INFO  ConfigurationProvider:263 - Opened bucket \
                             default";
        analyzer.parse(matching_line);

        assert_eq!(1, analyzer.found_buckets.len());
        assert_eq!(Some(&1), analyzer.found_buckets.get("default"));

        let another_matching_line = "2015-10-27 19:37:23 INFO  ConfigurationProvider:263 - Opened \
                                     bucket travel-sample";
        analyzer.parse(another_matching_line);

        assert_eq!(2, analyzer.found_buckets.len());
        assert_eq!(Some(&1), analyzer.found_buckets.get("travel-sample"));

        let same_bucket_again = "2015-10-27 19:37:23 INFO  ConfigurationProvider:263 - Opened \
                                 bucket default";
        analyzer.parse(same_bucket_again);

        assert_eq!(2, analyzer.found_buckets.len());
        assert_eq!(Some(&2), analyzer.found_buckets.get("default"));


    }

}
