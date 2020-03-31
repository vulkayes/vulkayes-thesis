### Queue

Validations for `vkGetDeviceQueue`:

\valbox

\valcombox

1. `queueFamilyIndex` must be one of the queue family indices specified when `device` was created, via the `VkDeviceQueueCreateInfo` structure
	- \valcom Handled by API design

2. `queueIndex` must be less than the number of queues created for the specified queue family index when `device` was created, via the `queueCount` member of the `VkDeviceQueueCreateInfo` structure
	- \valcom Handled by API design

3. `VkDeviceQueueCreateInfo`::`flags` must have been set to zero when `device` was created
	- \valcom Handled by API design

\valcomboxend

\valboxend

Validations for `vkGetDeviceQueue2`:

Validations for `vkQueueSubmit`:

\valbox

1. If `fence` is not `VK_NULL_HANDLE`, `fence` must be unsignaled

2. If `fence` is not `VK_NULL_HANDLE`, `fence` must not be associated with any other queue command that has not yet completed execution on that queue

3. Any calls to `vkCmdSetEvent`, `vkCmdResetEvent` or `vkCmdWaitEvents` that have been recorded into any of the command buffer elements of the `pCommandBuffers` member of any element of `pSubmits`, must not reference any `VkEvent` that is referenced by any of those commands in a command buffer that has been submitted to another queue and is still in the _pending state_

4. Any stage flag included in any element of the `pWaitDstStageMask` member of any element of `pSubmits` must be a pipeline stage supported by one of the capabilities of `queue`, as specified in the table of supported pipeline stages

5. Each element of the `pSignalSemaphores` member of any element of `pSubmits` must be unsignaled when the semaphore signal operation it defines is executed on the device

6. When a semaphore wait operation referring to a binary semaphore defined by any element of the `pWaitSemaphores` member of any element of `pSubmits` executes on `queue`, there must be no other queues waiting on the same semaphore

7. All elements of the `pWaitSemaphores` member of all elements of `pSubmits` created with a `VkSemaphoreType` of `VK_SEMAPHORE_TYPE_BINARY` must reference a semaphore signal operation that has been submitted for execution and any semaphore signal operations on which it depends (if any) must have also been submitted for execution

8. Each element of the `pCommandBuffers` member of each element of `pSubmits` must be in the pending or executable state

9. If any element of the `pCommandBuffers` member of any element of `pSubmits` was not recorded with the `VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT`, it must not be in the pending state

10. Any secondary command buffers recorded into any element of the `pCommandBuffers` member of any element of `pSubmits` must be in the pending or executable state

11. If any secondary command buffers recorded into any element of the `pCommandBuffers` member of any element of `pSubmits` was not recorded with the `VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT`, it must not be in the pending state

\valdonebox

12. Each element of the `pCommandBuffers` member of each element of `pSubmits` must have been allocated from a `VkCommandPool` that was created for the same queue family `queue` belongs to
	- \valdone Returns error

\valdoneboxend

13. If any element of `pSubmits->pCommandBuffers` includes a Queue Family Transfer Acquire Operation, there must exist a previously submitted Queue Family Transfer Release Operation on a queue in the queue family identified by the acquire operation, with parameters matching the acquire operation as defined in the definition of such acquire operations, and which happens before the acquire operation

14. If a command recorded into any element of `pCommandBuffers` was a `vkCmdBeginQuery` whose `queryPool` was created with a `queryType` of `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR`, the profiling lock must have been held continuously on the `VkDevice` that `queue` was retrieved from, throughout recording of those command buffers

15. Any resource created with `VK_SHARING_MODE_EXCLUSIVE` that is read by an operation specified by `pSubmits` must not be owned by any queue family other than the one which `queue` belongs to, at the time it is executed

\valboxend

Validations for `VkSubmitInfo`:

\valbox

1. Each element of `pCommandBuffers` must not have been allocated with `VK_COMMAND_BUFFER_LEVEL_SECONDARY`

2. If the geometry shaders feature is not enabled, each element of `pWaitDstStageMask` must not contain `VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT`

3. If the tessellation shaders feature is not enabled, each element of `pWaitDstStageMask` must not contain `VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT` or `VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT`

4. Each element of `pWaitDstStageMask` must not include `VK_PIPELINE_STAGE_HOST_BIT`.

5. If any element of `pWaitSemaphores` or `pSignalSemaphores` was created with a `VkSemaphoreType` of `VK_SEMAPHORE_TYPE_TIMELINE`, then the `pNext` chain must include a `VkTimelineSemaphoreSubmitInfo` structure

