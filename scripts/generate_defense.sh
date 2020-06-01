#!/bin/bash

PATH="/Library/TeX/texbin:$PATH"


function section() {
	printf '\n>> %s\n' "$1"
}

section 'Deleting old defense-output.pdf..'
rm 'pdfs/defense-output.pdf'

section 'Generating defense-output.pdf..'
scripts/include_filter.py 'documents/defense.md' | pandoc \
	--from markdown --to beamer --standalone --pdf-engine 'xelatex' \
	--highlight-style "assets/code.theme" \
	\
	--include-in-header 'assets/header-defense.tex' \
	\
	--output 'pdfs/defense-output.pdf'

section 'Opening defense-output.pdf..'
open 'pdfs/defense-output.pdf'