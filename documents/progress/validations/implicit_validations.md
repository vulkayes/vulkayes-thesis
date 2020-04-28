### Instance

Validations for `vkCreateInstance`:

\valbox

\valcombox

1. `pCreateInfo` must be a valid pointer to a valid `VkInstanceCreateInfo` structure
	- \valcom Handled by API design (ash)

2. If `pAllocator` is not `NULL`, `pAllocator` must be a valid pointer to a valid `VkAllocationCallbacks` structure
	- \valcom Handled by API design (ash)

3. `pInstance` must be a valid pointer to a `VkInstance` handle
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `VkInstanceCreateInfo`:

\valbox

\valcombox

1. `sType` must be `VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO`
	- \valcom Handled by API design (ash)

2. Each `pNext` member of any structure (including this one) in the `pNext` chain must be either `NULL` or a pointer to a valid instance of `VkDebugReportCallbackCreateInfoEXT`, `VkDebugUtilsMessengerCreateInfoEXT`, `VkValidationFeaturesEXT`, or `VkValidationFlagsEXT`
	- \valcom Handled by API design (ash)

3. The `sType` value of each struct in the `pNext` chain must be unique
	- \valcom Handled by API design

4. `flags` must be `0`
	- \valcom Handled by API design (ash)

5. If `pApplicationInfo` is not `NULL`, `pApplicationInfo` must be a valid pointer to a valid `VkApplicationInfo` structure
	- \valcom Handled by API design (ash)

\valcomboxend

\valdonebox

6. If `enabledLayerCount` is not `0`, `ppEnabledLayerNames` must be a valid pointer to an array of `enabledLayerCount` null-terminated UTF-8 strings
	- \valdone Returns error

7. If `enabledExtensionCount` is not `0`, `ppEnabledExtensionNames` must be a valid pointer to an array of `enabledExtensionCount` null-terminated UTF-8 strings
	- \valdone Returns error

\valdoneboxend

\valboxend

### Device

Validations for `vkCreateDevice`:

\valbox

\valcombox

1. `physicalDevice` must be a valid `VkPhysicalDevice` handle
	- \valcom Handled by API design (ash)

2. `pCreateInfo` must be a valid pointer to a valid `VkDeviceCreateInfo` structure
	- \valcom Handled by API design (ash)

3. If `pAllocator` is not `NULL`, `pAllocator` must be a valid pointer to a valid `VkAllocationCallbacks` structure
	- \valcom Handled by API design (ash)

4. `pDevice` must be a valid pointer to a `VkDevice` handle
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `VkDeviceCreateInfo`:

\valbox

\valcombox

1. `sType` must be `VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO`
	- \valcom Handled by API design (ash)

2. Each `pNext` member of any structure (including this one) in the `pNext` chain must be either `NULL` or a pointer to a valid instance of `VkDeviceDiagnosticsConfigCreateInfoNV`, `VkDeviceGroupDeviceCreateInfo`, `VkDeviceMemoryOverallocationCreateInfoAMD`, `VkPhysicalDevice16BitStorageFeatures`, `VkPhysicalDevice8BitStorageFeatures`, `VkPhysicalDeviceASTCDecodeFeaturesEXT`, `VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT`, `VkPhysicalDeviceBufferDeviceAddressFeatures`, `VkPhysicalDeviceBufferDeviceAddressFeaturesEXT`, `VkPhysicalDeviceCoherentMemoryFeaturesAMD`, `VkPhysicalDeviceComputeShaderDerivativesFeaturesNV`, `VkPhysicalDeviceConditionalRenderingFeaturesEXT`, `VkPhysicalDeviceCooperativeMatrixFeaturesNV`, `VkPhysicalDeviceCornerSampledImageFeaturesNV`, `VkPhysicalDeviceCoverageReductionModeFeaturesNV`, `VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV`, `VkPhysicalDeviceDepthClipEnableFeaturesEXT`, `VkPhysicalDeviceDescriptorIndexingFeatures`, `VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV`, `VkPhysicalDeviceDiagnosticsConfigFeaturesNV`, `VkPhysicalDeviceExclusiveScissorFeaturesNV`, `VkPhysicalDeviceFeatures2`, `VkPhysicalDeviceFragmentDensityMapFeaturesEXT`, `VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV`, `VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT`, `VkPhysicalDeviceHostQueryResetFeatures`, `VkPhysicalDeviceImagelessFramebufferFeatures`, `VkPhysicalDeviceIndexTypeUint8FeaturesEXT`, `VkPhysicalDeviceInlineUniformBlockFeaturesEXT`, `VkPhysicalDeviceLineRasterizationFeaturesEXT`, `VkPhysicalDeviceMemoryPriorityFeaturesEXT`, `VkPhysicalDeviceMeshShaderFeaturesNV`, `VkPhysicalDeviceMultiviewFeatures`, `VkPhysicalDevicePerformanceQueryFeaturesKHR`, `VkPhysicalDevicePipelineCreationCacheControlFeaturesEXT`, `VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR`, `VkPhysicalDeviceProtectedMemoryFeatures`, `VkPhysicalDeviceRayTracingFeaturesKHR`, `VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV`, `VkPhysicalDeviceSamplerYcbcrConversionFeatures`, `VkPhysicalDeviceScalarBlockLayoutFeatures`, `VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures`, `VkPhysicalDeviceShaderAtomicInt64Features`, `VkPhysicalDeviceShaderClockFeaturesKHR`, `VkPhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT`, `VkPhysicalDeviceShaderDrawParametersFeatures`, `VkPhysicalDeviceShaderFloat16Int8Features`, `VkPhysicalDeviceShaderImageFootprintFeaturesNV`, `VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL`, `VkPhysicalDeviceShaderSMBuiltinsFeaturesNV`, `VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures`, `VkPhysicalDeviceShadingRateImageFeaturesNV`, `VkPhysicalDeviceSubgroupSizeControlFeaturesEXT`, `VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT`, `VkPhysicalDeviceTextureCompressionASTCHDRFeaturesEXT`, `VkPhysicalDeviceTimelineSemaphoreFeatures`, `VkPhysicalDeviceTransformFeedbackFeaturesEXT`, `VkPhysicalDeviceUniformBufferStandardLayoutFeatures`, `VkPhysicalDeviceVariablePointersFeatures`, `VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT`, `VkPhysicalDeviceVulkan11Features`, `VkPhysicalDeviceVulkan12Features`, `VkPhysicalDeviceVulkanMemoryModelFeatures`, or `VkPhysicalDeviceYcbcrImageArraysFeaturesEXT`
	- \valcom Handled by API design (ash)

