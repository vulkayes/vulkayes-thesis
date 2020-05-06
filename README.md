# Vulkayes Thesis

This repository contains documents created for the Vulkayes bachelor thesis, such as images, videos, code samples and the document itself.

Vulkayes implementation code is available under [this GitHub company](https://github.com/vulkayes).

## Structure of this repository

Directory|Description
---------|-----------
codes|Code snipptes comparing selected features of Rust and C++, Rust code snippet benchmarks and the teapot benchmark.
documents|Documents containing topics covered in the final thesis and the thesis itself.
assets|Images, diagrams and other assets used in documents and thesis.
scripts|Scripts for building the thesis.
pdfs|Generated output pdfs.

## Building the thesis

Most of the thesis is written in markdown. [Pandoc](https://pandoc.org/) is used to convert markdown to latex and then to pdf. It is required to have `pandoc` and `python3` installed on the system.

_Note: Commonly used commands are defined as VSCode build tasks and are saved in `.vscode/tasks.json`._

1. Generate all puml diagrams in `assets/diagrams`. For this a [PlantUML](https://plantuml.com/) binary is needed.

2. Run `scripts/generate.sh` to generate the complete thesis. It will be opened using the `open` command afterwards. The generated pdf will be in `pdfs/thesis-output.pdf`.

3. _Optional_ - Run spec extractor in `scripts/spec_extractor.py` to create validation documents in `documents/validations` and then run `scripts/gen.sh progress` to generate the progress pdf.

