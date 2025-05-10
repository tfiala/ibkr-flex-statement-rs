mod fixtures;

use anyhow::Result;
use fixtures::*;
use ibkr_flex_statement::Parser;
use rstest::rstest;
use std::path::PathBuf;

#[rstest]
#[test]
fn xml_files_parse_correctly(sample_statement_paths: Result<Vec<PathBuf>>) {
    let parser = Parser::new().unwrap();

    for path in sample_statement_paths.unwrap() {
        let xml_content = std::fs::read_to_string(path).expect("Failed to read XML file");
        parser
            .parse_flex_query_response(&xml_content)
            .expect("Failed to parse XML content");
    }
}
