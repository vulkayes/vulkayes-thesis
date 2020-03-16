### Instance

Validations for `vkCreateInstance`:

\valbox

\valcombox

* `pCreateInfo` must be a valid pointer to a valid `VkInstanceCreateInfo` structure
	- \valcom Handled by API design (ash)

* If `pAllocator` is not `NULL`, `pAllocator` must be a valid pointer to a valid `VkAllocationCallbacks` structure
	- \valcom Handled by API design (ash)

* `pInstance` must be a valid pointer to a `VkInstance` handle
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `VkInstanceCreateInfo`:

\valbox

\valcombox

* `sType` must be `VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO`
	- \valcom Handled by API design (ash)

* Each `pNext` member of any structure (including this one) in the `pNext` chain must be either `NULL` or a pointer to a valid instance of `VkDebugReportCallbackCreateInfoEXT`, `VkDebugUtilsMessengerCreateInfoEXT`, `VkValidationFeaturesEXT`, or `VkValidationFlagsEXT`
	- \valcom Handled by API design (ash)

* The `sType` value of each struct in the `pNext` chain must be unique
	- \valcom Handled by API design

* `flags` must be `0`
	- \valcom Handled by API design (ash)

* If `pApplicationInfo` is not `NULL`, `pApplicationInfo` must be a valid pointer to a valid `VkApplicationInfo` structure
	- \valcom Handled by API design (ash)

\valcomboxend

\valdonebox

* If `enabledLayerCount` is not `0`, `ppEnabledLayerNames` must be a valid pointer to an array of `enabledLayerCount` null-terminated UTF-8 strings
	- \valdone Returns error

* If `enabledExtensionCount` is not `0`, `ppEnabledExtensionNames` must be a valid pointer to an array of `enabledExtensionCount` null-terminated UTF-8 strings
	- \valdone Returns error

\valdoneboxend

\valboxend

### Device

Validations for `vkCreateDevice`:

\valbox

\valcombox

* `physicalDevice` must be a valid `VkPhysicalDevice` handle
	- \valcom Handled by API design (ash)

* `pCreateInfo` must be a valid pointer to a valid `VkDeviceCreateInfo` structure
	- \valcom Handled by API design (ash)

* If `pAllocator` is not `NULL`, `pAllocator` must be a valid pointer to a valid `VkAllocationCallbacks` structure
	- \valcom Handled by API design (ash)

* `pDevice` must be a valid pointer to a `VkDevice` handle
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `VkDeviceCreateInfo`:

\valbox

\valcombox

* `sType` must be `VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO`
	- \valcom Handled by API design (ash)

* Each `pNext` member of any structure (including this one) in the `pNext` chain must be either `NULL` or a pointer to a valid instance of `VkDeviceGroupDeviceCreateInfo`, `VkDeviceMemoryOverallocationCreateInfoAMD`, `VkPhysicalDevice16BitStorageFeatures`, `VkPhysicalDevice8BitStorageFeatures`, `VkPhysicalDeviceASTCDecodeFeaturesEXT`, `VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT`, `VkPhysicalDeviceBufferDeviceAddressFeatures`, `VkPhysicalDeviceBufferDeviceAddressFeaturesEXT`, `VkPhysicalDeviceCoherentMemoryFeaturesAMD`, `VkPhysicalDeviceComputeShaderDerivativesFeaturesNV`, `VkPhysicalDeviceConditionalRenderingFeaturesEXT`, `VkPhysicalDeviceCooperativeMatrixFeaturesNV`, `VkPhysicalDeviceCornerSampledImageFeaturesNV`, `VkPhysicalDeviceCoverageReductionModeFeaturesNV`, `VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV`, `VkPhysicalDeviceDepthClipEnableFeaturesEXT`, `VkPhysicalDeviceDescriptorIndexingFeatures`, `VkPhysicalDeviceExclusiveScissorFeaturesNV`, `VkPhysicalDeviceFeatures2`, `VkPhysicalDeviceFragmentDensityMapFeaturesEXT`, `VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV`, `VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT`, `VkPhysicalDeviceHostQueryResetFeatures`, `VkPhysicalDeviceImagelessFramebufferFeatures`, `VkPhysicalDeviceIndexTypeUint8FeaturesEXT`, `VkPhysicalDeviceInlineUniformBlockFeaturesEXT`, `VkPhysicalDeviceLineRasterizationFeaturesEXT`, `VkPhysicalDeviceMemoryPriorityFeaturesEXT`, `VkPhysicalDeviceMeshShaderFeaturesNV`, `VkPhysicalDeviceMultiviewFeatures`, `VkPhysicalDevicePerformanceQueryFeaturesKHR`, `VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR`, `VkPhysicalDeviceProtectedMemoryFeatures`, `VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV`, `VkPhysicalDeviceSamplerYcbcrConversionFeatures`, `VkPhysicalDeviceScalarBlockLayoutFeatures`, `VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures`, `VkPhysicalDeviceShaderAtomicInt64Features`, `VkPhysicalDeviceShaderClockFeaturesKHR`, `VkPhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT`, `VkPhysicalDeviceShaderDrawParametersFeatures`, `VkPhysicalDeviceShaderFloat16Int8Features`, `VkPhysicalDeviceShaderImageFootprintFeaturesNV`, `VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL`, `VkPhysicalDeviceShaderSMBuiltinsFeaturesNV`, `VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures`, `VkPhysicalDeviceShadingRateImageFeaturesNV`, `VkPhysicalDeviceSubgroupSizeControlFeaturesEXT`, `VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT`, `VkPhysicalDeviceTextureCompressionASTCHDRFeaturesEXT`, `VkPhysicalDeviceTimelineSemaphoreFeatures`, `VkPhysicalDeviceTransformFeedbackFeaturesEXT`, `VkPhysicalDeviceUniformBufferStandardLayoutFeatures`, `VkPhysicalDeviceVariablePointersFeatures`, `VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT`, `VkPhysicalDeviceVulkan11Features`, `VkPhysicalDeviceVulkan12Features`, `VkPhysicalDeviceVulkanMemoryModelFeatures`, or `VkPhysicalDeviceYcbcrImageArraysFeaturesEXT`
	- \valcom Handled by API design (ash)

