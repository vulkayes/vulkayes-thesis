#!/usr/bin/python3

import re
import sys
import fileinput

STATE_TEXT = 0
STATE_CODE = 1
STATE_YAML_HEADER = 2

RE_LINK = re.compile("!\[([^]]*)\]\([^)]+\)")
RE_INLINE_CODE = re.compile("`[^`]+`")
RE_NOT_WORD_CHARACTER = re.compile("[^\w]")

class Counter:
	current_state = STATE_TEXT
	
	seen_header = False
	text_words = 0
	text_characters = 0
	code_lines = 0

	def __init__(self):
		pass
	
	@staticmethod
	def filter_line(line):
		processed_line = line.lstrip().rstrip()
		processed_line = RE_LINK.sub("\\1", processed_line)
		processed_line = RE_INLINE_CODE.sub("$$$", processed_line)

		return processed_line

	@staticmethod
	def count_words(line):
		processed_line = Counter.filter_line(line)
		split = processed_line.split(" ")
		# print(f"\"{line}\" has {length} words and they are \n{split}\n", file = sys.stderr)

		# empty lines and headings ignore headings
		if len(processed_line) == 0 or processed_line[0] == "#":
			return 0
		else:
			return len(split)

	@staticmethod
	def count_characters(line):
		processed_line = Counter.filter_line(line)
		filtered = RE_NOT_WORD_CHARACTER.sub("", processed_line)

		# print(f"\"{line}\" has {len(filtered)} word characters \n{filtered}\n", file = sys.stderr)
		return len(filtered)

	def toggle_state(self, state):
		"""Returns `True` if this is toggling away from state"""
		if self.current_state == state:
			self.current_state = STATE_TEXT
			return True
		else:
			self.current_state = state
			return False

	def line(self, line):
		if line[:3] == "---" and self.seen_header:
			if self.toggle_state(STATE_YAML_HEADER):
				self.seen_header = True
				return
		if self.current_state == STATE_YAML_HEADER:
			return
	
		if line.lstrip()[:3] == "```":
			if self.toggle_state(STATE_CODE):
				self.code_lines += 1
				return
		if self.current_state == STATE_CODE:
			self.code_lines += 1
			return
		
		self.text_words += Counter.count_words(line)
		self.text_characters += Counter.count_characters(line)

	def finish(self):
		self.current_state = STATE_TEXT

		return {
			"words": self.text_words,
			"word_characters": self.text_characters,
			"code_lines": self.code_lines
		}

def main():
	counter = Counter()
	for line in fileinput.input():
		counter.line(line)
	
	result = counter.finish()

	print(result)

main()