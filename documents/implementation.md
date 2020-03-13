---
geometry:
	- margin=2cm
header-includes:
	- |
		```{=latex}
			\usepackage[most,breakable]{tcolorbox}

			\definecolor{inline-code}{HTML}{264357}

			\definecolor{validation-box}{HTML}{EAEAEA}
			\definecolor{validation-comment}{HTML}{0099B3}
			\definecolor{validation-comment-box}{HTML}{0099B3}
			\definecolor{validation-done}{HTML}{00B372}
			\definecolor{validation-done-box}{HTML}{00B372}

			\let\oldtexttt\texttt
			\renewcommand{\texttt}[1]{\textcolor{inline-code}{\oldtexttt{#1}}}
			\newcommand{\valcom}{\color{validation-comment}}
			\newcommand{\valdone}{\color{validation-done}}

			\newtcolorbox{validation-box}{breakable,colback=validation-box,boxrule=0pt,arc=0pt,halign=left,left=-5pt}
			\newcommand{\valbox}{\begin{validation-box}\pretolerance=10000}
			\newcommand{\valboxend}{\end{validation-box}}

			\newtcolorbox{validation-comment-box}{breakable,colback=validation-box,colframe=validation-comment-box,size=fbox,left*=0pt,leftrule=10pt}
			\newcommand{\valcombox}{\begin{validation-comment-box}}
			\newcommand{\valcomboxend}{\end{validation-comment-box}}

			\newtcolorbox{validation-done-box}{breakable,colback=validation-box,colframe=validation-done-box,size=fbox,left*=0pt,leftrule=10pt}
			\newcommand{\valdonebox}{\begin{validation-done-box}}
			\newcommand{\valdoneboxend}{\end{validation-done-box}}
		```
mainfont: Fira Sans
sansfont: Fira Sans
monofont: Fira Code
---

![Object Dependency Graph](media/object_dependency_graph.svg)

This document describes the plan and progress of the implementation of Vulkayes.

![](synchronization/synchronization.md)

![](validations/validations.md)