6. If the `pNext` chain of this structure includes a `VkTimelineSemaphoreSubmitInfo` structure and any element of `pWaitSemaphores` was created with a `VkSemaphoreType` of `VK_SEMAPHORE_TYPE_TIMELINE`, then its `waitSemaphoreValueCount` member must equal `waitSemaphoreCount`

7. If the `pNext` chain of this structure includes a `VkTimelineSemaphoreSubmitInfo` structure and any element of `pSignalSemaphores` was created with a `VkSemaphoreType` of `VK_SEMAPHORE_TYPE_TIMELINE`, then its `signalSemaphoreValueCount` member must equal `signalSemaphoreCount`

8. For each element of `pSignalSemaphores` created with a `VkSemaphoreType` of `VK_SEMAPHORE_TYPE_TIMELINE` the corresponding element of `VkTimelineSemaphoreSubmitInfo`::pSignalSemaphoreValues must have a value greater than the current value of the semaphore when the semaphore signal operation is executed

9. For each element of `pWaitSemaphores` created with a `VkSemaphoreType` of `VK_SEMAPHORE_TYPE_TIMELINE` the corresponding element of `VkTimelineSemaphoreSubmitInfo`::pWaitSemaphoreValues must have a value which does not differ from the current value of the semaphore or the value of any outstanding semaphore wait or signal operation on that semaphore by more than `maxTimelineSemaphoreValueDifference`.

10. For each element of `pSignalSemaphores` created with a `VkSemaphoreType` of `VK_SEMAPHORE_TYPE_TIMELINE` the corresponding element of `VkTimelineSemaphoreSubmitInfo`::pSignalSemaphoreValues must have a value which does not differ from the current value of the semaphore or the value of any outstanding semaphore wait or signal operation on that semaphore by more than `maxTimelineSemaphoreValueDifference`.

11. If the mesh shaders feature is not enabled, each element of `pWaitDstStageMask` must not contain `VK_PIPELINE_STAGE_MESH_SHADER_BIT_NV`

12. If the task shaders feature is not enabled, each element of `pWaitDstStageMask` must not contain `VK_PIPELINE_STAGE_TASK_SHADER_BIT_NV`

\valboxend

### Swapchain

Validations for `vkAcquireNextImageKHR`:

\valbox

1. `swapchain` must not be in the retired state

2. If `semaphore` is not `VK_NULL_HANDLE` it must be unsignaled

3. If `semaphore` is not `VK_NULL_HANDLE` it must not have any uncompleted signal or wait operations pending

4. If `fence` is not `VK_NULL_HANDLE` it must be unsignaled and must not be associated with any other queue command that has not yet completed execution on that queue

\valcombox

5. `semaphore` and `fence` must not both be equal to `VK_NULL_HANDLE`
	- \valcom Handled by API design

\valcomboxend

6. If the number of currently acquired images is greater than the difference between the number of images in `swapchain` and the value of `VkSurfaceCapabilitiesKHR`::`minImageCount` as returned by a call to `vkGetPhysicalDeviceSurfaceCapabilities2KHR` with the `surface` used to create `swapchain`, `timeout` must not be `UINT64_MAX`

\valcombox

7. `semaphore` must have a `VkSemaphoreType` of `VK_SEMAPHORE_TYPE_BINARY`
	- \valcom Guaranteed by the type system

\valcomboxend

\valboxend

Validations for `vkQueuePresentKHR`:

\valbox

1. Each element of `pSwapchains` member of `pPresentInfo` must be a swapchain that is created for a surface for which presentation is supported from `queue` as determined using a call to `vkGetPhysicalDeviceSurfaceSupportKHR`

2. If more than one member of `pSwapchains` was created from a display surface, all display surfaces referenced that refer to the same display must use the same display mode

3. When a semaphore wait operation referring to a binary semaphore defined by the elements of the `pWaitSemaphores` member of `pPresentInfo` executes on `queue`, there must be no other queues waiting on the same semaphore.

4. All elements of the `pWaitSemaphores` member of `pPresentInfo` must be semaphores that are signaled, or have semaphore signal operations previously submitted for execution.

\valcombox

5. All elements of the `pWaitSemaphores` member of `pPresentInfo` must be created with a `VkSemaphoreType` of `VK_SEMAPHORE_TYPE_BINARY`.
	- \valcom Handled by API design

\valcomboxend

6. All elements of the `pWaitSemaphores` member of `pPresentInfo` must reference a semaphore signal operation that has been submitted for execution and any semaphore signal operations on which it depends (if any) must have also been submitted for execution.

\valboxend

Validations for `VkPresentInfoKHR`:

\valbox

\valcombox

