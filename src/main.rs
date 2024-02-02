use regex::Regex;
use std::io::Result;

#[cfg(not(tarpaulin))]
fn main() -> Result<()> {
    println!("Hello, world!");

    Ok(())
}

struct Context {
    level: i128,
    text: String,
}

fn parse_header_context(context: String) -> Result<Vec<Context>> {
    let return_value = context;
    let asdf = Regex::new(r"^(#+)\s(.*)$").unwrap();

    let mut results = Vec::new();
    for (_, [path, line]) in asdf
        .captures_iter(return_value.as_str())
        .map(|c| c.extract())
    {
        let level = path.to_string().len();

        results.push(Context {
            level: level as i128,
            text: line.to_string(),
        });
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use log::info;

    use super::*;

    fn init_logger() {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::max())
            .is_test(true)
            .try_init();
    }
    #[test]
    fn test_parse_header_context() {
        init_logger();
        info!("test_parse_header_context");
        let context = "## Hello World";
        let result = parse_header_context(context.to_string()).unwrap();

        for context in result {
            assert_eq!(context.level, 2);
            assert_eq!(context.text, "Hello World");
        }
    }
}