3. The `sType` value of each struct in the `pNext` chain must be unique
	- \valcom Handled by API design

4. `flags` must be `0`
	- \valcom Handled by API design (ash)

5. `pQueueCreateInfos` must be a valid pointer to an array of `queueCreateInfoCount` valid `VkDeviceQueueCreateInfo` structures
	- \valcom Handled by API design (ash)

\valcomboxend

\valdonebox

6. If `enabledLayerCount` is not `0`, `ppEnabledLayerNames` must be a valid pointer to an array of `enabledLayerCount` null-terminated UTF-8 strings
	- \valdone Returns error

7. If `enabledExtensionCount` is not `0`, `ppEnabledExtensionNames` must be a valid pointer to an array of `enabledExtensionCount` null-terminated UTF-8 strings
	- \valdone Returns error

\valdoneboxend

\valcombox

8. If `pEnabledFeatures` is not `NULL`, `pEnabledFeatures` must be a valid pointer to a valid `VkPhysicalDeviceFeatures` structure
	- \valcom Handled by API design (ash)

\valcomboxend

\valdonebox

9. `queueCreateInfoCount` must be greater than `0`
	- \valdone Returns error

\valdoneboxend

\valboxend

### Queue

Validations for `VkDeviceQueueCreateInfo`:

\valbox

\valcombox

1. `sType` must be `VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO`
	- \valcom Handled by API design (ash)

2. `pNext` must be `NULL` or a pointer to a valid instance of `VkDeviceQueueGlobalPriorityCreateInfoEXT`
	- \valcom Handled by API design (ash)

3. The `sType` value of each struct in the `pNext` chain must be unique
	- \valcom Handled by API design

4. `flags` must be a valid combination of `VkDeviceQueueCreateFlagBits` values
	- \valcom Handled by API design (ash)

5. `pQueuePriorities` must be a valid pointer to an array of `queueCount` `float` values
	- \valcom Handled by API design (ash)

\valcomboxend

\valdonebox

6. `queueCount` must be greater than `0`
	- \valdone Returns error

\valdoneboxend

\valboxend

Validations for `vkGetDeviceQueue`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

2. `pQueue` must be a valid pointer to a `VkQueue` handle
	- \valcom Handled by API design

\valcomboxend

\valboxend

Validations for `vkGetDeviceQueue2`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

2. `pQueueInfo` must be a valid pointer to a valid `VkDeviceQueueInfo2` structure
	- \valcom Handled by API design

3. `pQueue` must be a valid pointer to a `VkQueue` handle
	- \valcom Handled by API design

\valcomboxend

\valboxend

Validations for `VkDeviceQueueInfo2`:

\valbox

\valcombox

1. `sType` must be `VK_STRUCTURE_TYPE_DEVICE_QUEUE_INFO_2`
	- \valcom Handled by API design (ash)

2. `pNext` must be `NULL`
	- \valcom Handled by API design (ash)

3. `flags` must be a valid combination of `VkDeviceQueueCreateFlagBits` values
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `vkQueueSubmit`:

\valbox

\valcombox

1. `queue` must be a valid `VkQueue` handle
	- \valcom Handled by API design

2. If `submitCount` is not `0`, `pSubmits` must be a valid pointer to an array of `submitCount` valid `VkSubmitInfo` structures
	- \valcom Handled by API design

3. If `fence` is not `VK_NULL_HANDLE`, `fence` must be a valid `VkFence` handle
	- \valcom Handled by API design

\valcomboxend

\valdonebox

4. Both of `fence`, and `queue` that are valid handles of non-ignored parameters must have been created, allocated, or retrieved from the same `VkDevice`
	- \valdone Returns error

\valdoneboxend

\valboxend

Validations for `VkSubmitInfo`:

\valbox

\valcombox

1. `sType` must be `VK_STRUCTURE_TYPE_SUBMIT_INFO`
	- \valcom Handled by API design (ash)

2. Each `pNext` member of any structure (including this one) in the `pNext` chain must be either `NULL` or a pointer to a valid instance of `VkD3D12FenceSubmitInfoKHR`, `VkDeviceGroupSubmitInfo`, `VkPerformanceQuerySubmitInfoKHR`, `VkProtectedSubmitInfo`, `VkTimelineSemaphoreSubmitInfo`, `VkWin32KeyedMutexAcquireReleaseInfoKHR`, or `VkWin32KeyedMutexAcquireReleaseInfoNV`
	- \valcom Handled by API design (ash)

3. The `sType` value of each struct in the `pNext` chain must be unique
	- \valcom Handled by API design

4. If `waitSemaphoreCount` is not `0`, `pWaitSemaphores` must be a valid pointer to an array of `waitSemaphoreCount` valid `VkSemaphore` handles
	- \valcom Handled by API design (ash)

5. If `waitSemaphoreCount` is not `0`, `pWaitDstStageMask` must be a valid pointer to an array of `waitSemaphoreCount` valid combinations of `VkPipelineStageFlagBits` values
	- \valcom Handled by API design (ash)

6. Each element of `pWaitDstStageMask` must not be `0`
	- \valcom Handled by API design

7. If `commandBufferCount` is not `0`, `pCommandBuffers` must be a valid pointer to an array of `commandBufferCount` valid `VkCommandBuffer` handles
	- \valcom Handled by API design (ash)

8. If `signalSemaphoreCount` is not `0`, `pSignalSemaphores` must be a valid pointer to an array of `signalSemaphoreCount` valid `VkSemaphore` handles
	- \valcom Handled by API design (ash)

\valcomboxend

\valdonebox

9. Each of the elements of `pCommandBuffers`, the elements of `pSignalSemaphores`, and the elements of `pWaitSemaphores` that are valid handles of non-ignored parameters must have been created, allocated, or retrieved from the same `VkDevice`
	- \valdone Returns error

\valdoneboxend

\valboxend

### Swapchain

Validations for `vkCreateSwapchainKHR`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design (ash)

2. `pCreateInfo` must be a valid pointer to a valid `VkSwapchainCreateInfoKHR` structure
	- \valcom Handled by API design (ash)

3. If `pAllocator` is not `NULL`, `pAllocator` must be a valid pointer to a valid `VkAllocationCallbacks` structure
	- \valcom Handled by API design (ash)

4. `pSwapchain` must be a valid pointer to a `VkSwapchainKHR` handle
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `VkSwapchainCreateInfoKHR`:

\valbox

\valcombox

