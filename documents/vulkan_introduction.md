## Vulkan API overview

Vulkan API, originally released in 2016[@VulkanAPIRelease], is a specification of an open API for high-efficiency, cross-platform access to graphics and compute on modern GPUs.

It is designed to minimize the overhead between the user application and the hardware device. Vulkan achieves this by staying low level and explicitly requiring all relevant state to be referenced by the user application, minimizing required lookups and orchestration on the driver side. This allows the user application to optimize for their specific usecase instead of relying on the driver to guess the correct strategy. However, it requires much more complexity from the user application and is much harder to master than OpenGL.

One of the reasons for Vulkans popularity is that it was designed in an intense collaboration between leading hardware, game engine and platform vendors[@VulkanAPIRelease]. This resulted in a lot of vendors having zero-day support for the specification in their drivers and software and it being immediatelly adopted as a native rendering solution on many platforms.

The openness of Vulkan also goes hand-in-hand its cross-platform capabilities. Vulkan is available on all three major desktop platforms (Linux, macOS, Windows) and both major smartphone platforms (Android, iOS), but also on many smaller and embedded platforms. This allows applications to easily target multiple platforms with minimal variance in the rendering code. It also prevents vendor locks as seen with DirectX or Metal APIs. Lastly, it allows the community of both professionals and hobbyists to participate in the standard itself and improve it.

One of the first mainstream games supporting Vulkan was Dota 2[@VulkanDota2] developed by Valve, the founding company behind LunarG. LunarG is a company that specializes in developing Vulkan SDK and increasing Vulkan support[@LunarG]. Support has also quickly been added to game engines such as Unity, Unreal or Godot, allowing its power to be presented to bigger and bigger audiences.

Khronos Group, the industry consortium responsible for Vulkan API, has been continuously improving the API and releasing updates. The API is currently on version 1.2[@VulkanAPIRelease12], which brough important updates that have been requested by the community. This proves that Vulkan aims to improve alongside the industry and provide support and improvements into the forseeable future.

## Vulkan API architecture

Vulkan is designed to be very explicit about communicating intentions and possibly expensive operations between the implementation and application. The entry point into Vulkan is the instance object, which is created by calling the `vkCreateInstace` function. This function has to be dynamically loaded, since Vulkan may be linked dynamically instead of statically. The instance object serves as the parent of all other Vulkan objects in given context and should encapsulate the whole application.

### Execution model

Execution model of Vulkan specifies how to initialize, prepare and execute actions on Vulkan-capable hardware. Given an instance object the application can enumerate physical devices connected to the system. These Vulkan objects represent the hardware objects supported by the local Vulkan implementation instance. Typically, they represent the GPUs (both integrated and discrete) connected to the system.

The application can create a logical device object for each physical device. This is one of the first signs on how multi-gpu paralellism works in Vulkan. Objects created from a specific Vulkan device are device-private, but Vulkan specifies a way to export objects from one device to another. Each logical device exposes so-called queues, which process work independently of each other. This represents the single-gpu parallelism in Vulkan.

Queues in Vulkan are partitioned into queue families. Each family contains queues which are compatible with each other and can seamlessly execute identical workloads. Queues not belonging to one family may not be able to execute identical workloads, the capabilities of queue families can be queried from the device.

Device memory is allocated using logica device as a parent. Device memory is always visible to the device and can be either physically located in the device memory or in the host memory. The memory can also additionaly be visible and mappable to the host memory. Devices advertise supported memory types as heaps with its types exposed as bit flags. The device can advertise many heaps, but some devices, notably integrated ones, often advertise a single multi-purpose heap for all device allocations.

Once the application has initialize the instance object, allocated memory and prepared workloads into command buffers, the work is submitted onto queues requested along with creation of logical devices. When the work is submitted, control returns to the host application and and work is asynchronously executed on the device until completion. There is no implicit way to check workload completion nor are there guarantees between submission order and task completion. Even within a specific device queue, some work may interleave and execute out of order (within some coherency constraints).

To synchronize between the host and device, between two devices or even within the device itself the synchronization primitives have to be used explicitly. Fences can be used to synchronize between the host and the device while semaphores can be used to synchronize between device operations. All of this is the responsibility of the application, but can result in great performance if used correctly.

### Object model

Entities in Vulkan are represented as opaque objects and are handled through handles. Handles are either dipatchable (e.g. pointers) or non-dispatchable (e.g. integers). Dispatchable handles are guaranteed to be unique while non-dispatchable handles are fully opaque up to the value of the handle. The only guarantee is the binary interface (the size) of the handle.

There are parent-child relationships between certain objects and this structure forms a partial ordering on both their initialization and their destruction. Some objects are destroyed implicitly (when their parent object is destroyed) and some objects have to be destroyed explicitily. There are exceptions where the child object does not need to be destroyed before its parent but must not be used after its parent is destroyed (e.g. such object can be destroyed after its parent). Vulkayes observes these relationship using reference counting as described in [@sec:object_lifetime_management].

### Application structure

![
	Overwiew of Vulkan API objects and basic data flow.[@V-EZ]
](assets/images/VulkanAPI.png)

The high level structure of Vulkan is that the user application creates an instance and chooses one or more physical devices. Queues on these devices can be split into graphics, compute, transfer and sparse categories. Some queues may support multiple properties. The application will create the queues as needed together with the device.

