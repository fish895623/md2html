#include "hello.hpp"
#include <gtest/gtest.h>

using namespace std;

TEST(Parsing, HeaderStringParsing) {
  string result0 = parse_header("# World");
  string result1 = parse_header("# Hello World");

  EXPECT_EQ(result0, "World");
  EXPECT_EQ(result1, "Hello World");
}
TEST(Parsing, HeaderStringParsingMultipleLevel) {
  string result0 = parse_header("## World");

  EXPECT_EQ(result0, "World");
}

TEST(Parsing, HeaderStringToHtml) {
  Context ctx;
  ctx.context = parse_header("## World Hello");
  ctx.level = parsing_header_level("## World Hello");
  string result = convert_header_to_html(ctx.context, ctx.level);

  EXPECT_EQ(ctx.context, "World Hello");
  EXPECT_EQ(ctx.level, 2);
  EXPECT_EQ(result, "<h2>World Hello</h2>");
}

TEST(Parsing, HeaderStringToHtmlMultipleLevel) {
  Context ctx;
  ctx.context = parse_header("## World");
  ctx.level = parsing_header_level("## World");
  string result = convert_header_to_html(ctx.context, ctx.level);

  EXPECT_EQ(ctx.context, "World");
  EXPECT_EQ(ctx.level, 2);
  EXPECT_EQ(result, "<h2>World</h2>");
}

TEST(Parsing, HeaderLevel) {
  auto val = parsing_header_level("## Hello World");
  EXPECT_EQ(val, 2);
}


TEST(Parsing, FindAllMarkdownFilesInPaths) {
  find_all_markdown_files_in_paths(".");
}