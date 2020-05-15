## Benchmark

A benchmark of ash vs Vulkayes was designed to compare the speed of Vulkayes against a "control sample" of ash. This benchmark measures several stages of a common rendering loop between ash and Vulkayes. Since Vulkayes is mostly safety and usability wrapper, not much runtime overhead is added, at least not in the critical hot-paths used in rendering loops. Some specific areas, however, such as memory mapping and writing need special handling to ensure safety, as mentioned in [@sec:user-code].

The benchmark results have three separate columns. `ash` is the control sample/baseline measurement. `vy_ST` is the single-threaded Vulkayes while `vy_MT` is the multi-threaded feature of Vulkayes.

![
	The benchmark consists of 25 non-instanced teapots with each having 531 vertices and normals and 3072 indices. Teapots at even positions are controlled by taking the $sin(time)$ while odd positions are using $-sin(time)$. Color and spin of the teapot is computed using the harmonic function. Color and world matrix are uploaded using push constants while view and projection matrices are uploaded using uniform buffers. View and projection matrices don't change but are uploaded anyway to bench their speed. There is a very simple lighting model with hard-coded directional light in the fragment shader.
](assets/images/teapot_bench_screenshot.png)

### Stages

The rendering loop was split into 8 stages:

#### preinit

In this stage frame specific variables are calculated, such as the data dependent on the elapsed time. This stage represents the user logic that is not being benchmarked.

#### acquire

In this stage the present image is acquired from Vulkan implementation. This stage is heavily dependent on the Vulkan implementation and is not being benchmarked.

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

### Results {#sec:benchmark_results}

The benchmarks were run on three hardware and software configurations, note that only the relevant stages are present:

![
	_Average median time (n = 99000): macOS 10.15.3 (19D76), Quad-Core Intel Core i5, Intel Iris Plus Graphics 655, Vulkan 1.2.135_
](assets/images/mac_bars.svg)

Table: _Average median time (n = 99000): macOS 10.15.3 (19D76), Quad-Core Intel Core i5, Intel Iris Plus Graphics 655, Vulkan 1.2.135_

Stage|ash|vy_ST|vy_MT
-----|---|-----|-----
uniform|1.5 us|2.37 us (157%)|2.39 us (159%)
command|23.66 us|24.43 us (103%)|26.51 us (112%)
submit|169.43 us|171.11 us (101%)|170.99 us (101%)
present|32.76 us|33.36 us (102%)|34.14 us (104%)

![
	_Average median time (n = 99000): Linux 5.4.35_1, Intel i5-7300HQ,  Intel HD Graphics 630, Vulkan v1.2.137_
](assets/images/linux_intel_bars.svg)

Table: _Average median time (n = 99000): Linux 5.4.35_1, Intel i5-7300HQ,  Intel HD Graphics 630, Vulkan v1.2.137_

Stage|ash|vy_ST|vy_MT
-----|---|-----|-----
uniform|717.0 ns|1.53 us (214%)|1.38 us (193%)
command|39.37 us|40.03 us (102%)|39.78 us (101%)
submit|39.57 us|37.36 us (94%)|38.64 us (98%)
present|25.34 us|26.07 us (103%)|26.45 us (104%)

![
	_Average median time (n = 99000): Linux 5.4.35_1, Intel i5-7300HQ, NVIDIA GeForce GTX 1050 Mobile, Vulkan v1.2.137_
](assets/images/linux_nv_bars.svg)

Table: _Average median time (n = 99000): Linux 5.4.35_1, Intel i5-7300HQ, NVIDIA GeForce GTX 1050 Mobile, Vulkan v1.2.137_

Stage|ash|vy_ST|vy_MT
-----|---|-----|-----
uniform|1.42 us|2.15 us (152%)|2.24 us (158%)
command|28.51 us|27.99 us (98%)|28.8 us (101%)
submit|13.15 us|13.76 us (105%)|14.38 us (109%)
present|27.08 us|26.63 us (98%)|27.29 us (101%)

As can be seen, all three tested systems exhibit similar trends. The command stage is on par with pure ash benchmark, the only possible overhead is one mutex lock, which will only have an effect on multi-thread feature in the worst case.

The submit stage also closely follows the ash baseline. This stage potentially locks great number of mutexes, so could be a potential performance bottleneck on the multi-thread feature. However, the intention of an explicit submit operation in Vulkan API is to reduce overhead of submitting smaller batches of work in favor of bigger ones, where the overhead is less noticeable. Thus, for real life applications where the command buffer size will be much bigger, it is expected to be manageable.

The present stage, similaliry, does not exhibit any noticeable slowdown. The reasoning is the same as for the submit stage. Additionaly, the present stage may also include the vertical synchronization delay if enabled, and will thus shadow smaller overhead factors such as locking mutexes.

Finally, the uniform stage exhibits the most interesting results. The accesses performed in Vulkayes are 1.5 to 2 times as slow as when performed by ash. This seems like a lot, but it is important to mention that the absolute difference between the median points is in range of 1 micro second and the overhead is of constant nature.

![
	_Histogram of uniform stage of the benchmarks (n = 99000). It is clear that ash is faster than both single- and multi-threaded Vulkayes. However, the overhead is constant._
](assets/images/uniform_mac_hist.svg)

Furthermore, [@tbl:macos_uniform_1000] demonstrates doing 1000 writes into the mapped memory instead of 1 each frame. In fact, Vulkayes is even sligtly faster in this case because it decides on the most efficient strategy for the write, which becomes efficient with larger number of writes.

Stage|ash_u1000|vy_ST_u1000|vy_MT_u1000
-----|---------|-----------|-----------
uniform|45.16 us|40.61 us (90%)|42.14 us (93%)

Table: _Average median time (n = 99000): macOS 10.15.3 (19D76), Quad-Core Intel Core i5, Intel Iris Plus Graphics 655, Vulkan 1.2.135_ {#tbl:macos_uniform_1000}

![
	_Histogram of uniform stage of the benchmarks (n = 99000) with 1000 writes instead of 1. The overhead displayed in previous bench is overshadowed by the gains of proper writing strategy._
](assets/images/uniform1000_mac_hist.svg)