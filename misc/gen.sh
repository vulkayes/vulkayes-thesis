#!/bin/sh

pandoc \
	--highlight-style "misc/code.theme" --css "misc/style.css" \
	--from markdown --to pdf --standalone --pdf-engine=xelatex \
	"documents/$1.md" --output "pdfs/$1.pdf" \
&& open "pdfs/$1.pdf"