* The `sType` value of each struct in the `pNext` chain must be unique
	- \valcom Handled by API design

* `flags` must be `0`
	- \valcom Handled by API design (ash)

* `pQueueCreateInfos` must be a valid pointer to an array of `queueCreateInfoCount` valid `VkDeviceQueueCreateInfo` structures
	- \valcom Handled by API design (ash)

\valcomboxend

\valdonebox

* If `enabledLayerCount` is not `0`, `ppEnabledLayerNames` must be a valid pointer to an array of `enabledLayerCount` null-terminated UTF-8 strings
	- \valdone Returns error

* If `enabledExtensionCount` is not `0`, `ppEnabledExtensionNames` must be a valid pointer to an array of `enabledExtensionCount` null-terminated UTF-8 strings
	- \valdone Returns error

\valdoneboxend

\valcombox

* If `pEnabledFeatures` is not `NULL`, `pEnabledFeatures` must be a valid pointer to a valid `VkPhysicalDeviceFeatures` structure
	- \valcom Handled by API design (ash)

\valcomboxend

\valdonebox

* `queueCreateInfoCount` must be greater than `0`
	- \valdone Returns error

\valdoneboxend

\valboxend

### Queue

Validations for `VkDeviceQueueCreateInfo`:

\valbox

\valcombox

* `sType` must be `VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO`
	- \valcom Handled by API design (ash)

* `pNext` must be `NULL` or a pointer to a valid instance of `VkDeviceQueueGlobalPriorityCreateInfoEXT`
	- \valcom Handled by API design (ash)

* The `sType` value of each struct in the `pNext` chain must be unique
	- \valcom Handled by API design

* `flags` must be a valid combination of `VkDeviceQueueCreateFlagBits` values
	- \valcom Handled by API design (ash)

* `pQueuePriorities` must be a valid pointer to an array of `queueCount` `float` values
	- \valcom Handled by API design (ash)

\valcomboxend

\valdonebox

* `queueCount` must be greater than `0`
	- \valdone Returns error

\valdoneboxend

\valboxend

Validations for `vkGetDeviceQueue`:

\valbox

\valcombox

* `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

* `pQueue` must be a valid pointer to a `VkQueue` handle
	- \valcom Handled by API design

\valcomboxend

\valboxend

Validations for `vkGetDeviceQueue2`:

\valbox

\valcombox

* `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

* `pQueueInfo` must be a valid pointer to a valid `VkDeviceQueueInfo2` structure
	- \valcom Handled by API design

* `pQueue` must be a valid pointer to a `VkQueue` handle
	- \valcom Handled by API design

\valcomboxend

\valboxend

Validations for `VkDeviceQueueInfo2`:

\valbox

\valcombox

