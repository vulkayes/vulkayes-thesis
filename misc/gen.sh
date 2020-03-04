#!/bin/sh

function render() {
	local document=$1

	pandoc \
		--highlight-style "misc/code.theme" --css "misc/style.css" \
		--from markdown --to pdf --standalone --pdf-engine=xelatex \
		"documents/$document.md" --output "pdfs/$document.pdf"
}

and_open=${2:-1}
if [ "$and_open" -ne 0 ]; then 
	render "$1" && open "pdfs/$1.pdf"
else
	render "$1"
fi