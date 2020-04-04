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

## Comments and dones
def end_comment_block(stream, running_block):
	if running_block == "comment":
		print(f"{COMMENT}boxend", file = stream, end = "\n\n")
	elif running_block == "done":
		print(f"{DONE}boxend", file = stream, end = "\n\n")

def handle_comment(stream, line_buffer, running_block, comments, index, stats):
	if index not in comments or comments[index] is None:
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

	comment_line = None
	if comment[0] == "comment":
		stats["comment"] += 1
		comment_line = f"	- {COMMENT} {comment[1]}"
	elif comment[0] == "done":
		stats["done"] += 1
		comment_line = f"	- {DONE} {comment[1]}"
	print(comment_line, file = stream, end = "\n\n")

	return running_block

## Section entry and usages
def extract_usage(stream, soup, name, type, comments, stats):
	looking_for_title = None
	if type == "implicit":
		looking_for_title = "Valid Usage (Implicit)"
	elif type == "explicit":
		looking_for_title = "Valid Usage"
	
	# Find validation block from name
	def_block = soup.select_one("#" + name)
	parent = def_block.parent
	sidebar_block = None
	try:
		sidebar_block = next(filter(
			lambda block: block.select_one("div.title").string == looking_for_title,
			parent.select("div.sidebarblock")
		))
	except StopIteration:
		print(f"Could not find {name} ({type})", file = sys.stderr)
		return

	ul = sidebar_block.select_one("div.ulist ul")

	print(f"{VAL}box", file = stream, end = "\n\n")
	running_block = None

	lines = ul.select("li > p")
	for index in range(len(lines)):
		line_buffer = f"{index + 1}."

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
			running_block = handle_comment(stream, line_buffer, running_block, comments, str(index), stats)
		else:
			print(line_buffer, file = stream, end = "\n\n")
		stats["total"] += 1
	
	if running_block is not None:
		end_comment_block(stream, running_block)
	print(f"{VAL}boxend", file = stream, end = "\n\n")

## Sections
def process_section(stream, soup, name, section):
	print(">", name, file = sys.stderr)

	section_stats = {
		"total": 0,
		"comment": 0,
		"done": 0
	}

	for entry in section:
		if isinstance(entry, str):
			print(f"### {entry}", file = stream, end = "\n\n")
		else:
			entry_comments = None
			if len(entry) == 3:
				entry_comments = entry[2]

			print(f"Validations for `{entry[0]}`:", file = stream, end = "\n\n")
			
			print("  -", entry[0], f"({entry[1]})", file = sys.stderr)
			extract_usage(stream, soup, entry[0], entry[1], entry_comments, section_stats)
	
	return section_stats

## Utility
def print_stats(title, stats):
	print()
	print(title)
	print("  Comment:", stats["comment"])
	print("  Done:", stats["done"])
	print("  Rest:", stats["total"] - stats["comment"] - stats["done"])
	print("Total:", stats["total"])
	print()

def load_spec(path):
	soup = None

	print("Loading spec...", file = sys.stderr)
	time_before = time.perf_counter()
	with open(path, "r") as file:
		soup = bs4.BeautifulSoup(file.read(), features = PARSER)
	
	load_time = time.perf_counter() - time_before
	print("Loaded spec in {:.2f}s".format(load_time), file = sys.stderr)

	return soup

def main():
	if len(sys.argv) != 4:
		print(f"usage: {sys.argv[0]} structure spec out")
		exit(1)
	
	structure_path = sys.argv[1]
	spec_path = sys.argv[2]
	out_path = sys.argv[3]
	
	structure = None
	soup = None

	with open(structure_path, "r") as file:
		structure = json.load(file)

	soup = load_spec(spec_path)

	total_stats = {
		"total": 0,
		"comment": 0,
		"done": 0
	}
	for name in structure:
		print("\n###", name, "###", file = sys.stderr, end = "\n\n")
		with open(os.path.join(out_path, name + ".md"), "w") as file:
			section_stats = process_section(file, soup, name, structure[name])

			print_stats("Section stats: " + name, section_stats)
			total_stats["total"] += section_stats["total"]
			total_stats["comment"] += section_stats["comment"]
			total_stats["done"] += section_stats["done"]
	
	print_stats("Total stats:", total_stats)

main()