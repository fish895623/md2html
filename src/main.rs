use regex::Regex;
use std::io::Result;

#[cfg(not(tarpaulin))]
fn main() -> Result<()> {
    println!("Hello, world!");

    Ok(())
}

fn parse_header_context(context: String) -> Result<Vec<(String, String)>> {
    let return_value = context;
    let asdf = Regex::new(r"^#+\s(.*)$").unwrap();

    let mut results = Vec::new();
    for (_, [path, line]) in asdf
        .captures_iter(return_value.as_str())
        .map(|c| c.extract())
    {
        results.push((path.to_string(), line.to_string()));
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::info;

    fn init_logger() {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::max())
            .is_test(true)
            .try_init();
    }
    #[test]
    fn test_parse_header_context() {
        init_logger();

        info!("Hello World");
        let context = "## Hello World";
        let result = parse_header_context(context.to_string()).unwrap();

        for (path, line) in result {
            info!("Path: {}, Line: {}", path, line);
        }
    }
}