1. `sType` must be `VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR`
	- \valcom Handled by API design (ash)

2. Each `pNext` member of any structure (including this one) in the `pNext` chain must be either `NULL` or a pointer to a valid instance of `VkDeviceGroupSwapchainCreateInfoKHR`, `VkImageFormatListCreateInfo`, `VkSurfaceFullScreenExclusiveInfoEXT`, `VkSurfaceFullScreenExclusiveWin32InfoEXT`, `VkSwapchainCounterCreateInfoEXT`, or `VkSwapchainDisplayNativeHdrCreateInfoAMD`
	- \valcom Handled by API design (ash)

3. The `sType` value of each struct in the `pNext` chain must be unique
	- \valcom Handled by API design

4. `flags` must be a valid combination of `VkSwapchainCreateFlagBitsKHR` values
	- \valcom Handled by API design (ash)

5. `surface` must be a valid `VkSurfaceKHR` handle
	- \valcom Handled by API design (ash)

6. `imageFormat` must be a valid `VkFormat` value
	- \valcom Handled by API design (ash)

7. `imageColorSpace` must be a valid `VkColorSpaceKHR` value
	- \valcom Handled by API design (ash)

8. `imageUsage` must be a valid combination of `VkImageUsageFlagBits` values
	- \valcom Handled by API design (ash)

\valcomboxend

\valdonebox

9. `imageUsage` must not be `0`
	- \valdone Returns error

\valdoneboxend

\valcombox

10. `imageSharingMode` must be a valid `VkSharingMode` value
	- \valcom Handled by API design (ash)

11. `preTransform` must be a valid `VkSurfaceTransformFlagBitsKHR` value
	- \valcom Handled by API design (ash)

12. `compositeAlpha` must be a valid `VkCompositeAlphaFlagBitsKHR` value
	- \valcom Handled by API design (ash)

13. `presentMode` must be a valid `VkPresentModeKHR` value
	- \valcom Handled by API design (ash)

14. If `oldSwapchain` is not `VK_NULL_HANDLE`, `oldSwapchain` must be a valid `VkSwapchainKHR` handle
	- \valcom Handled by API design (ash)

15. If `oldSwapchain` is a valid handle, it must have been created, allocated, or retrieved from `surface`
	- \valcom Handled by API design

16. Both of `oldSwapchain`, and `surface` that are valid handles of non-ignored parameters must have been created, allocated, or retrieved from the same `VkInstance`
	- \valcom Handled by API design

\valcomboxend

\valboxend

Validations for `vkGetSwapchainImagesKHR`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

2. `swapchain` must be a valid `VkSwapchainKHR` handle
	- \valcom Handled by API design

3. `pSwapchainImageCount` must be a valid pointer to a `uint32_t` value
	- \valcom Handled by API design (ash)

4. If the value referenced by `pSwapchainImageCount` is not `0`, and `pSwapchainImages` is not `NULL`, `pSwapchainImages` must be a valid pointer to an array of `pSwapchainImageCount` `VkImage` handles
	- \valcom Handled by API design (ash)

5. Both of `device`, and `swapchain` must have been created, allocated, or retrieved from the same `VkInstance`
	- \valcom Handled by API design

\valcomboxend

\valboxend

Validations for `vkQueuePresentKHR`:

\valbox

\valcombox

1. `queue` must be a valid `VkQueue` handle
	- \valcom Handled by API design (ash)

2. `pPresentInfo` must be a valid pointer to a valid `VkPresentInfoKHR` structure
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `VkPresentInfoKHR`:

\valbox

\valcombox

1. `sType` must be `VK_STRUCTURE_TYPE_PRESENT_INFO_KHR`
	- \valcom Handled by API design (ash)

2. Each `pNext` member of any structure (including this one) in the `pNext` chain must be either `NULL` or a pointer to a valid instance of `VkDeviceGroupPresentInfoKHR`, `VkDisplayPresentInfoKHR`, `VkPresentFrameTokenGGP`, `VkPresentRegionsKHR`, or `VkPresentTimesInfoGOOGLE`
	- \valcom Handled by API design (ash)

3. The `sType` value of each struct in the `pNext` chain must be unique
	- \valcom Handled by API design

4. If `waitSemaphoreCount` is not `0`, `pWaitSemaphores` must be a valid pointer to an array of `waitSemaphoreCount` valid `VkSemaphore` handles
	- \valcom Handled by API design (ash)

5. `pSwapchains` must be a valid pointer to an array of `swapchainCount` valid `VkSwapchainKHR` handles
	- \valcom Handled by API design (ash)

6. `pImageIndices` must be a valid pointer to an array of `swapchainCount` `uint32_t` values
	- \valcom Handled by API design (ash)

7. If `pResults` is not `NULL`, `pResults` must be a valid pointer to an array of `swapchainCount` `VkResult` values
	- \valcom Handled by API design (ash)

\valcomboxend

\valdonebox

8. `swapchainCount` must be greater than `0`
	- \valdone Returns error

9. Both of the elements of `pSwapchains`, and the elements of `pWaitSemaphores` that are valid handles of non-ignored parameters must have been created, allocated, or retrieved from the same `VkInstance`
	- \valdone Returns error

\valdoneboxend

\valboxend

Validations for `vkAcquireNextImageKHR`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

2. `swapchain` must be a valid `VkSwapchainKHR` handle
	- \valcom Handled by API design

3. If `semaphore` is not `VK_NULL_HANDLE`, `semaphore` must be a valid `VkSemaphore` handle
	- \valcom Handled by API design

4. If `fence` is not `VK_NULL_HANDLE`, `fence` must be a valid `VkFence` handle
	- \valcom Handled by API design

5. `pImageIndex` must be a valid pointer to a `uint32_t` value
	- \valcom Handled by API design (ash)

\valcomboxend

\valdonebox

6. If `semaphore` is a valid handle, it must have been created, allocated, or retrieved from `device`
	- \valdone Returns error

7. If `fence` is a valid handle, it must have been created, allocated, or retrieved from `device`
	- \valdone Returns error

\valdoneboxend

\valcombox

8. Both of `device`, and `swapchain` that are valid handles of non-ignored parameters must have been created, allocated, or retrieved from the same `VkInstance`
	- \valcom Handled by API design

\valcomboxend

\valboxend

### Command Buffer

Validations for `vkCreateCommandPool`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

2. `pCreateInfo` must be a valid pointer to a valid `VkCommandPoolCreateInfo` structure
	- \valcom Handled by API design (ash)

