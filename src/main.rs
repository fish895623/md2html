use log::info;
use std::io::Result;

#[cfg(not(tarpaulin))]
fn main() -> Result<()> {
    println!("Hello, world!");

    Ok(())
}

fn parse_header_context(context: String) -> Result<String> {
    info!("Parsing header context: {}", context);
    let return_value = context;

    Ok(return_value)
}

#[cfg(test)]
mod main_test {
    use super::*;

    #[test]
    fn test_parse_header_context() {
        let context = "## Hello World";
        let result = parse_header_context(context.to_string()).unwrap();
        assert_eq!(result, "Hello World");
    }
}
