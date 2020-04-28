import sys
from dataclasses import dataclass

import numpy as np
import matplotlib
import matplotlib.pyplot as plt

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
STAGE_COUNT = len(list(filter(lambda x: x, STAGE_FILTER)))

STAGE_LABELS = [
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

COLOR_ALPHA = "D0"
COLORS = [
	f"#FE6F5E{COLOR_ALPHA}",
	f"#5EFEBF{COLOR_ALPHA}",
	f"#5E9DFE{COLOR_ALPHA}",
	f"#5EEDFE{COLOR_ALPHA}"
]
INPUT_NAMES = [
	"ash",
	"vy_ST",
	"vy_MT"
]

WORK_FOLDER = "mac"
if len(sys.argv) > 1:
	WORK_FOLDER = sys.argv[1]

OUTLIERS_DEVIATIONS = 2.0

class StageData:
	data = []
	mean = 0
	median = 0
	std_dev = 0

	def __init__(self, data):
		self.data = data
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

	print(f"Loading file {path}", file = sys.stderr)
	with open(path, "r") as file:
		for line in file:
			parsed = parse_line(line)
			if parsed is not None:
				data_points += parsed
	
	return np.array(data_points, dtype = np.int64)

## PROCESSING ##
def compute_averages(data_points):
	counter = np.zeros(shape = (STAGE_COUNT,), dtype = np.int64)
	for point in data_points:
		counter += point
	
	return counter / len(data_points)

def reject_outliers(data):
	return data[abs(data - np.median(data)) < OUTLIERS_DEVIATIONS * np.std(data)]

## PLOTTING ##
def format_time(nano_seconds):
	nanos = round(nano_seconds % 1**3)
	micros = round(nano_seconds / 10**3 % 10**6)
	millis = round(nano_seconds / 10**6 % 10**9)
	seconds = round(nano_seconds / 10**9)

	if seconds > 0:
		return f"{seconds:.0f}.{millis:0>3.0f} s"
	if millis > 0:
		return f"{millis:.0f}.{micros:0>3.0f} ms"
	if micros > 0:
		return f"{micros:.0f}.{nanos:0>3.0f} us"
	return f"{nanos:.2f} ns"

def plot_averages(averages, names, colors, title = None):
	print(f"Plotting averages {title}", file = sys.stderr)

	bar_width = 0.7
	group_width = bar_width * (len(averages) + 2)
	x = np.arange(STAGE_COUNT * group_width, step = group_width)

	fig, ax = plt.subplots()
	base_offset = -(len(averages) - 1) * bar_width / 2
	
	all_bars = []
	for index in range(len(averages)):
		bars = ax.bar(
			(x + base_offset + index * bar_width),
			averages[index],
			bar_width,
			label = names[index],
			color = colors[index]
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

	plt.savefig(f"{WORK_FOLDER}/graphs/bars.png")

def plot_histograms(stages, names, colors, title = None, bins = 100):
	print(f"Plotting histogram {title}", file = sys.stderr)

	fig, ax = plt.subplots()

	for index in range(len(stages)):
		ax.hist(
			stages[index].reject_outliers(),
			bins = bins,
			label = names[index],
			color = colors[index]
		)

		ax.axvline(
			x = stages[index].median,
			color = colors[index][:-2],
			linestyle = "--"
		)
		# ax.axvline(
		# 	x = stages[index].mean,
		# 	color = colors[index][:-2]
		# )

		# ax.axvline(
		# 	x = stages[index].outlier_bounds()[0],
		# 	color = colors[index][:-2],
		# 	linestyle = ":"
		# )
		# ax.axvline(
		# 	x = stages[index].outlier_bounds()[1],
		# 	color = colors[index][:-2],
		# 	linestyle = ":"
		# )

	ax.set_title(title)
	ax.set_xlabel("Time (ns)")
	ax.set_ylabel("Number of samples")
	ax.legend()

	plt.savefig(f"{WORK_FOLDER}/graphs/{title}_hist.png")
	# plt.show()

def plot_histograms_helper(datas, index):
	plot_histograms(
		[StageData(dat[:, index]) for dat in datas],
		INPUT_NAMES,
		COLORS,
		title = f"{STAGE_LABELS[index]} samples"
	)

def main():
	ash_datas = read_input(f"{WORK_FOLDER}/raw_ash.txt")
	vy_ST_datas = read_input(f"{WORK_FOLDER}/raw_vulkayes_ST.txt")
	vy_MT_datas = read_input(f"{WORK_FOLDER}/raw_vulkayes_MT.txt")

	ash_averages = compute_averages(ash_datas[1000:])
	vy_ST_averages = compute_averages(vy_ST_datas[1000:])
	vy_MT_averages = compute_averages(vy_MT_datas[1000:])

	plot_averages(
		[ash_averages, vy_ST_averages, vy_MT_averages],
		INPUT_NAMES,
		COLORS,
		title = "Average time per stage"
	)

	for index in range(STAGE_COUNT):
		plot_histograms_helper([ash_datas, vy_ST_datas, vy_MT_datas], index)

main()