3. If `pAllocator` is not `NULL`, `pAllocator` must be a valid pointer to a valid `VkAllocationCallbacks` structure
	- \valcom Handled by API design

4. `pCommandPool` must be a valid pointer to a `VkCommandPool` handle
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `VkCommandPoolCreateInfo`:

\valbox

\valcombox

1. `sType` must be `VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO`
	- \valcom Handled by API design (ash)

2. `pNext` must be `NULL`
	- \valcom Handled by API design (ash)

3. `flags` must be a valid combination of `VkCommandPoolCreateFlagBits` values
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `vkTrimCommandPool`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

2. `commandPool` must be a valid `VkCommandPool` handle
	- \valcom Handled by API design

3. `flags` must be `0`
	- \valcom Handled by API design

4. `commandPool` must have been created, allocated, or retrieved from `device`
	- \valcom Handled by API design

\valcomboxend

\valboxend

Validations for `vkResetCommandPool`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

2. `commandPool` must be a valid `VkCommandPool` handle
	- \valcom Handled by API design

3. `flags` must be a valid combination of `VkCommandPoolResetFlagBits` values
	- \valcom Handled by API design

4. `commandPool` must have been created, allocated, or retrieved from `device`
	- \valcom Handled by API design

\valcomboxend

\valboxend

Validations for `VkCommandBufferAllocateInfo`:

\valbox

\valcombox

1. `sType` must be `VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO`
	- \valcom Handled by API design (ash)

2. `pNext` must be `NULL`
	- \valcom Handled by API design (ash)

3. `commandPool` must be a valid `VkCommandPool` handle
	- \valcom Handled by API design (ash)

4. `level` must be a valid `VkCommandBufferLevel` value
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

### Fence

Validations for `vkCreateFence`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design (ash)

2. `pCreateInfo` must be a valid pointer to a valid `VkFenceCreateInfo` structure
	- \valcom Handled by API design (ash)

3. If `pAllocator` is not `NULL`, `pAllocator` must be a valid pointer to a valid `VkAllocationCallbacks` structure
	- \valcom Handled by API design (ash)

4. `pFence` must be a valid pointer to a `VkFence` handle
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `VkFenceCreateInfo`:

\valbox

\valcombox

1. `sType` must be `VK_STRUCTURE_TYPE_FENCE_CREATE_INFO`
	- \valcom Handled by API design (ash)

2. Each `pNext` member of any structure (including this one) in the `pNext` chain must be either `NULL` or a pointer to a valid instance of `VkExportFenceCreateInfo` or `VkExportFenceWin32HandleInfoKHR`
	- \valcom Handled by API design (ash)

3. The `sType` value of each struct in the `pNext` chain must be unique
	- \valcom Handled by API design

4. `flags` must be a valid combination of `VkFenceCreateFlagBits` values
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `vkGetFenceStatus`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

2. `fence` must be a valid `VkFence` handle
	- \valcom Handled by API design

3. `fence` must have been created, allocated, or retrieved from `device`
	- \valcom Handled by API design

\valcomboxend

\valboxend

Validations for `vkResetFences`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

2. `pFences` must be a valid pointer to an array of `fenceCount` valid `VkFence` handles
	- \valcom Handled by API design

3. `fenceCount` must be greater than `0`
	- \valcom Handled by API design

4. Each element of `pFences` must have been created, allocated, or retrieved from `device`
	- \valcom Handled by API design

\valcomboxend

\valboxend

Validations for `vkWaitForFences`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

2. `pFences` must be a valid pointer to an array of `fenceCount` valid `VkFence` handles
	- \valcom Handled by API design

3. `fenceCount` must be greater than `0`
	- \valcom Handled by API design

4. Each element of `pFences` must have been created, allocated, or retrieved from `device`
	- \valcom Handled by API design

\valcomboxend

\valboxend

### Sempahore

Validations for `vkCreateSemaphore`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design (ash)

2. `pCreateInfo` must be a valid pointer to a valid `VkSemaphoreCreateInfo` structure
	- \valcom Handled by API design (ash)

3. If `pAllocator` is not `NULL`, `pAllocator` must be a valid pointer to a valid `VkAllocationCallbacks` structure
	- \valcom Handled by API design (ash)

4. `pSemaphore` must be a valid pointer to a `VkSemaphore` handle
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `VkSemaphoreCreateInfo`:

\valbox

\valcombox

1. `sType` must be `VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO`
	- \valcom Handled by API design (ash)

2. Each `pNext` member of any structure (including this one) in the `pNext` chain must be either `NULL` or a pointer to a valid instance of `VkExportSemaphoreCreateInfo`, `VkExportSemaphoreWin32HandleInfoKHR`, or `VkSemaphoreTypeCreateInfo`
	- \valcom Handled by API design (ash)

3. The `sType` value of each struct in the `pNext` chain must be unique
	- \valcom Handled by API design

4. `flags` must be `0`
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `VkSemaphoreTypeCreateInfo`:

\valbox

\valcombox

1. `sType` must be `VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO`
	- \valcom Handled by API design (ash)

2. `semaphoreType` must be a valid `VkSemaphoreType` value
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

### Image

Validations for `vkCreateImage`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

2. `pCreateInfo` must be a valid pointer to a valid `VkImageCreateInfo` structure
	- \valcom Handled by API design (ash)

3. If `pAllocator` is not `NULL`, `pAllocator` must be a valid pointer to a valid `VkAllocationCallbacks` structure
	- \valcom Handled by API design

4. `pImage` must be a valid pointer to a `VkImage` handle
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `VkImageCreateInfo`:

\valbox

\valcombox

1. `sType` must be `VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO`
	- \valcom Handled by API design (ash)

2. Each `pNext` member of any structure (including this one) in the `pNext` chain must be either `NULL` or a pointer to a valid instance of `VkDedicatedAllocationImageCreateInfoNV`, `VkExternalFormatANDROID`, `VkExternalMemoryImageCreateInfo`, `VkExternalMemoryImageCreateInfoNV`, `VkImageDrmFormatModifierExplicitCreateInfoEXT`, `VkImageDrmFormatModifierListCreateInfoEXT`, `VkImageFormatListCreateInfo`, `VkImageStencilUsageCreateInfo`, or `VkImageSwapchainCreateInfoKHR`
	- \valcom Handled by API design (ash)

3. The `sType` value of each struct in the `pNext` chain must be unique
	- \valcom Handled by API design

4. `flags` must be a valid combination of `VkImageCreateFlagBits` values
	- \valcom Handled by API design

