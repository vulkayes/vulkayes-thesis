\thispagestyle{empty}

# Abstract {.unnumbered .unlisted}

Vulkan is today the de-facto standard for open cross-platform real-time graphics rendering. It allows the developers to leverage the performance of GPUs on many platforms thanks to its uniform design and brings many improvements when compared to the older OpenGL API. Contrary to its predecessor, however, Vulkan is much more low-level and requires more work from the developer. This thesis presents a Rust crate (library) with core wrapper types and a high-level API design for creating and managing Vulkan objects safely in concurrent manner, with minimal impact on performance. Advantages of Rust for this specific task are discussed and the performance and verbosity in comparison to the ash crate (library), which provides raw bindings to the C Vulkan API, is evaluated.

**Keywords**: Vulkan, Vulkayes, graphics API, GPU, computer graphics, Vulkan API abstraction

\vfill

# Abstrakt {.unnumbered .unlisted}

Vulkan je dnes de-facto štandardom pre otvorené mnoho-platformové vykresľovanie grafiky v reálnom čase. Vývojári môžu vďaka Vulkanu naplno využiť silu grafických kariet na mnohých platformách vďaka jeho jednotnému dizajnu. Vulkan naviac prináša mnoho vylepšení v porovnaní so starším OpenGL. Vulkan je však zameraný na omnoho nižší level ako OpenGL a preto vyžaduje viac práce na strane vývojára. Táto práca predkladá knižnicu v jazyku Rust so základnými typmi a vysokoúrovňovým návrhom na vytváranie a spracovávanie objektov Vulkanu bezpečne vo viac vláknovom prostredí a s minimálnym dopadom na rýchlosť. Diskutované sú aj výhody jazyka Rust pre túto konkrétnu úlohu. Naviac je porovnaná rýchlosť a veľkosť kódu voči Rustovej knižnici ash, ktorá poskytuje priame prepojenia do Vulkan API napísaného v jazyku C.

**Kľúčové slová**: Vulkan, Vulkayes, grafické API, GPU, počítačová grafika, abstrakcia Vulkan API

**Preklad názvu**: Implementácia zobrazovacieho systému v jazyku Rust

\vfill