1. Each element of `pImageIndices` must be the index of a presentable image acquired from the swapchain specified by the corresponding element of the `pSwapchains` array, and the presented image subresource must be in the `VK_IMAGE_LAYOUT_PRESENT_SRC_KHR` or `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR` layout at the time the operation is executed on a `VkDevice`
	- \valcom Guaranteed by the type system

\valcomboxend

2. All elements of the `pWaitSemaphores` must have a `VkSemaphoreType` of `VK_SEMAPHORE_TYPE_BINARY`

\valboxend

### Fence

Validations for `vkResetFences`:

\valbox

1. Each element of `pFences` must not be currently associated with any queue command that has not yet completed execution on that queue

\valboxend

### Image

Validations for `vkBindImageMemory`:

\valbox

1. `image` must not have been created with the `VK_IMAGE_CREATE_DISJOINT_BIT` set.

\valcombox

2. `image` must not already be backed by a memory object
	- \valcom Handled by API design

\valcomboxend

3. `image` must not have been created with any sparse memory binding flags

\valcombox

4. `memoryOffset` must be less than the size of `memory`
	- \valcom Handled by API design

5. `memory` must have been allocated using one of the memory types allowed in the `memoryTypeBits` member of the `VkMemoryRequirements` structure returned from a call to `vkGetImageMemoryRequirements` with `image`
	- \valcom Handled by API design

6. `memoryOffset` must be an integer multiple of the `alignment` member of the `VkMemoryRequirements` structure returned from a call to `vkGetImageMemoryRequirements` with `image`
	- \valcom Handled by API design

7. The difference of the size of `memory` and `memoryOffset` must be greater than or equal to the `size` member of the `VkMemoryRequirements` structure returned from a call to `vkGetImageMemoryRequirements` with the same `image`
	- \valcom Handled by API design

\valcomboxend

8. If `image` requires a dedicated allocation (as reported by `vkGetImageMemoryRequirements2` in `VkMemoryDedicatedRequirements`::requiresDedicatedAllocation for `image`), `memory` must have been created with `VkMemoryDedicatedAllocateInfo`::`image` equal to `image`

9. If the dedicated allocation image aliasing feature is not enabled, and the `VkMemoryAllocateInfo` provided when `memory` was allocated included a `VkMemoryDedicatedAllocateInfo` structure in its `pNext` chain, and `VkMemoryDedicatedAllocateInfo`::`image` was not `VK_NULL_HANDLE`, then `image` must equal `VkMemoryDedicatedAllocateInfo`::`image` and `memoryOffset` must be zero.

10. If the dedicated allocation image aliasing feature is enabled, and the `VkMemoryAllocateInfo` provided when `memory` was allocated included a `VkMemoryDedicatedAllocateInfo` structure in its `pNext` chain, and `VkMemoryDedicatedAllocateInfo`::`image` was not `VK_NULL_HANDLE`, then `memoryOffset` must be zero, and `image` must be either equal to `VkMemoryDedicatedAllocateInfo`::`image` or an image that was created using the same parameters in `VkImageCreateInfo`, with the exception that `extent` and `arrayLayers` may differ subject to the following restrictions: every dimension in the `extent` parameter of the image being bound must be equal to or smaller than the original image for which the allocation was created; and the `arrayLayers` parameter of the image being bound must be equal to or smaller than the original image for which the allocation was created.

11. If image was created with the `VK_IMAGE_CREATE_PROTECTED_BIT` bit set, the image must be bound to a memory object allocated with a memory type that reports `VK_MEMORY_PROPERTY_PROTECTED_BIT`

12. If image was created with the `VK_IMAGE_CREATE_PROTECTED_BIT` bit not set, the image must not be bound to a memory object created with a memory type that reports `VK_MEMORY_PROPERTY_PROTECTED_BIT`

13. If `image` was created with `VkDedicatedAllocationImageCreateInfoNV`::`dedicatedAllocation` equal to `VK_TRUE`, `memory` must have been created with `VkDedicatedAllocationMemoryAllocateInfoNV`::`image` equal to an image handle created with identical creation parameters to `image` and `memoryOffset` must be zero

14. If the value of `VkExportMemoryAllocateInfo`::`handleTypes` used to allocate `memory` is not `0`, it must include at least one of the handles set in `VkExternalMemoryImageCreateInfo`::`handleTypes` when `image` was created

15. If `memory` was created by a memory import operation, the external handle type of the imported memory must also have been set in `VkExternalMemoryImageCreateInfo`::`handleTypes` when `image` was created

\valboxend

### Buffer

Validations for `vkBindBufferMemory`:

\valbox

\valcombox

1. `buffer` must not already be backed by a memory object
	- \valcom Handled by API design

\valcomboxend

2. `buffer` must not have been created with any sparse memory binding flags

\valcombox

