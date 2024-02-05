import logging
import os
import os.path
import re
from logging import getLogger

import markdown

logging.basicConfig(level=logging.DEBUG)
log = getLogger("pymd2html.hello")


def find_all_markdowns_recursively(path=".", exclude: str = ""):
    markdowns = []
    for root, _, files in os.walk(top=path):
        for file in files:
            if file.endswith(".md"):
                markdowns.append(os.path.join(root, file))
    # Exclude with regex
    if exclude == "":
        pass
    else:
        p = re.compile(exclude)
        # if markdown matchs the regex, remove it from the list
        markdowns = [i for i in markdowns if not p.match(i)]

    return markdowns


if __name__ == "__main__":
    a = find_all_markdowns_recursively(exclude="./test/exclude*")

    for i in a:
        markdown.markdownFromFile(input=i, output=i.replace(".md", ".html"))
        log.info(i)
