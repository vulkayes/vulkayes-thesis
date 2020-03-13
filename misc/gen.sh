#!/bin/sh

function include_filter() {
	python3 -c '
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

with open(sys.argv[1], "r") as file:
	filter_include(file)
' "$@"
}

function render() {
	local document=$1

	include_filter "documents/$document.md" | pandoc \
		--highlight-style "misc/code.theme" --css "misc/style.css" \
		--from markdown --to pdf --standalone --pdf-engine=xelatex \
		--output "pdfs/$document.pdf"
}

and_open=${2:-1}
if [ "$and_open" -ne 0 ]; then 
	render "$1" && open "pdfs/$1.pdf"
else
	render "$1"
fi