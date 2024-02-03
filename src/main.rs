use regex::Regex;
use std::io::Result;
use std::path::Path;
use walkdir::WalkDir;

#[cfg(not(tarpaulin))]
fn main() -> Result<()> {
  let all_files_in_directory = find_all_markdown_recursively_in_path(".")?;
  for file in all_files_in_directory {
    println!("file: {}", file);
  }

  Ok(())
}

struct Context {
  level: i128,
  text: String,
}

fn parse_header_context(context: String) -> Result<Vec<Context>> {
  let return_value: String = context;
  let header_regex: Regex = Regex::new(r"^(#+)\s(.*)$").unwrap();

  let mut results = Vec::new();
  for (_, [path, line]) in header_regex
    .captures_iter(return_value.as_str())
    .map(|c| c.extract())
  {
    results.push(Context {
      level: path.to_string().len() as i128,
      text: line.to_string(),
    });
  }

  Ok(results)
}

fn find_all_markdown_recursively_in_path(path: &str) -> Result<Vec<String>> {
  let root_path = Path::new(&path);
  let markdown_pattern = Regex::new(r".*\.md$").unwrap();
  let mut return_value = Vec::new();

  for entry in WalkDir::new(root_path).into_iter().filter_map(|e| e.ok()) {
    if entry.file_type().is_file() {
      if let Some(file_name) = entry.file_name().to_str() {
        if markdown_pattern.is_match(file_name) {
          let file_path = entry.path().to_str().unwrap().to_owned();
          return_value.push(file_path);
        }
      }
    }
  }

  Ok(return_value)
}

#[cfg(test)]
mod main_tests {
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
  #[test]
  fn test_find_all_files_in_path() {
    init_logger();
    let result = find_all_markdown_recursively_in_path(".").unwrap();
    for file in &result {
      info!("file: {}", file);
    }
    assert_eq!(result.len(), 2);
  }
  #[test]
  fn test_find_all_files_in_path_with_no_files() {
    init_logger();
    let result = find_all_markdown_recursively_in_path(".").unwrap();
    for file in &result {
      info!("file: {}", file);
    }
    assert_eq!(result.len(), 2);
  }
}
