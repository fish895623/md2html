#pragma once
#include <string>
#include <vector>

using namespace std;

struct Context {
  string context;
  int level;
};

string parse_header(const string &context);
string convert_header_to_html(const string &context, int level = 1);
/** @brief Parse the header level
 *
 * @param context The string to parse
 * @return The level of the header
 */
int parsing_header_level(const string &context);

/**
 * @brief Find all markdown files in path
 */
vector<string> find_all_markdown_files_in_paths(const string &path,
                                                vector<string> exclude);
