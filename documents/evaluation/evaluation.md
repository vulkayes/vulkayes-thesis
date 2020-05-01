# Evaluation

![](evaluation/user_code.md)

## Benchmark

A benchmark of ash vs Vulkayes was designed to compare the speed of Vulkayes against a "control sample" of ash. This benchmark measures several stages of a common rendering loop between ash and Vulkayes. Since Vulkayes is mostly safety and usability wrapper, not much runtime overhead is added, at least not in the critical hot-paths used in rendering loops. Some specific areas, however, such as memory mapping and writing need special handling to ensure safety, as mentioned in [@sec:user-code].

The rendering loop was split into 8 stages:

#### preinit

In this stage frame specific variables are calculated, such as the data dependent on the elapsed time. This stage represents the user logic that is not being benchmarked.

#### acquire

In this stage the present image is acquired from Vulkan implementation. This stage is heavily dependent on the performance of synchronization primitives - mutexes - so some overhead is expected.

#### uniform

In this stage uniform data specific for the frame is written into device visible mapped memory and flushed. Some absolute overhead is expected because Vulkayes does checks to ensure memory safety.

#### command

In this stage command buffer is recorded by binding necessary state and submitting draw commands for each teapot, along with push constants. Minimal overhead is expected as only one mutex needs to be locked.

#### submit

In this stage the previously recorded command buffer is submitted for execution to Vulkan. This operation is costly on its own, but only minimal overhead is expected.

#### present

In this stage the acquired image is submitted for presentation and a happens-before relationship is estabilished using semaphores and fences so that submitted execution is guaranteed to finish before presentation begins. Again, overhead of a mutex is expected.

#### wait

In this stage the application waits on the presentation fence. This ensures that all measurements done inside one loop are correctly assigned to that loop. In real life applications, this wait should not happen and each frame should asynchronously finish in the background while the user logic computes data for the next frame (akin to the preinit stage). This stage represents the cost of the submitted operations on the GPU, but might also invoke some implementation-dependent synchronization the application is not aware of. Timings of this stage are thus not considered.

#### update

In this stage the update function is called on the window and all outstanding windowing events are handled. This stage represents the update of the windowing system events and a window redraw request and is not being benchmarked.

The benchmarks were run on three hardware and software configurations:

Stage|ash|vy_ST|vy_MT
-----|---|-----|-----
acquire|52.78 us|54.32 us (103%)|55.0 us (104%)
uniform|1.16 us|2.19 us (189%)|2.32 us (200%)
command|23.71 us|23.75 us (100%)|24.2 us (102%)
submit|168.21 us|173.3 us (103%)|170.96 us (102%)
present|32.88 us|33.83 us (103%)|34.46 us (105%)

Table: macOS 10.15.3 (19D76), Quad-Core Intel Core i5, Intel Iris Plus Graphics 655, Vulkan 1.2.135

Stage|ash|vy_ST|vy_MT
-----|---|-----|-----
acquire|1.84 us|2.36 us (129%)|2.32 us (126%)
uniform|717.0 ns|1.53 us (214%)|1.38 us (193%)
command|39.37 us|40.03 us (102%)|39.78 us (101%)
submit|39.57 us|37.36 us (94%)|38.64 us (98%)
present|25.34 us|26.07 us (103%)|26.45 us (104%)

Table: Linux 5.4.35_1, Intel i5-7300HQ, TODO

Stage|ash|vy_ST|vy_MT
-----|---|-----|-----
acquire|7.23 us|8.78 us (121%)|16.86 us (233%)
uniform|1.42 us|2.15 us (152%)|2.24 us (158%)
command|28.51 us|27.99 us (98%)|28.8 us (101%)
submit|13.15 us|13.76 us (105%)|14.38 us (109%)
present|27.08 us|26.63 us (98%)|27.29 us (101%)

Table: Linux 5.4.35_1, Intel i5-7300HQ, NVIDIA GeForce GTX 1050 Mobile, Vulkan v1.2.137

![
	Histogram of uniform write stage of the benchmarks (n = 100000). It's clear that ash is faster than both single- and multi-threaded Vulkayes simply because it does no other safety handling.
](assets/images/result_uniform_mac.svg)