5. `imageType` must be a valid `VkImageType` value
	- \valcom Handled by API design (ash)

6. `format` must be a valid `VkFormat` value
	- \valcom Handled by API design (ash)

7. `samples` must be a valid `VkSampleCountFlagBits` value
	- \valcom Handled by API design (ash)

8. `tiling` must be a valid `VkImageTiling` value
	- \valcom Handled by API design (ash)

9. `usage` must be a valid combination of `VkImageUsageFlagBits` values
	- \valcom Handled by API design

\valcomboxend

\valdonebox

10. `usage` must not be `0`
	- \valdone Returns error

\valdoneboxend

\valcombox

11. `sharingMode` must be a valid `VkSharingMode` value
	- \valcom Handled by API design (ash)

12. `initialLayout` must be a valid `VkImageLayout` value
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `vkBindImageMemory`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

2. `image` must be a valid `VkImage` handle
	- \valcom Handled by API design

3. `memory` must be a valid `VkDeviceMemory` handle
	- \valcom Handled by API design

4. `image` must have been created, allocated, or retrieved from `device`
	- \valcom Handled by API design

\valcomboxend

\valdonebox

5. `memory` must have been created, allocated, or retrieved from `device`
	- \valdone Returns error

\valdoneboxend

\valboxend

Validations for `vkCreateImageView`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

2. `pCreateInfo` must be a valid pointer to a valid `VkImageViewCreateInfo` structure
	- \valcom Handled by API design (ash)

3. If `pAllocator` is not `NULL`, `pAllocator` must be a valid pointer to a valid `VkAllocationCallbacks` structure
	- \valcom Handled by API design

4. `pView` must be a valid pointer to a `VkImageView` handle
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `VkImageViewCreateInfo`:

\valbox

\valcombox

1. `sType` must be `VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO`
	- \valcom Handled by API design (ash)

2. Each `pNext` member of any structure (including this one) in the `pNext` chain must be either `NULL` or a pointer to a valid instance of `VkImageViewASTCDecodeModeEXT`, `VkImageViewUsageCreateInfo`, or `VkSamplerYcbcrConversionInfo`
	- \valcom Handled by API design (ash)

3. The `sType` value of each struct in the `pNext` chain must be unique
	- \valcom Handled by API design

4. `flags` must be a valid combination of `VkImageViewCreateFlagBits` values
	- \valcom Handled by API design

5. `image` must be a valid `VkImage` handle
	- \valcom Handled by API design

6. `viewType` must be a valid `VkImageViewType` value
	- \valcom Handled by API design

7. `format` must be a valid `VkFormat` value
	- \valcom Handled by API design

8. `components` must be a valid `VkComponentMapping` structure
	- \valcom Handled by API design

9. `subresourceRange` must be a valid `VkImageSubresourceRange` structure
	- \valcom Handled by API design

\valcomboxend

\valboxend

### Buffer

Validations for `vkCreateBuffer`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

2. `pCreateInfo` must be a valid pointer to a valid `VkBufferCreateInfo` structure
	- \valcom Handled by API design (ash)

3. If `pAllocator` is not `NULL`, `pAllocator` must be a valid pointer to a valid `VkAllocationCallbacks` structure
	- \valcom Handled by API design

4. `pBuffer` must be a valid pointer to a `VkBuffer` handle
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `VkBufferCreateInfo`:

\valbox

\valcombox

1. `sType` must be `VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO`
	- \valcom Handled by API design (ash)

2. Each `pNext` member of any structure (including this one) in the `pNext` chain must be either `NULL` or a pointer to a valid instance of `VkBufferDeviceAddressCreateInfoEXT`, `VkBufferOpaqueCaptureAddressCreateInfo`, `VkDedicatedAllocationBufferCreateInfoNV`, or `VkExternalMemoryBufferCreateInfo`
	- \valcom Handled by API design (ash)

3. The `sType` value of each struct in the `pNext` chain must be unique
	- \valcom Handled by API design

4. `flags` must be a valid combination of `VkBufferCreateFlagBits` values
	- \valcom Handled by API design

5. `usage` must be a valid combination of `VkBufferUsageFlagBits` values
	- \valcom Handled by API design

\valcomboxend

\valdonebox

6. `usage` must not be `0`
	- \valdone Return error

\valdoneboxend

\valcombox

7. `sharingMode` must be a valid `VkSharingMode` value
	- \valcom Handled by API design

\valcomboxend

\valboxend

Validations for `vkBindBufferMemory`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

2. `buffer` must be a valid `VkBuffer` handle
	- \valcom Handled by API design

3. `memory` must be a valid `VkDeviceMemory` handle
	- \valcom Handled by API design

4. `buffer` must have been created, allocated, or retrieved from `device`
	- \valcom Handled by API design

\valcomboxend

\valdonebox

5. `memory` must have been created, allocated, or retrieved from `device`
	- \valdone Returns error

\valdoneboxend

\valboxend

Validations for `vkCreateBufferView`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

2. `pCreateInfo` must be a valid pointer to a valid `VkBufferViewCreateInfo` structure
	- \valcom Handled by API design (ash)

3. If `pAllocator` is not `NULL`, `pAllocator` must be a valid pointer to a valid `VkAllocationCallbacks` structure
	- \valcom Handled by API design

4. `pView` must be a valid pointer to a `VkBufferView` handle
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `VkBufferViewCreateInfo`:

\valbox

\valcombox

1. `sType` must be `VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO`
	- \valcom Handled by API design (ash)

2. `pNext` must be `NULL`
	- \valcom Handled by API design (ash)

3. `flags` must be `0`
	- \valcom Handled by API design

4. `buffer` must be a valid `VkBuffer` handle
	- \valcom Handled by API design (ash)

5. `format` must be a valid `VkFormat` value
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

### Memory

Validations for `vkMapMemory`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

2. `memory` must be a valid `VkDeviceMemory` handle
	- \valcom Handled by API design

3. `flags` must be `0`
	- \valcom Handled by API design

4. `ppData` must be a valid pointer to a pointer value
	- \valcom Handled by API design (ash)

5. `memory` must have been created, allocated, or retrieved from `device`
	- \valcom Handled by API design

\valcomboxend

\valboxend

Validations for `vkFlushMappedMemoryRanges`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

2. `pMemoryRanges` must be a valid pointer to an array of `memoryRangeCount` valid `VkMappedMemoryRange` structures
	- \valcom Handled by API design

3. `memoryRangeCount` must be greater than `0`
	- \valcom Handled by API design

\valcomboxend

\valboxend

