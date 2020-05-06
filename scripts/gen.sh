#!/bin/bash

function render() {
	local document=$1

	scripts/include_filter.py "documents/$document.md" | pandoc \
		--highlight-style "assets/code.theme" \
		--from markdown --to pdf --standalone --pdf-engine=xelatex \
		--output "pdfs/$document.pdf"
}

and_open=${2:-1}
if [ "$and_open" -ne 0 ]; then 
	render "$1" && open "pdfs/$1.pdf"
else
	render "$1"
fi