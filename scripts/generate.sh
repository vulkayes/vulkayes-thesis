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

section 'Deleting old thesis-output.pdf..'
rm 'pdfs/thesis-output.pdf'

section 'Linking latex class..'
lfblis ctuthesis.cls
lfblis ctuth-core.tex
lfblis ctuth-names.tex
lfblis ctuth-pkg.tex
lfblis ctuth-templates.tex
lfblis ctu_logo_black.pdf

section 'Generating thesis-output.pdf..'
scripts/include_filter.py 'documents/thesis.md' | pandoc \
	--from markdown --to latex --standalone --pdf-engine 'xelatex' --template 'assets/template.latex' \
	\
	--include-in-header 'assets/header.tex' \
	--include-before 'documents/title.tex' \
	--include-before 'documents/thanks.tex' --include-before 'documents/abstract.tex' \
	\
	--filter 'pandoc-crossref' --filter 'pandoc-citeproc' \
	--output 'pdfs/thesis-output.pdf'

section 'Unlinking latex class..'
ufblis ctuthesis.cls
ufblis ctuth-core.tex
ufblis ctuth-names.tex
ufblis ctuth-pkg.tex
ufblis ctuth-templates.tex
ufblis ctu_logo_black.pdf

section 'Opening thesis-output.pdf..'
open 'pdfs/thesis-output.pdf'