Validations for `vkInvalidateMappedMemoryRanges`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

2. `pMemoryRanges` must be a valid pointer to an array of `memoryRangeCount` valid `VkMappedMemoryRange` structures
	- \valcom Handled by API design

3. `memoryRangeCount` must be greater than `0`
	- \valcom Handled by API design

\valcomboxend

\valboxend

Validations for `VkMappedMemoryRange`:

\valbox

\valcombox

1. `sType` must be `VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE`
	- \valcom Handled by API design (ash)

2. `pNext` must be `NULL`
	- \valcom Handled by API design (ash)

3. `memory` must be a valid `VkDeviceMemory` handle
	- \valcom Handled by API design

\valcomboxend

\valboxend

### Descriptors

Validations for `vkCreateDescriptorSetLayout`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

2. `pCreateInfo` must be a valid pointer to a valid `VkDescriptorSetLayoutCreateInfo` structure
	- \valcom Handled by API design

3. If `pAllocator` is not `NULL`, `pAllocator` must be a valid pointer to a valid `VkAllocationCallbacks` structure
	- \valcom Handled by API design

4. `pSetLayout` must be a valid pointer to a `VkDescriptorSetLayout` handle
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `VkDescriptorSetLayoutCreateInfo`:

\valbox

\valcombox

1. `sType` must be `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO`
	- \valcom Handled by API design (ash)

2. `pNext` must be `NULL` or a pointer to a valid instance of `VkDescriptorSetLayoutBindingFlagsCreateInfo`
	- \valcom Handled by API design (ash)

3. The `sType` value of each struct in the `pNext` chain must be unique
	- \valcom Handled by API design

4. `flags` must be a valid combination of `VkDescriptorSetLayoutCreateFlagBits` values
	- \valcom Handled by API design

5. If `bindingCount` is not `0`, `pBindings` must be a valid pointer to an array of `bindingCount` valid `VkDescriptorSetLayoutBinding` structures
	- \valcom Handled by API design

\valcomboxend

\valboxend

Validations for `VkDescriptorSetLayoutBinding`:

\valbox

\valcombox

1. `descriptorType` must be a valid `VkDescriptorType` value
	- \valcom Handled by API design

\valcomboxend

\valboxend

Validations for `vkCreateDescriptorPool`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

2. `pCreateInfo` must be a valid pointer to a valid `VkDescriptorPoolCreateInfo` structure
	- \valcom Handled by API design (ash)

3. If `pAllocator` is not `NULL`, `pAllocator` must be a valid pointer to a valid `VkAllocationCallbacks` structure
	- \valcom Handled by API design

4. `pDescriptorPool` must be a valid pointer to a `VkDescriptorPool` handle
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `VkDescriptorPoolCreateInfo`:

\valbox

\valcombox

1. `sType` must be `VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO`
	- \valcom Handled by API design (ash)

2. `pNext` must be `NULL` or a pointer to a valid instance of `VkDescriptorPoolInlineUniformBlockCreateInfoEXT`
	- \valcom Handled by API design (ash)

3. The `sType` value of each struct in the `pNext` chain must be unique
	- \valcom Handled by API design

4. `flags` must be a valid combination of `VkDescriptorPoolCreateFlagBits` values
	- \valcom Handled by API design (ash)

5. `pPoolSizes` must be a valid pointer to an array of `poolSizeCount` valid `VkDescriptorPoolSize` structures
	- \valcom Handled by API design (ash)

6. `poolSizeCount` must be greater than `0`
	- \valcom Guaranteed by the type system

\valcomboxend

\valboxend

Validations for `VkDescriptorPoolInlineUniformBlockCreateInfoEXT`:

\valbox

\valcombox

1. `sType` must be `VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO_EXT`
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `vkAllocateDescriptorSets`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

2. `pAllocateInfo` must be a valid pointer to a valid `VkDescriptorSetAllocateInfo` structure
	- \valcom Handled by API design

3. `pDescriptorSets` must be a valid pointer to an array of `pAllocateInfo`::descriptorSetCount `VkDescriptorSet` handles
	- \valcom Handled by API design (ash)

4. The value referenced by `pAllocateInfo`::`descriptorSetCount` must be greater than `0`
	- \valcom Guaranteed by the type system

\valcomboxend

\valboxend

Validations for `VkDescriptorSetAllocateInfo`:

\valbox

\valcombox

1. `sType` must be `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO`
	- \valcom Handled by API design (ash)

2. `pNext` must be `NULL` or a pointer to a valid instance of `VkDescriptorSetVariableDescriptorCountAllocateInfo`
	- \valcom Handled by API design (ash)

3. The `sType` value of each struct in the `pNext` chain must be unique
	- \valcom Handled by API design

4. `descriptorPool` must be a valid `VkDescriptorPool` handle
	- \valcom Handled by API design

5. `pSetLayouts` must be a valid pointer to an array of `descriptorSetCount` valid `VkDescriptorSetLayout` handles
	- \valcom Handled by API design

\valcomboxend

\valdonebox

6. `descriptorSetCount` must be greater than `0`
	- \valdone Returns error

7. Both of `descriptorPool`, and the elements of `pSetLayouts` must have been created, allocated, or retrieved from the same `VkDevice`
	- \valdone Returns error

\valdoneboxend

\valboxend

Validations for `vkCreateSampler`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

2. `pCreateInfo` must be a valid pointer to a valid `VkSamplerCreateInfo` structure
	- \valcom Handled by API design

3. If `pAllocator` is not `NULL`, `pAllocator` must be a valid pointer to a valid `VkAllocationCallbacks` structure
	- \valcom Handled by API design

4. `pSampler` must be a valid pointer to a `VkSampler` handle
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `VkSamplerCreateInfo`:

\valbox

\valcombox

1. `sType` must be `VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO`
	- \valcom Handled by API design (ash)

2. Each `pNext` member of any structure (including this one) in the `pNext` chain must be either `NULL` or a pointer to a valid instance of `VkSamplerReductionModeCreateInfo` or `VkSamplerYcbcrConversionInfo`
	- \valcom Handled by API design (ash)

3. The `sType` value of each struct in the `pNext` chain must be unique
	- \valcom Handled by API design

4. `flags` must be a valid combination of `VkSamplerCreateFlagBits` values
	- \valcom Handled by API design

5. `magFilter` must be a valid `VkFilter` value
	- \valcom Handled by API design

6. `minFilter` must be a valid `VkFilter` value
	- \valcom Handled by API design

7. `mipmapMode` must be a valid `VkSamplerMipmapMode` value
	- \valcom Handled by API design

