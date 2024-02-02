#include "hello.hpp"

#include <iostream>
#include <regex>
#include <string>

using namespace std;

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