3. `memoryOffset` must be less than the size of `memory`
	- \valcom Handled by API design

4. `memory` must have been allocated using one of the memory types allowed in the `memoryTypeBits` member of the `VkMemoryRequirements` structure returned from a call to `vkGetBufferMemoryRequirements` with `buffer`
	- \valcom Handled by API design

5. `memoryOffset` must be an integer multiple of the `alignment` member of the `VkMemoryRequirements` structure returned from a call to `vkGetBufferMemoryRequirements` with `buffer`
	- \valcom Handled by API design

6. The `size` member of the `VkMemoryRequirements` structure returned from a call to `vkGetBufferMemoryRequirements` with `buffer` must be less than or equal to the size of `memory` minus `memoryOffset`
	- \valcom Handled by API design

\valcomboxend

7. If `buffer` requires a dedicated allocation(as reported by `vkGetBufferMemoryRequirements2` in `VkMemoryDedicatedRequirements`::requiresDedicatedAllocation for `buffer`), `memory` must have been created with `VkMemoryDedicatedAllocateInfo`::`buffer` equal to `buffer`

8. If the `VkMemoryAllocateInfo` provided when `memory` was allocated included a `VkMemoryDedicatedAllocateInfo` structure in its `pNext` chain, and `VkMemoryDedicatedAllocateInfo`::`buffer` was not `VK_NULL_HANDLE`, then `buffer` must equal `VkMemoryDedicatedAllocateInfo`::`buffer`, and `memoryOffset` must be zero.

9. If buffer was created with the `VK_BUFFER_CREATE_PROTECTED_BIT` bit set, the buffer must be bound to a memory object allocated with a memory type that reports `VK_MEMORY_PROPERTY_PROTECTED_BIT`

10. If buffer was created with the `VK_BUFFER_CREATE_PROTECTED_BIT` bit not set, the buffer must not be bound to a memory object created with a memory type that reports `VK_MEMORY_PROPERTY_PROTECTED_BIT`

11. If `buffer` was created with `VkDedicatedAllocationBufferCreateInfoNV`::`dedicatedAllocation` equal to `VK_TRUE`, `memory` must have been created with `VkDedicatedAllocationMemoryAllocateInfoNV`::`buffer` equal to a buffer handle created with identical creation parameters to `buffer` and `memoryOffset` must be zero

12. If the value of `VkExportMemoryAllocateInfo`::`handleTypes` used to allocate `memory` is not `0`, it must include at least one of the handles set in `VkExternalMemoryBufferCreateInfo`::`handleTypes` when `buffer` was created

13. If `memory` was created by a memory import operation, the external handle type of the imported memory must also have been set in `VkExternalMemoryBufferCreateInfo`::`handleTypes` when `buffer` was created

14. If the `VkPhysicalDeviceBufferDeviceAddressFeatures`::`bufferDeviceAddress` feature is enabled and `buffer` was created with the `VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT` bit set, `memory` must have been allocated with the `VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT` bit set

\valboxend

Validations for `vkMapMemory`:

\valbox

\valcombox

1. `memory` must not be currently host mapped
	- \valcom Handled by API design

\valcomboxend

2. `offset` must be less than the size of `memory`

\valcombox

3. If `size` is not equal to `VK_WHOLE_SIZE`, `size` must be greater than `0`
	- \valcom Guaranteed by the type system

\valcomboxend

4. If `size` is not equal to `VK_WHOLE_SIZE`, `size` must be less than or equal to the size of the `memory` minus `offset`

5. `memory` must have been created with a memory type that reports `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT`

6. `memory` must not have been allocated with multiple instances.

\valboxend

Validations for `VkMappedMemoryRange`:

\valbox

\valcombox

1. `memory` must be currently host mapped
	- \valcom Handled by API design

2. If `size` is not equal to `VK_WHOLE_SIZE`, `offset` and `size` must specify a range contained within the currently mapped range of `memory`
	- \valcom Handled by API design

3. If `size` is equal to `VK_WHOLE_SIZE`, `offset` must be within the currently mapped range of `memory`
	- \valcom Handled by API design

\valcomboxend

4. If `size` is equal to `VK_WHOLE_SIZE`, the end of the current mapping of `memory` must be a multiple of `VkPhysicalDeviceLimits`::`nonCoherentAtomSize` bytes from the beginning of the memory object.

5. `offset` must be a multiple of `VkPhysicalDeviceLimits`::`nonCoherentAtomSize`

6. If `size` is not equal to `VK_WHOLE_SIZE`, `size` must either be a multiple of `VkPhysicalDeviceLimits`::`nonCoherentAtomSize`, or `offset` plus `size` must equal the size of `memory`.

\valboxend

