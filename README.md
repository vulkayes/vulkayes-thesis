# Vulkayes Thesis

This repository contains documents created for the Vulkayes bachelor thesis, such as images, videos, code samples and the document itself.

Vulkayes implementation code is available under [this GitHub company](https://github.com/vulkayes).

## Structure of this repository

Directory|Description
---------|-----------
codes|Code snipptes comparing selected features of Rust and C++ and Rust code snippet benchmarks.
documents|Documents containing topics covered in the final thesis and the thesis itself.
assets|Images, diagrams and other assets used in documents and thesis.
scripts|Scripts for building the thesis.
pdfs|Generated output pdfs.

## Building the thesis

Most of the thesis is written in markdown. [Pandoc](https://pandoc.org/) is used to convert markdown to latex and then to pdf. 

_Note: Commonly used commands are defined as VSCode build tasks and are saved in `.vscode/tasks.json`._

1. Generate all puml diagrams in `assets/diagrams`. For this a [PlantUML](https://plantuml.com/) binary is needed.

2. Run spec extractor in `scripts/spec_extractor.py` to create validation documents in `documents/validations`.

3. ??? TODO

4. Run `scripts/generate.sh` to generate the complete thesis. It will be opened using the `open` command afterwards. The generated pdf will be in `pdfs/thesis-output.pdf`.