* `sType` must be `VK_STRUCTURE_TYPE_DEVICE_QUEUE_INFO_2`
	- \valcom Handled by API design (ash)

* `pNext` must be `NULL`
	- \valcom Handled by API design (ash)

* `flags` must be a valid combination of `VkDeviceQueueCreateFlagBits` values
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `vkQueueSubmit`:

\valbox

\valcombox

* `queue` must be a valid `VkQueue` handle
	- \valcom Handled by API design

* If `submitCount` is not `0`, `pSubmits` must be a valid pointer to an array of `submitCount` valid `VkSubmitInfo` structures
	- \valcom Handled by API design

* If `fence` is not `VK_NULL_HANDLE`, `fence` must be a valid `VkFence` handle
	- \valcom Handled by API design

\valcomboxend

\valdonebox

* Both of `fence`, and `queue` that are valid handles of non-ignored parameters must have been created, allocated, or retrieved from the same `VkDevice`
	- \valdone Returns error

\valdoneboxend

\valboxend

Validations for `VkSubmitInfo`:

\valbox

\valcombox

* `sType` must be `VK_STRUCTURE_TYPE_SUBMIT_INFO`
	- \valcom Handled by API design (ash)

* Each `pNext` member of any structure (including this one) in the `pNext` chain must be either `NULL` or a pointer to a valid instance of `VkD3D12FenceSubmitInfoKHR`, `VkDeviceGroupSubmitInfo`, `VkPerformanceQuerySubmitInfoKHR`, `VkProtectedSubmitInfo`, `VkTimelineSemaphoreSubmitInfo`, `VkWin32KeyedMutexAcquireReleaseInfoKHR`, or `VkWin32KeyedMutexAcquireReleaseInfoNV`
	- \valcom Handled by API design (ash)

* The `sType` value of each struct in the `pNext` chain must be unique
	- \valcom Handled by API design

* If `waitSemaphoreCount` is not `0`, `pWaitSemaphores` must be a valid pointer to an array of `waitSemaphoreCount` valid `VkSemaphore` handles
	- \valcom Handled by API design (ash)

* If `waitSemaphoreCount` is not `0`, `pWaitDstStageMask` must be a valid pointer to an array of `waitSemaphoreCount` valid combinations of `VkPipelineStageFlagBits` values
	- \valcom Handled by API design (ash)

* Each element of `pWaitDstStageMask` must not be `0`
	- \valcom Handled by API design

* If `commandBufferCount` is not `0`, `pCommandBuffers` must be a valid pointer to an array of `commandBufferCount` valid `VkCommandBuffer` handles
	- \valcom Handled by API design (ash)

* If `signalSemaphoreCount` is not `0`, `pSignalSemaphores` must be a valid pointer to an array of `signalSemaphoreCount` valid `VkSemaphore` handles
	- \valcom Handled by API design (ash)

\valcomboxend

\valdonebox

* Each of the elements of `pCommandBuffers`, the elements of `pSignalSemaphores`, and the elements of `pWaitSemaphores` that are valid handles of non-ignored parameters must have been created, allocated, or retrieved from the same `VkDevice`
	- \valdone Returns error

\valdoneboxend

\valboxend

### Swapchain

Validations for `vkCreateSwapchainKHR`:

\valbox

\valcombox

