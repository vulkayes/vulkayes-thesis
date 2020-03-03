#!/bin/sh

pandoc \
	--highlight-style "misc/code.theme" \
	--from markdown --to pdf --standalone "documents/$1.md" --output "pdfs/$1.pdf"

open "pdfs/$1.pdf"
