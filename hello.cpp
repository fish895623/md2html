#include "hello.hpp"

#include <filesystem>
#include <iostream>
#include <regex>
#include <string>

using namespace std;
namespace fs = std::filesystem;

string parse_header(const string &context) {
  smatch match;
  regex re("^#+ (.*)$");

  regex_match(context, match, re);

  return match[1];
}
string convert_header_to_html(const string &context, const int level) {
  auto return_value =
      "<h" + to_string(level) + ">" + context + "</h" + to_string(level) + ">";

  return return_value;
}
int parsing_header_level(const string &context) {
  smatch match;
  regex re("^#+");
  regex_match(context, match, re);

  auto begin = sregex_iterator(context.begin(), context.end(), re);
  auto end = sregex_iterator();

  return begin->str().size();
}

vector<string> find_all_markdown_files_in_paths(const string &path,
                                                const vector<string> exclude) {
  vector<string> files;

  for (const fs::directory_entry &dir_entry :
       fs::recursive_directory_iterator(path)) {
    if (dir_entry.is_regular_file() && dir_entry.path().extension() == ".md") {
      files.push_back(dir_entry.path().string());
    }
  }

  return files;
}