* `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design (ash)

* `pCreateInfo` must be a valid pointer to a valid `VkSwapchainCreateInfoKHR` structure
	- \valcom Handled by API design (ash)

* If `pAllocator` is not `NULL`, `pAllocator` must be a valid pointer to a valid `VkAllocationCallbacks` structure
	- \valcom Handled by API design (ash)

* `pSwapchain` must be a valid pointer to a `VkSwapchainKHR` handle
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `VkSwapchainCreateInfoKHR`:

\valbox

\valcombox

* `sType` must be `VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR`
	- \valcom Handled by API design (ash)

* Each `pNext` member of any structure (including this one) in the `pNext` chain must be either `NULL` or a pointer to a valid instance of `VkDeviceGroupSwapchainCreateInfoKHR`, `VkImageFormatListCreateInfo`, `VkSurfaceFullScreenExclusiveInfoEXT`, `VkSurfaceFullScreenExclusiveWin32InfoEXT`, `VkSwapchainCounterCreateInfoEXT`, or `VkSwapchainDisplayNativeHdrCreateInfoAMD`
	- \valcom Handled by API design (ash)

* The `sType` value of each struct in the `pNext` chain must be unique
	- \valcom Handled by API design

* `flags` must be a valid combination of `VkSwapchainCreateFlagBitsKHR` values
	- \valcom Handled by API design (ash)

* `surface` must be a valid `VkSurfaceKHR` handle
	- \valcom Handled by API design (ash)

* `imageFormat` must be a valid `VkFormat` value
	- \valcom Handled by API design (ash)

* `imageColorSpace` must be a valid `VkColorSpaceKHR` value
	- \valcom Handled by API design (ash)

* `imageUsage` must be a valid combination of `VkImageUsageFlagBits` values
	- \valcom Handled by API design (ash)

\valcomboxend

\valdonebox

* `imageUsage` must not be `0`
	- \valdone Returns error

\valdoneboxend

\valcombox

* `imageSharingMode` must be a valid `VkSharingMode` value
	- \valcom Handled by API design (ash)

* `preTransform` must be a valid `VkSurfaceTransformFlagBitsKHR` value
	- \valcom Handled by API design (ash)

* `compositeAlpha` must be a valid `VkCompositeAlphaFlagBitsKHR` value
	- \valcom Handled by API design (ash)

* `presentMode` must be a valid `VkPresentModeKHR` value
	- \valcom Handled by API design (ash)

* If `oldSwapchain` is not `VK_NULL_HANDLE`, `oldSwapchain` must be a valid `VkSwapchainKHR` handle
	- \valcom Handled by API design (ash)

* If `oldSwapchain` is a valid handle, it must have been created, allocated, or retrieved from `surface`
	- \valcom Handled by API design

* Both of `oldSwapchain`, and `surface` that are valid handles of non-ignored parameters must have been created, allocated, or retrieved from the same `VkInstance`
	- \valcom Handled by API design

\valcomboxend

\valboxend

Validations for `vkGetSwapchainImagesKHR`:

\valbox

\valcombox

* `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

* `swapchain` must be a valid `VkSwapchainKHR` handle
	- \valcom Handled by API design

* `pSwapchainImageCount` must be a valid pointer to a `uint32_t` value
	- \valcom Handled by API design (ash)

* If the value referenced by `pSwapchainImageCount` is not `0`, and `pSwapchainImages` is not `NULL`, `pSwapchainImages` must be a valid pointer to an array of `pSwapchainImageCount` `VkImage` handles
	- \valcom Handled by API design (ash)

* Both of `device`, and `swapchain` must have been created, allocated, or retrieved from the same `VkInstance`
	- \valcom Handled by API design

\valcomboxend

\valboxend

Validations for `vkQueuePresentKHR`:

\valbox

\valcombox

* `queue` must be a valid `VkQueue` handle
	- \valcom Handled by API design (ash)

* `pPresentInfo` must be a valid pointer to a valid `VkPresentInfoKHR` structure
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `VkPresentInfoKHR`:

\valbox

\valcombox

* `sType` must be `VK_STRUCTURE_TYPE_PRESENT_INFO_KHR`
	- \valcom Handled by API design (ash)

* Each `pNext` member of any structure (including this one) in the `pNext` chain must be either `NULL` or a pointer to a valid instance of `VkDeviceGroupPresentInfoKHR`, `VkDisplayPresentInfoKHR`, `VkPresentFrameTokenGGP`, `VkPresentRegionsKHR`, or `VkPresentTimesInfoGOOGLE`
	- \valcom Handled by API design (ash)

* The `sType` value of each struct in the `pNext` chain must be unique
	- \valcom Handled by API design

* If `waitSemaphoreCount` is not `0`, `pWaitSemaphores` must be a valid pointer to an array of `waitSemaphoreCount` valid `VkSemaphore` handles
	- \valcom Handled by API design (ash)

* `pSwapchains` must be a valid pointer to an array of `swapchainCount` valid `VkSwapchainKHR` handles
	- \valcom Handled by API design (ash)

* `pImageIndices` must be a valid pointer to an array of `swapchainCount` `uint32_t` values
	- \valcom Handled by API design (ash)

* If `pResults` is not `NULL`, `pResults` must be a valid pointer to an array of `swapchainCount` `VkResult` values
	- \valcom Handled by API design (ash)

\valcomboxend

\valdonebox

* `swapchainCount` must be greater than `0`
	- \valdone Returns error

* Both of the elements of `pSwapchains`, and the elements of `pWaitSemaphores` that are valid handles of non-ignored parameters must have been created, allocated, or retrieved from the same `VkInstance`
	- \valdone Returns error

\valdoneboxend

\valboxend

