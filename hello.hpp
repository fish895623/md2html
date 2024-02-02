#pragma once
#include <string>

struct Context {
  std::string context;
  int level;
};

std::string parse_header(const std::string &context);
std::string convert_header_to_html(const std::string &context,
                                   const int level = 1);
/** @brief Parse the header level
 *
 * @param context The string to parse
 * @return The level of the header
 */
int parsing_header_level(const std::string &context);
