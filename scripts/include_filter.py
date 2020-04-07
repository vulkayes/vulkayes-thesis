#!/usr/bin/python3

import sys
import re

INCLUDE_RE = re.compile("!\[\]\(([^).]+)\.md\)")

def filter_include(lines):
	for line in lines:
		match = INCLUDE_RE.match(line)
		if match is not None:
			with open("documents/" + match.group(1) + ".md", "r") as sub:
				filter_include(sub)
		else:
			print(line, end = "")
	print()

with open(sys.argv[1], "r") as file:
	filter_include(file)