8. `addressModeU` must be a valid `VkSamplerAddressMode` value
	- \valcom Handled by API design

9. `addressModeV` must be a valid `VkSamplerAddressMode` value
	- \valcom Handled by API design

10. `addressModeW` must be a valid `VkSamplerAddressMode` value
	- \valcom Handled by API design

\valcomboxend

\valboxend

Validations for `VkDescriptorBufferInfo`:

\valbox

\valcombox

1. `buffer` must be a valid `VkBuffer` handle
	- \valcom Handled by API design

\valcomboxend

\valboxend

Validations for `VkDescriptorImageInfo`:

\valbox

\valdonebox

1. Both of `imageView`, and `sampler` that are valid handles of non-ignored parameters must have been created, allocated, or retrieved from the same `VkDevice`
	- \valdone Returns error

\valdoneboxend

\valboxend

Validations for `VkWriteDescriptorSetInlineUniformBlockEXT`:

\valbox

\valcombox

1. `sType` must be `VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK_EXT`
	- \valcom Handled by API design (ash)

2. `pData` must be a valid pointer to an array of `dataSize` bytes
	- \valcom Handled by API design

\valcomboxend

\valdonebox

3. `dataSize` must be greater than `0`
	- \valdone Returns error

\valdoneboxend

\valboxend

Validations for `VkWriteDescriptorSet`:

\valbox

\valcombox

1. `sType` must be `VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET`
	- \valcom Handled by API design (ash)

2. Each `pNext` member of any structure (including this one) in the `pNext` chain must be either `NULL` or a pointer to a valid instance of `VkWriteDescriptorSetAccelerationStructureKHR` or `VkWriteDescriptorSetInlineUniformBlockEXT`
	- \valcom Handled by API design (ash)

3. The `sType` value of each struct in the `pNext` chain must be unique
	- \valcom Handled by API design

4. `descriptorType` must be a valid `VkDescriptorType` value
	- \valcom Handled by API design

\valcomboxend

\valdonebox

5. `descriptorCount` must be greater than `0`
	- \valdone Returns error

\valdoneboxend

6. Both of `dstSet`, and the elements of `pTexelBufferView` that are valid handles of non-ignored parameters must have been created, allocated, or retrieved from the same `VkDevice`

\valboxend

Validations for `VkCopyDescriptorSet`:

\valbox

\valcombox

1. `sType` must be `VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET`
	- \valcom Handled by API design (ash)

2. `pNext` must be `NULL`
	- \valcom Handled by API design (ash)

3. `srcSet` must be a valid `VkDescriptorSet` handle
	- \valcom Handled by API design

4. `dstSet` must be a valid `VkDescriptorSet` handle
	- \valcom Handled by API design

\valcomboxend

5. Both of `dstSet`, and `srcSet` must have been created, allocated, or retrieved from the same `VkDevice`

\valboxend

Validations for `vkUpdateDescriptorSets`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

2. If `descriptorWriteCount` is not `0`, `pDescriptorWrites` must be a valid pointer to an array of `descriptorWriteCount` valid `VkWriteDescriptorSet` structures
	- \valcom Handled by API design

3. If `descriptorCopyCount` is not `0`, `pDescriptorCopies` must be a valid pointer to an array of `descriptorCopyCount` valid `VkCopyDescriptorSet` structures
	- \valcom Handled by API design

\valcomboxend

\valboxend

### Render pass

Validations for `vkCreateRenderPass`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

2. `pCreateInfo` must be a valid pointer to a valid `VkRenderPassCreateInfo` structure
	- \valcom Handled by API design

3. If `pAllocator` is not `NULL`, `pAllocator` must be a valid pointer to a valid `VkAllocationCallbacks` structure
	- \valcom Handled by API design

4. `pRenderPass` must be a valid pointer to a `VkRenderPass` handle
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `VkRenderPassCreateInfo`:

\valbox

\valcombox

1. `sType` must be `VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO`
	- \valcom Handled by API design (ash)

2. Each `pNext` member of any structure (including this one) in the `pNext` chain must be either `NULL` or a pointer to a valid instance of `VkRenderPassFragmentDensityMapCreateInfoEXT`, `VkRenderPassInputAttachmentAspectCreateInfo`, or `VkRenderPassMultiviewCreateInfo`
	- \valcom Handled by API design (ash)

3. The `sType` value of each struct in the `pNext` chain must be unique
	- \valcom Handled by API design

4. `flags` must be a valid combination of `VkRenderPassCreateFlagBits` values
	- \valcom Handled by API design

5. If `attachmentCount` is not `0`, `pAttachments` must be a valid pointer to an array of `attachmentCount` valid `VkAttachmentDescription` structures
	- \valcom Handled by API design

6. `pSubpasses` must be a valid pointer to an array of `subpassCount` valid `VkSubpassDescription` structures
	- \valcom Handled by API design

7. If `dependencyCount` is not `0`, `pDependencies` must be a valid pointer to an array of `dependencyCount` valid `VkSubpassDependency` structures
	- \valcom Handled by API design

\valcomboxend

\valdonebox

8. `subpassCount` must be greater than `0`
	- \valdone Returns error

\valdoneboxend

\valboxend

Validations for `VkAttachmentDescription`:

\valbox

\valcombox

1. `flags` must be a valid combination of `VkAttachmentDescriptionFlagBits` values
	- \valcom Handled by API design

2. `format` must be a valid `VkFormat` value
	- \valcom Handled by API design (ash)

3. `samples` must be a valid `VkSampleCountFlagBits` value
	- \valcom Handled by API design (ash)

4. `loadOp` must be a valid `VkAttachmentLoadOp` value
	- \valcom Handled by API design (ash)

5. `storeOp` must be a valid `VkAttachmentStoreOp` value
	- \valcom Handled by API design (ash)

6. `stencilLoadOp` must be a valid `VkAttachmentLoadOp` value
	- \valcom Handled by API design (ash)

7. `stencilStoreOp` must be a valid `VkAttachmentStoreOp` value
	- \valcom Handled by API design (ash)

8. `initialLayout` must be a valid `VkImageLayout` value
	- \valcom Handled by API design (ash)

9. `finalLayout` must be a valid `VkImageLayout` value
	- \valcom Handled by API design

\valcomboxend

\valboxend

Validations for `VkSubpassDescription`:

\valbox

\valcombox

1. `flags` must be a valid combination of `VkSubpassDescriptionFlagBits` values
	- \valcom Handled by API design