After device creation, the application is expected describe as much state as it can beforehand. The application needs to create the render pass and within it describe all attachments, subpasses and dependencies of those subpasses. This early definition allows the implementation to transform this description into internal performance-oriented representation that is specific to the device. Similar process happens with the descriptor set layouts and pipeline layouts, where the application describes the requested features and settings of the descriptor/pipeline and then can allocate these objects based on this description.

Assuming that the application is going to render to the screen a surface needs to be created along with a swapchain. Creating surface is platform-dependent process since it requires a specific extension for the given platform and a platform-specific window handle. After the surface is created, the application creates a swapchain which takes care of presenting images onto the surface. The swapchain is platform-agnostic but is also implemented as an extension since not all platform necessairly need or support display surfaces or swapchains.

Images and buffers are another requirement in the process. The memory for both images and buffers is allocated and bound separately. This allows the application to use custom allocators and/or (with specific extensions) to create sparsely-bound images. Images additionaly specify a layout of their memory. This layout type can either be linear, and thus freely accesible from host, or optimal, and thus its structure is unspecified. Since most of the time images are not accessed from the host and are instead uploaded using staging buffers, most images are recommended to use the optimal layout type for performance.

The optimal layout type specifies multiple layouts as an enumeration. This enumeration always contains the `GENERAL` layout, which can be used in any context but may be least performant, and other additional layouts that may be used in specific contexts to potentially improve the performance. For example, there is a specific layout for transfer operations which is optimal for all copy and blit operations but cannot be used in attachment context. Additional complexity is added by the fact that the image can have multiple layouts at once in form of subranges. If an image is mipmapped or is an array image, each array layer and each mipmap level can potentially have different layout. The application is required to keep track of the current image layout because it is required to specify the layout, _at the time of execution_, in most of the commands that work with images.

To use images and buffers as attachments, the application needs to create views. A view object is a view into a specific subrange (mipmap levels and array layers) of an image or a specific range (offset and size) of a buffer. Views contain additional metadata that is used within the operation they are passed into. This metadata includes the subrange size, component mapping (e.g. mapping RGB to BGR) and, if an extension is enabled, different (but compatible) format for accesing the image data.

At some point, the application also needs to create memory pools. There are two types of memory pools, the descriptor pool and the command pool. These pools both serve the same general purpose, but they have different usage requirements. The importance of memory pools is that some allocations, namely the allocation of descriptor sets and command buffers, happens very frequently. System memory allocation can have considerable overhead and should be done infrequently. Pools solve this issue by allocating system memory separately from the resource memory. Allocating and freeing from the same pool will produce much less overhead than allocating and freeing from the system. Descriptor pools additionaly require a list of descriptor set layouts for which they can allocate, while command pools grow not only with initial command buffer allocation but also with each recorded command.

Shader modules are developed and compiled into SPIRV independently of the Vulkan API. The application should retrieve its shader code and pass it to Vulkan shader module creation function to create shader module object. Shader modules can later be bound to pipelines.

The next step is actualization of previously described resources. Framebuffers represent an actualization of the render pass description attachment list. Framebuffers are used in render pass commands to bind already prepared render passes with current attachments, since attachment may and often do change frame-by-frame. Descriptor sets represent an actualization of the shader interface which is "described" by the shader modules. Pipelines represent an actualization of pipeline layouts. Most notably, pipelines describe the connection between shader modules and descriptor sets.

To render a singular frame not much beyond the device `waitIdle` method is need. However, since most applications will want to render continuously, synchronization is needed not only between frames but also between acquiring and using presentable images from the swapchain. Fences and semaphores need to be created by the application.

After preparing all the necessary resources, descriptions and actualization, the work to be done needs to be recorded into a command buffer. Command buffer is allocated from the command pool and then a few types of commands are recorded into it. Some commands may perform work of more than one type.

First type is state setting type. These commands alter the current state of the command buffer at the time of the command execution. These commands are generally not reordered during execution because they create an implici happens-before relationship with commands after it that use that same state. These commands will bind the actualizations to the current context, such as the current frame frame buffers, pipeline and descriptors.

Second type is action type. These commands peforms actions on resources, perform reads and writes and produce observable results. In general, these commands are most likely to be reordered and executed in parallel to improve performance. These commands include clearing, copying, blitting, drawing and computing.

Last type is synchronization type. These commands explicitly define relationships and contrains of the reodering of other commands. For example, to avoid race conditions between writing to an image and subsequently reading from it a pipeline barrier must be inserted using a synchronization command.

Finally, after a command buffer is recorded, it may be submitted for execution on a selected device queue. This queue must support the command types used within the command buffer. All resources used by the command buffer must be kept alive for at least as long as the execution lasts and all images must have the correct layouts at the time of execution of the specific command inside the command buffer that layout is defined in. All this has to be done manually by the application.

### Complexity

As can be seen from the previous section, complexity of Vulkan applications is much greater than of its predecessor OpenGL. All this complexity gives opportunity to the application to fully express exactly when it does and doesn't need and thus be as performant as possible. However, adhering to the strict rules imposed by the API can be challenging for programmers and can be improved upon by using modern programming technologies and concepts.