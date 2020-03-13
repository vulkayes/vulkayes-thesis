#!/usr/bin/python3

import sys
import json
import bs4
import os.path

import time

PARSER = "lxml"

VAL = "\\val"
COMMENT = "\\valcom"
DONE = "\\valdone"

def end_comment_block(stream, running_block):
	if running_block == "comment":
		print(f"{COMMENT}boxend", file = stream, end = "\n\n")
	elif running_block == "done":
		print(f"{DONE}boxend", file = stream, end = "\n\n")

def build_comment_line(type, comment):
	if type == "comment":
		return f"	- {COMMENT} {comment}"
	elif type == "done":
		return f"	- {DONE} {comment}"

def handle_comment(stream, line_buffer, running_block, comments, index):
	if index not in comments:
		end_comment_block(stream, running_block)

		print(line_buffer, file = stream, end = "\n\n")
		return None

	comment = comments[index]
	if running_block is not None and running_block != comment[0]:
		end_comment_block(stream, running_block)

	if running_block != comment[0]:
		if comment[0] == "comment":
			print(f"{COMMENT}box", file = stream, end = "\n\n")
		elif comment[0] == "done":
			print(f"{DONE}box", file = stream, end = "\n\n")
		running_block = comment[0]
	
	print(line_buffer, file = stream, end = "\n")

	print(build_comment_line(comment[0], comment[1]), file = stream, end = "\n\n")

	return running_block

def extract_usage(stream, soup, selector, comments = None):
	ul = soup.select_one(selector)

	print(f"{VAL}box", file = stream, end = "\n\n")
	running_block = None

	lines = ul.select("li > p")
	for index in range(len(lines)):
		line_buffer = "*"

		line = lines[index]
		for child in line.children:
			if child.name is not None:
				child_str = "".join(child.stripped_strings).replace("\n", " ")
				
				if child.name == "a":
					if child_str != "":
						if child_str.find(" ") == -1:
							line_buffer += f"`{child_str}`"
						else:
							line_buffer += child_str
				elif child.name == "strong":
					line_buffer += child_str
				elif child.name == "code":
					line_buffer += f"`{child_str}`"
				elif child.name == "span":
					line_buffer += f"`{child_str}`"
				elif child.name == "em":
					line_buffer += f"_{child_str}_"
				else:
					print("Unexpected child", child.name, child, file = sys.stderr)
			else:
				line_buffer += child
		
		line_buffer = line_buffer.replace("\n", " ")
		# spec fix, shouldn't have to be here
		line_buffer = line_buffer.replace("\\(\\lceil", "$\\lceil").replace("\\rceil\\)", "\\rceil$")

		if comments is not None:
			running_block = handle_comment(stream, line_buffer, running_block, comments, str(index))
		else:
			print(line_buffer, file = stream, end = "\n\n")
	
	if running_block is not None:
		end_comment_block(stream, running_block)
	print(f"{VAL}boxend", file = stream, end = "\n\n")

def process_section(stream, soup, name, section, comments = None):
	print(">", name, file = sys.stderr)
	
	for entry in section:
		if isinstance(entry, str):
			print(f"### {entry}", file = stream, end = "\n\n")
		else:
			entry_comments = None
			if comments is not None and entry[0] in comments:
				entry_comments = comments[entry[0]]

			print(f"Validations for `{entry[0]}`:", file = stream, end = "\n\n")
			
			print("  -", entry[0], file = sys.stderr)
			extract_usage(stream, soup, entry[1], entry_comments)

def main():
	if len(sys.argv) < 4:
		print(f"usage: {sys.argv[0]} structure comments spec out")
		exit(1)
	
	structure_path = sys.argv[1]
	comments_path = sys.argv[2]
	spec_path = sys.argv[3]
	out_path = sys.argv[4]
	
	structure = None
	comments = None
	soup = None

	with open(structure_path, "r") as file:
		structure = json.load(file)
	
	with open(comments_path, "r") as file:
		comments = json.load(file)

	print("Loading spec...", file = sys.stderr)
	time_before = time.perf_counter()
	with open(spec_path, "r") as file:
		soup = bs4.BeautifulSoup(file.read(), features = PARSER)
	print("Loaded spec in", time.perf_counter() - time_before, file = sys.stderr)

	for name in structure:
		section_comments = None
		if name in comments:
			section_comments = comments[name]

		print("\n###", name, "###", file = sys.stderr, end = "\n\n")
		with open(os.path.join(out_path, name + ".md"), "w") as file:
			process_section(file, soup, name, structure[name], section_comments)

main()