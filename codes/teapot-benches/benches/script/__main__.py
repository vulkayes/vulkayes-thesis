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
	False,
	False,
	True,
	True,
	True,
	True,
	False,
	False,
	False
]
STAGE_LABELS = [
	_STAGE_LABELS[index] for index in range(len(STAGE_FILTER)) if STAGE_FILTER[index]
]
STAGE_COUNT = len(STAGE_LABELS)

INPUT_IGNORE_FIRST_SAMPLES = 0
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

	def outlier_mask(self):
		return np.abs(
			self.data - self.median
		) <= self.std_dev * OUTLIERS_DEVIATIONS

	def reject_outliers(self):
		return self.data[self.outlier_mask()]

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
	stages_outlier_mask = None

	def __init__(self, path, name, color):
		self.path = f"{path}.txt"
		self.name = name
		self.color = f"{color}{COLOR_ALPHA}"

	def set_data(self, data):
		self.stages = [
			StageData(data[:, index]) for index in range(STAGE_COUNT)
		]
		self.stages_outlier_mask = self.stages[0].outlier_mask()
		for stage in self.stages[1:]:
			self.stages_outlier_mask &= stage.outlier_mask()

INPUTS = [
	# InputInfo("raw_ash_RHA", "ash_RHA", "#5EFEBF"),

	InputInfo("raw_ash", "ash", "#FE6F5E"),
	InputInfo("raw_vulkayes_ST", "vy_ST", "#5E9DFE"),
	InputInfo("raw_vulkayes_MT", "vy_MT", "#5EEDFE"),

	# InputInfo("raw_ash_uniform1000", "ash_u1000", "#FE6F5E"),
	# InputInfo("raw_vulkayes_ST_uniform1000", "vy_ST_u1000", "#5E9DFE"),
	# InputInfo("raw_vulkayes_MT_uniform1000", "vy_MT_u1000", "#5EEDFE"),
]

## IO ##
OUTPUT_FORMAT = "png"
WORK_FOLDER = "mac"
if len(sys.argv) > 1:
	WORK_FOLDER = sys.argv[1]
if len(sys.argv) > 2:
	OUTPUT_FORMAT = sys.argv[2]

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

def output_table_averages(base_input, inputs):
	print()
	print(
		"Stage|{}|{}".format(
			base_input.name,
			"|".join(inp.name for inp in inputs)
		)
	)
	print(
		"-----|{}|{}".format(
			"-" * len(base_input.name),
			"|".join("-" * len(inp.name) for inp in inputs)
		)
	)
	for index in range(STAGE_COUNT):
		print(
			"{}|{}|{}".format(
				STAGE_LABELS[index],
				format_time(base_input.stages[index].median),
				"|".join(
					[
						"{} ({:.0f}%)".format(
							format_time(inp.stages[index].median),
							inp.stages[index].median / base_input.stages[index].median * 100
						) for inp in inputs
					]
				)
			)
		)

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
def annotate_average_bars(ax, bars):
	y_limits = ax.get_ylim()
	y_height = abs(y_limits[0] - y_limits[1])
	annotation_height = y_height * 0.02

	for bar in bars:
		height = bar.get_height()

		ax.annotate(
			format_time(height),
			xy = (
				bar.get_x() + bar.get_width() / 2,
				annotation_height
			),
			xytext = (0, 0), textcoords = "offset points",
			ha = "center", va = "bottom",
			rotation = 90
		)

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

		annotate_average_bars(ax, bars)
		all_bars.append(bars)
	
	ax.set_title(title)
	# ax.set_ylabel("Time")
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
		dpi = 200
	)

	print(f"saved to {save_path}", file = sys.stderr)

def plot_histograms(inputs, stage_index, title, bins = 100):
	print(f"Plotting histogram: {title}.. ", file = sys.stderr, end = "", flush = True)

	fig, ax = plt.subplots()

	for inp in inputs:
		stage = inp.stages[stage_index]
		ax.hist(
			stage.data[inp.stages_outlier_mask],
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
	# ax.set_xlabel("Time")
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
	plt.savefig(
		save_path,
		dpi = 200
	)

	print(f"saved to {save_path}", file = sys.stderr)

def plot_linear(inp, title):
	print(f"Plotting histogram: {title}.. ", file = sys.stderr, end = "", flush = True)

	fig, ax = plt.subplots()
	x = np.arange(0, len(inp.stages[0].data), 1)
	y = np.zeros(shape = x.shape, dtype = np.int64)

	stage_datas = np.zeros(
		shape = (STAGE_COUNT, len(x))
	)

	stage_datas[0] = inp.stages[0].data
	for index in range(1, STAGE_COUNT):
		stage_datas[index] = stage_datas[index - 1] + inp.stages[index].data

	for index in range(STAGE_COUNT):
		ax.plot(
			x[inp.stages_outlier_mask],
			stage_datas[index][inp.stages_outlier_mask],
			label = STAGE_LABELS[index],
			marker = "|",
			linestyle = " "
		)

	ax.set_title(title)
	ax.set_xlabel("Sample")
	ax.legend()

	ax.yaxis.set_major_formatter(
		ticker.FuncFormatter(
			lambda x, pos: format_time(x)
		)
	)

	# plt.show()
	save_path = f"{WORK_FOLDER}/graphs/{title}_line.{OUTPUT_FORMAT}"
	plt.savefig(
		save_path,
		dpi = 200
	)

	print(f"saved to {save_path}", file = sys.stderr)

def main():
	for inp in INPUTS:
		data = read_input(f"{WORK_FOLDER}/{inp.path}")
		inp.set_data(data)

	plot_averages(
		INPUTS,
		"median time"
	)
	# output_table_averages(
	# 	INPUTS[0], INPUTS[1:]
	# )

	# for index in range(STAGE_COUNT):
	# 	plot_histograms(
	# 		INPUTS,
	# 		index,
	# 		f"{STAGE_LABELS[index]} samples"
	# 	)

	# if OUTPUT_FORMAT != "svg":
	# 	for inp in INPUTS:
	# 		plot_linear(
	# 			inp,
	# 			f"{inp.name} sample time"
	# 		)

main()