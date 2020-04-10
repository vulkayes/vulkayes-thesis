#!/bin/bash

PATH="/Library/TeX/texbin:$PATH"

# link file because latex is stupid
function lfblis() {
	ln -s "assets/ctuthesis/$1" "$1"
}
# unlink file because latex is stupid
function ufblis() {
	rm "$1"
}

function section() {
	printf '\n>> %s\n' "$1"
}

section 'Deleting old {thanks, abstract}.tex files and thesis-output.pdf..'
rm 'documents/thanks.tex'
rm 'documents/abstract.tex'
rm 'pdfs/thesis-output.pdf'

section 'Generating thanks.tex and abstract.tex..'
pandoc 'documents/thanks.md' --from markdown --to latex --output 'documents/thanks.tex'
pandoc 'documents/abstract.md' --from markdown --to latex --output 'documents/abstract.tex'

section 'Generating thesis-output.pdf..'
scripts/include_filter.py 'documents/thesis.md' | pandoc \
	--from markdown --to latex --standalone --pdf-engine 'xelatex' --template 'assets/template.latex' \
	--highlight-style "assets/code.theme" \
	\
	--include-in-header 'assets/header.tex' --include-in-header 'assets/title.tex' \
	--include-before 'documents/thanks.tex' --include-before 'documents/abstract.tex' \
	\
	--filter 'pandoc-crossref' --filter 'pandoc-citeproc' \
	--number-sections --table-of-contents \
	--output 'pdfs/thesis-output.pdf'

section 'Opening thesis-output.pdf..'
open 'pdfs/thesis-output.pdf'