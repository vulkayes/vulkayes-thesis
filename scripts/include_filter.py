#!/usr/bin/python3

import sys
import re
import os.path

INCLUDE_RE = re.compile("!\[\]\(([^).]+)\.md\)")

def filter_include(document_path):
	current_dir = os.path.dirname(document_path)
	with open(document_path, "r") as file:
		for line in file:
			match = INCLUDE_RE.match(line)
			if match is not None:
				print(f"Including file {match.group(1)} (current dir: {current_dir})", file = sys.stderr)
				filter_include(
					os.path.join(
						current_dir,
						match.group(1) + ".md"
					)
				)
			else:
				print(line, end = "")
		print()

filter_include(sys.argv[1])