Validations for `vkAcquireNextImageKHR`:

\valbox

\valcombox

* `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

* `swapchain` must be a valid `VkSwapchainKHR` handle
	- \valcom Handled by API design

* If `semaphore` is not `VK_NULL_HANDLE`, `semaphore` must be a valid `VkSemaphore` handle
	- \valcom Handled by API design

* If `fence` is not `VK_NULL_HANDLE`, `fence` must be a valid `VkFence` handle
	- \valcom Handled by API design

* `pImageIndex` must be a valid pointer to a `uint32_t` value
	- \valcom Handled by API design (ash)

\valcomboxend

\valdonebox

* If `semaphore` is a valid handle, it must have been created, allocated, or retrieved from `device`
	- \valdone Returns error

* If `fence` is a valid handle, it must have been created, allocated, or retrieved from `device`
	- \valdone Returns error

\valdoneboxend

\valcombox

* Both of `device`, and `swapchain` that are valid handles of non-ignored parameters must have been created, allocated, or retrieved from the same `VkInstance`
	- \valcom Handled by API design

\valcomboxend

\valboxend

### Command Buffer

Validations for `vkCreateCommandPool`:

\valbox

\valcombox

* `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

* `pCreateInfo` must be a valid pointer to a valid `VkCommandPoolCreateInfo` structure
	- \valcom Handled by API design (ash)

* If `pAllocator` is not `NULL`, `pAllocator` must be a valid pointer to a valid `VkAllocationCallbacks` structure
	- \valcom Handled by API design

* `pCommandPool` must be a valid pointer to a `VkCommandPool` handle
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `VkCommandPoolCreateInfo`:

\valbox

\valcombox

* `sType` must be `VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO`
	- \valcom Handled by API design (ash)

* `pNext` must be `NULL`
	- \valcom Handled by API design (ash)

* `flags` must be a valid combination of `VkCommandPoolCreateFlagBits` values
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `vkTrimCommandPool`:

\valbox

\valcombox

* `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

* `commandPool` must be a valid `VkCommandPool` handle
	- \valcom Handled by API design

* `flags` must be `0`
	- \valcom Handled by API design

* `commandPool` must have been created, allocated, or retrieved from `device`
	- \valcom Handled by API design

\valcomboxend

\valboxend

Validations for `vkResetCommandPool`:

\valbox

\valcombox

* `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

* `commandPool` must be a valid `VkCommandPool` handle
	- \valcom Handled by API design

* `flags` must be a valid combination of `VkCommandPoolResetFlagBits` values
	- \valcom Handled by API design

* `commandPool` must have been created, allocated, or retrieved from `device`
	- \valcom Handled by API design

\valcomboxend

\valboxend

Validations for `VkCommandBufferAllocateInfo`:

\valbox

\valcombox

* `sType` must be `VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO`
	- \valcom Handled by API design (ash)

* `pNext` must be `NULL`
	- \valcom Handled by API design (ash)

* `commandPool` must be a valid `VkCommandPool` handle
	- \valcom Handled by API design (ash)

* `level` must be a valid `VkCommandBufferLevel` value
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

### Fence

Validations for `vkCreateFence`:

\valbox

\valcombox

* `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design (ash)

* `pCreateInfo` must be a valid pointer to a valid `VkFenceCreateInfo` structure
	- \valcom Handled by API design (ash)

* If `pAllocator` is not `NULL`, `pAllocator` must be a valid pointer to a valid `VkAllocationCallbacks` structure
	- \valcom Handled by API design (ash)

* `pFence` must be a valid pointer to a `VkFence` handle
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `VkFenceCreateInfo`:

\valbox

\valcombox

* `sType` must be `VK_STRUCTURE_TYPE_FENCE_CREATE_INFO`
	- \valcom Handled by API design (ash)

* Each `pNext` member of any structure (including this one) in the `pNext` chain must be either `NULL` or a pointer to a valid instance of `VkExportFenceCreateInfo` or `VkExportFenceWin32HandleInfoKHR`
	- \valcom Handled by API design (ash)

* The `sType` value of each struct in the `pNext` chain must be unique
	- \valcom Handled by API design

* `flags` must be a valid combination of `VkFenceCreateFlagBits` values
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `vkGetFenceStatus`:

\valbox

\valcombox

* `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

* `fence` must be a valid `VkFence` handle
	- \valcom Handled by API design

* `fence` must have been created, allocated, or retrieved from `device`
	- \valcom Handled by API design

\valcomboxend

\valboxend

