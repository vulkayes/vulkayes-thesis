import sys
import time

import numpy as np
import matplotlib
import matplotlib.pyplot as plt
import matplotlib.ticker as ticker

## STAGES ##
_STAGE_LABELS = [
	"preinit",
	"acquire",
	"uniform",
	"command",
	"submit",
	"present",
	"wait",
	"update",
	"total"
]
STAGE_FILTER = [
	True,
	True,
	True,
	True,
	True,
	True,
	True,
	True,
	True
]
STAGE_LABELS = [
	_STAGE_LABELS[index] for index in range(len(STAGE_FILTER)) if STAGE_FILTER[index]
]
STAGE_COUNT = len(STAGE_LABELS)

INPUT_IGNORE_FIRST_SAMPLES = 1000
OUTLIERS_DEVIATIONS = 2.0
class StageData:
	data = []
	mean = 0
	median = 0
	std_dev = 0

	def __init__(self, data):
		self.data = data[INPUT_IGNORE_FIRST_SAMPLES:]
		self.mean = np.mean(data)
		self.median = np.median(data)
		self.std_dev = np.std(data)

	def reject_outliers(self):
		indices = np.abs(
			self.data - self.median
		) <= self.std_dev * OUTLIERS_DEVIATIONS
		
		return self.data[indices]

	def outlier_bounds(self):
		return (
			self.median - self.std_dev * OUTLIERS_DEVIATIONS,
			self.median + self.std_dev * OUTLIERS_DEVIATIONS
		)

## INPUTS AND COLORS ##
COLOR_ALPHA = "D0"
class InputInfo:
	path = None
	name = None
	color = f"#000000{COLOR_ALPHA}"
	stages = None

	def __init__(self, path, name, color):
		self.path = f"{path}.txt"
		self.name = name
		self.color = f"{color}{COLOR_ALPHA}"

	def set_data(self, data):
		self.stages = [
			StageData(data[:, index]) for index in range(STAGE_COUNT)
		]

INPUTS = [
	InputInfo("raw_ash", "ash", "#FE6F5E"),
	# InputInfo("raw_ash_RHA_broken", "ash_RHA_broken", "#B3AAA9"),
	# InputInfo("raw_ash_RHA", "ash_RHA", "#5EFEBF"),
	InputInfo("raw_vulkayes_ST", "vy_ST", "#5E9DFE"),
	InputInfo("raw_vulkayes_MT", "vy_MT", "#5EEDFE")
]

## IO ##
OUTPUT_FORMAT = "png"
WORK_FOLDER = "mac"
if len(sys.argv) > 1:
	WORK_FOLDER = sys.argv[1]

def format_time(nanos):
	if nanos >= 10**9:
		seconds = round(nanos / 10**9, 2)
		return f"{seconds} s"
	
	if nanos >= 10**6:
		millis = round(nanos / 10**6, 2)
		return f"{millis} ms"

	if nanos >= 10**3:
		micros = round(nanos / 10**3, 2)
		return f"{micros} us"
	
	return f"{nanos} ns"

## PARSING ##
def parse_bench_point(span):
	marks = list(map(
		lambda x: np.int64(x.split(" = ")[1]),
		span.split(", ")
	))

	return np.array(marks, dtype = np.int64)[STAGE_FILTER]

def parse_line(line):
	LINE_PREFIX = "MARK_BUFFER: ["

	parsed_points = []

	if line[:len(LINE_PREFIX)] != LINE_PREFIX:
		return None

	current_index = len(LINE_PREFIX)
	while line[current_index] != "]":
		if line[current_index] == ",":
			current_index += 1
		elif line[current_index] == " ":
			current_index += 1
		elif line[current_index] == "{":
			span_end = current_index + 1
			while line[span_end] != "}":
				span_end += 1

			parsed_points.append(
				parse_bench_point(line[current_index + 1:span_end])
			)
			
			current_index = span_end + 1
	
	return parsed_points

def read_input(path):
	data_points = []

	print(f"Loading file {path}.. ", file = sys.stderr, end = "", flush = True)
	before_load = time.perf_counter()

	with open(path, "r") as file:
		for line in file:
			parsed = parse_line(line)
			if parsed is not None:
				data_points += parsed

	after_load = time.perf_counter()
	print(f"took {after_load - before_load:.3f}s", file = sys.stderr)
	
	return np.array(data_points, dtype = np.int64)

## PLOTTING ##
def plot_averages(inputs, title):
	print(f"Plotting averages: {title}.. ", file = sys.stderr, end = "", flush = True)

	bar_width = 0.7
	group_width = bar_width * (len(inputs) + 2)
	x = np.arange(STAGE_COUNT * group_width, step = group_width)

	fig, ax = plt.subplots()
	base_offset = -(len(inputs) - 1) * bar_width / 2
	
	all_bars = []
	for index in range(len(inputs)):
		inp = inputs[index]
		bars = ax.bar(
			x + base_offset + index * bar_width,
			[stage.median for stage in inp.stages],
			bar_width,
			label = inp.name,
			color = inp.color
		)

		for rect in bars:
			height = rect.get_height()
			ax.annotate(
				format_time(height),
				xy = (rect.get_x() + rect.get_width() / 2, height),
				xytext = (0, 3), textcoords = "offset points",
				ha = "center", va = "bottom",
				rotation = 75
			)

		all_bars.append(bars)
	
	ax.set_title(title)
	ax.set_ylabel("Time")
	ax.set_xticks(x)
	ax.set_xticklabels(STAGE_LABELS)
	ax.legend()

	plt.setp(
		ax.xaxis.get_majorticklabels(),
		rotation = -25,
		ha = "left",
		rotation_mode = "anchor"
	)

	save_path = f"{WORK_FOLDER}/graphs/bars.{OUTPUT_FORMAT}"
	plt.savefig(
		save_path,
		pad_inches = 1,
		dpi = 200
	)

	print(f"saved to {save_path}", file = sys.stderr)

def plot_histograms(inputs, stage_index, title, bins = 100):
	print(f"Plotting histogram: {title}.. ", file = sys.stderr, end = "", flush = True)

	fig, ax = plt.subplots()

	for inp in inputs:
		stage = inp.stages[stage_index]
		ax.hist(
			stage.reject_outliers(),
			bins = bins,
			label = inp.name,
			color = inp.color
		)

		ax.axvline(
			x = stage.median,
			color = inp.color[:-2],
			linestyle = "--"
		)

	ax.set_title(title)
	ax.set_xlabel("Time")
	ax.set_ylabel("Number of samples")
	ax.legend()

	ax.xaxis.set_major_formatter(
		ticker.FuncFormatter(
			lambda x, pos: format_time(x)
		)
	)
	plt.setp(
		ax.xaxis.get_majorticklabels(),
		rotation = -25,
		ha = "left",
		rotation_mode = "anchor"
	)

	save_path = f"{WORK_FOLDER}/graphs/{title}_hist.{OUTPUT_FORMAT}"
	plt.savefig(save_path)

	print(f"saved to {save_path}", file = sys.stderr)

def main():
	for inp in INPUTS:
		data = read_input(f"{WORK_FOLDER}/{inp.path}")
		inp.set_data(data)

	plot_averages(
		INPUTS,
		"median time"
	)

	for index in range(STAGE_COUNT):
		plot_histograms(
			INPUTS,
			index,
			f"{STAGE_LABELS[index]} samples"
		)

main()