2. `pipelineBindPoint` must be a valid `VkPipelineBindPoint` value
	- \valcom Handled by API design (ash)

3. If `inputAttachmentCount` is not `0`, `pInputAttachments` must be a valid pointer to an array of `inputAttachmentCount` valid `VkAttachmentReference` structures
	- \valcom Handled by API design

4. If `colorAttachmentCount` is not `0`, `pColorAttachments` must be a valid pointer to an array of `colorAttachmentCount` valid `VkAttachmentReference` structures
	- \valcom Handled by API design

\valcomboxend

\valdonebox

5. If `colorAttachmentCount` is not `0`, and `pResolveAttachments` is not `NULL`, `pResolveAttachments` must be a valid pointer to an array of `colorAttachmentCount` valid `VkAttachmentReference` structures
	- \valdone Returns error

\valdoneboxend

\valcombox

6. If `pDepthStencilAttachment` is not `NULL`, `pDepthStencilAttachment` must be a valid pointer to a valid `VkAttachmentReference` structure
	- \valcom Handled by API design

7. If `preserveAttachmentCount` is not `0`, `pPreserveAttachments` must be a valid pointer to an array of `preserveAttachmentCount` `uint32_t` values
	- \valcom Handled by API design

\valcomboxend

\valboxend

Validations for `VkSubpassDependency`:

\valbox

\valcombox

1. `srcStageMask` must be a valid combination of `VkPipelineStageFlagBits` values
	- \valcom Handled by API design (ash)

\valcomboxend

\valdonebox

2. `srcStageMask` must not be `0`
	- \valdone Returns error

\valdoneboxend

\valcombox

3. `dstStageMask` must be a valid combination of `VkPipelineStageFlagBits` values
	- \valcom Handled by API design (ash)

\valcomboxend

\valdonebox

4. `dstStageMask` must not be `0`
	- \valdone Returns error

\valdoneboxend

\valcombox

5. `srcAccessMask` must be a valid combination of `VkAccessFlagBits` values
	- \valcom Handled by API design (ash)

6. `dstAccessMask` must be a valid combination of `VkAccessFlagBits` values
	- \valcom Handled by API design (ash)

7. `dependencyFlags` must be a valid combination of `VkDependencyFlagBits` values
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

### Framebuffer

Validations for `vkCreateFramebuffer`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

2. `pCreateInfo` must be a valid pointer to a valid `VkFramebufferCreateInfo` structure
	- \valcom Handled by API design (ash)

3. If `pAllocator` is not `NULL`, `pAllocator` must be a valid pointer to a valid `VkAllocationCallbacks` structure
	- \valcom Handled by API design

4. `pFramebuffer` must be a valid pointer to a `VkFramebuffer` handle
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `VkFramebufferCreateInfo`:

\valbox

\valcombox

1. `sType` must be `VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO`
	- \valcom Handled by API design (ash)

2. `pNext` must be `NULL` or a pointer to a valid instance of `VkFramebufferAttachmentsCreateInfo`
	- \valcom Handled by API design (ash)

3. The `sType` value of each struct in the `pNext` chain must be unique
	- \valcom Handled by API design

4. `flags` must be a valid combination of `VkFramebufferCreateFlagBits` values
	- \valcom Handled by API design

5. `renderPass` must be a valid `VkRenderPass` handle
	- \valcom Handled by API design

\valcomboxend

6. Both of `renderPass`, and the elements of `pAttachments` that are valid handles of non-ignored parameters must have been created, allocated, or retrieved from the same `VkDevice`

\valboxend

### Shader

Validations for `vkCreateShaderModule`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

2. `pCreateInfo` must be a valid pointer to a valid `VkShaderModuleCreateInfo` structure
	- \valcom Handled by API design

3. If `pAllocator` is not `NULL`, `pAllocator` must be a valid pointer to a valid `VkAllocationCallbacks` structure
	- \valcom Handled by API design

4. `pShaderModule` must be a valid pointer to a `VkShaderModule` handle
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `VkShaderModuleCreateInfo`:

\valbox

\valcombox

1. `sType` must be `VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO`
	- \valcom Handled by API design (ash)

2. `pNext` must be `NULL` or a pointer to a valid instance of `VkShaderModuleValidationCacheCreateInfoEXT`
	- \valcom Handled by API design (ash)

3. The `sType` value of each struct in the `pNext` chain must be unique
	- \valcom Handled by API design

4. `flags` must be `0`
	- \valcom Handled by API design (ash)

5. `pCode` must be a valid pointer to an array of $\textrm{codeSize} \over 4$ `uint32_t` values
	- \valcom Handled by API design

\valcomboxend

\valboxend

Validations for `VkVertexInputBindingDescription`:

\valbox

\valcombox

1. `inputRate` must be a valid `VkVertexInputRate` value
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `VkVertexInputAttributeDescription`:

\valbox

\valcombox

1. `format` must be a valid `VkFormat` value
	- \valcom Handled by API design

\valcomboxend

\valboxend

### Pipeline

Validations for `vkCreatePipelineLayout`:

\valbox

\valcombox

1. `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

2. `pCreateInfo` must be a valid pointer to a valid `VkPipelineLayoutCreateInfo` structure
	- \valcom Handled by API design

3. If `pAllocator` is not `NULL`, `pAllocator` must be a valid pointer to a valid `VkAllocationCallbacks` structure
	- \valcom Handled by API design

4. `pPipelineLayout` must be a valid pointer to a `VkPipelineLayout` handle
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `VkPipelineLayoutCreateInfo`:

\valbox

\valcombox

1. `sType` must be `VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO`
	- \valcom Handled by API design (ash)

2. `pNext` must be `NULL`
	- \valcom Handled by API design (ash)

3. `flags` must be `0`
	- \valcom Handled by API design (ash)

4. If `setLayoutCount` is not `0`, `pSetLayouts` must be a valid pointer to an array of `setLayoutCount` valid `VkDescriptorSetLayout` handles
	- \valcom Handled by API design

5. If `pushConstantRangeCount` is not `0`, `pPushConstantRanges` must be a valid pointer to an array of `pushConstantRangeCount` valid `VkPushConstantRange` structures
	- \valcom Handled by API design

\valcomboxend

\valboxend

Validations for `VkPushConstantRange`:

\valbox

\valcombox

1. `stageFlags` must be a valid combination of `VkShaderStageFlagBits` values
	- \valcom Handled by API design (ash)

\valcomboxend

\valdonebox

2. `stageFlags` must not be `0`
	- \valdone Returns error

\valdoneboxend

\valboxend