Validations for `vkResetFences`:

\valbox

\valcombox

* `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

* `pFences` must be a valid pointer to an array of `fenceCount` valid `VkFence` handles
	- \valcom Handled by API design

* `fenceCount` must be greater than `0`
	- \valcom Handled by API design

* Each element of `pFences` must have been created, allocated, or retrieved from `device`
	- \valcom Handled by API design

\valcomboxend

\valboxend

Validations for `vkWaitForFences`:

\valbox

\valcombox

* `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

* `pFences` must be a valid pointer to an array of `fenceCount` valid `VkFence` handles
	- \valcom Handled by API design

* `fenceCount` must be greater than `0`
	- \valcom Handled by API design

* Each element of `pFences` must have been created, allocated, or retrieved from `device`
	- \valcom Handled by API design

\valcomboxend

\valboxend

### Sempahore

Validations for `vkCreateSemaphore`:

\valbox

\valcombox

* `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design (ash)

* `pCreateInfo` must be a valid pointer to a valid `VkSemaphoreCreateInfo` structure
	- \valcom Handled by API design (ash)

* If `pAllocator` is not `NULL`, `pAllocator` must be a valid pointer to a valid `VkAllocationCallbacks` structure
	- \valcom Handled by API design (ash)

* `pSemaphore` must be a valid pointer to a `VkSemaphore` handle
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `VkSemaphoreCreateInfo`:

\valbox

\valcombox

* `sType` must be `VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO`
	- \valcom Handled by API design (ash)

* Each `pNext` member of any structure (including this one) in the `pNext` chain must be either `NULL` or a pointer to a valid instance of `VkExportSemaphoreCreateInfo`, `VkExportSemaphoreWin32HandleInfoKHR`, or `VkSemaphoreTypeCreateInfo`
	- \valcom Handled by API design (ash)

* The `sType` value of each struct in the `pNext` chain must be unique
	- \valcom Handled by API design

* `flags` must be `0`
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `VkSemaphoreTypeCreateInfo`:

\valbox

\valcombox

* `sType` must be `VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO`
	- \valcom Handled by API design (ash)

* `semaphoreType` must be a valid `VkSemaphoreType` value
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

### Image

Validations for `vkCreateImage`:

\valbox

\valcombox

* `device` must be a valid `VkDevice` handle
	- \valcom Handled by API design

* `pCreateInfo` must be a valid pointer to a valid `VkImageCreateInfo` structure
	- \valcom Handled by API design (ash)

* If `pAllocator` is not `NULL`, `pAllocator` must be a valid pointer to a valid `VkAllocationCallbacks` structure
	- \valcom Handled by API design

* `pImage` must be a valid pointer to a `VkImage` handle
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

Validations for `VkImageCreateInfo`:

\valbox

\valcombox

* `sType` must be `VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO`
	- \valcom Handled by API design (ash)

* Each `pNext` member of any structure (including this one) in the `pNext` chain must be either `NULL` or a pointer to a valid instance of `VkDedicatedAllocationImageCreateInfoNV`, `VkExternalFormatANDROID`, `VkExternalMemoryImageCreateInfo`, `VkExternalMemoryImageCreateInfoNV`, `VkImageDrmFormatModifierExplicitCreateInfoEXT`, `VkImageDrmFormatModifierListCreateInfoEXT`, `VkImageFormatListCreateInfo`, `VkImageStencilUsageCreateInfo`, or `VkImageSwapchainCreateInfoKHR`
	- \valcom Handled by API design (ash)

* The `sType` value of each struct in the `pNext` chain must be unique
	- \valcom Handled by API design

* `flags` must be a valid combination of `VkImageCreateFlagBits` values
	- \valcom Handled by API design

* `imageType` must be a valid `VkImageType` value
	- \valcom Handled by API design (ash)

* `format` must be a valid `VkFormat` value
	- \valcom Handled by API design (ash)

* `samples` must be a valid `VkSampleCountFlagBits` value
	- \valcom Handled by API design (ash)

* `tiling` must be a valid `VkImageTiling` value
	- \valcom Handled by API design (ash)

* `usage` must be a valid combination of `VkImageUsageFlagBits` values
	- \valcom Handled by API design

\valcomboxend

* `usage` must not be `0`

\valcombox

* `sharingMode` must be a valid `VkSharingMode` value
	- \valcom Handled by API design (ash)

* `initialLayout` must be a valid `VkImageLayout` value
	- \valcom Handled by API design (ash)

\valcomboxend

\valboxend

