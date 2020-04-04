#!/bin/bash

PATH="/Library/TeX/texbin:$PATH"

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

# link file because latex is stupid
function lfblis() {
	ln -s "assets/ctuthesis/$1" "$1"
}
# unlink file because latex is stupid
function ufblis() {
	rm "$1"
}

lfblis ctuthesis.cls
lfblis ctuth-core.tex
lfblis ctuth-names.tex
lfblis ctuth-pkg.tex
lfblis ctuth-templates.tex
lfblis ctu_logo_black.pdf

echo '>> Generating thesis-output.pdf..'
include_filter 'documents/thesis.md' | pandoc  \
    --from markdown --to latex --standalone --template "assets/template.latex"\
	\
    --filter 'pandoc-crossref' --filter 'pandoc-citeproc' \
	--metadata 'csl:assets/ieee-with-url.csl' --metadata 'link-citations:true' \
	--metadata 'bibliography:assets/bibliography.json' --metadata 'reference-section-title:Bibliography' \
	\
	--top-level-division 'chapter' \
    --include-in-header 'assets/header.tex' --include-in-header 'assets/abstract.tex' \
    --output 'pdfs/thesis-output.pdf'

ufblis ctuthesis.cls
ufblis ctuth-core.tex
ufblis ctuth-names.tex
ufblis ctuth-pkg.tex
ufblis ctuth-templates.tex
ufblis ctu_logo_black.pdf

echo '>> Opening thesis-output.pdf..'
open 'pdfs/thesis-output.pdf'