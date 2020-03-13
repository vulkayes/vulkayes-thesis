## Implicit validations

### Instance

Validations for `vkCreateInstance`:

\valbox

* `pCreateInfo` must be a valid pointer to a valid `VkInstanceCreateInfo` structure

* If `pAllocator` is not `NULL`, `pAllocator` must be a valid pointer to a valid `VkAllocationCallbacks` structure

* `pInstance` must be a valid pointer to a `VkInstance` handle

\valboxend

Validations for `VkInstanceCreateInfo`:

\valbox

* `sType` must be `VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO`

* Each `pNext` member of any structure (including this one) in the `pNext` chain must be either `NULL` or a pointer to a valid instance of `VkDebugReportCallbackCreateInfoEXT`, `VkDebugUtilsMessengerCreateInfoEXT`, `VkValidationFeaturesEXT`, or `VkValidationFlagsEXT`

* The `sType` value of each struct in the `pNext` chain must be unique

* `flags` must be `0`

* If `pApplicationInfo` is not `NULL`, `pApplicationInfo` must be a valid pointer to a valid `VkApplicationInfo` structure

* If `enabledLayerCount` is not `0`, `ppEnabledLayerNames` must be a valid pointer to an array of `enabledLayerCount` null-terminated UTF-8 strings

* If `enabledExtensionCount` is not `0`, `ppEnabledExtensionNames` must be a valid pointer to an array of `enabledExtensionCount` null-terminated UTF-8 strings

\valboxend

### Device

Validations for `vkCreateDevice`:

\valbox

* `physicalDevice` must be a valid `VkPhysicalDevice` handle

* `pCreateInfo` must be a valid pointer to a valid `VkDeviceCreateInfo` structure

* If `pAllocator` is not `NULL`, `pAllocator` must be a valid pointer to a valid `VkAllocationCallbacks` structure

* `pDevice` must be a valid pointer to a `VkDevice` handle

\valboxend

Validations for `VkDeviceCreateInfo`:

\valbox

* `sType` must be `VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO`

* Each `pNext` member of any structure (including this one) in the `pNext` chain must be either `NULL` or a pointer to a valid instance of `VkDeviceGroupDeviceCreateInfo`, `VkDeviceMemoryOverallocationCreateInfoAMD`, `VkPhysicalDevice16BitStorageFeatures`, `VkPhysicalDevice8BitStorageFeatures`, `VkPhysicalDeviceASTCDecodeFeaturesEXT`, `VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT`, `VkPhysicalDeviceBufferDeviceAddressFeatures`, `VkPhysicalDeviceBufferDeviceAddressFeaturesEXT`, `VkPhysicalDeviceCoherentMemoryFeaturesAMD`, `VkPhysicalDeviceComputeShaderDerivativesFeaturesNV`, `VkPhysicalDeviceConditionalRenderingFeaturesEXT`, `VkPhysicalDeviceCooperativeMatrixFeaturesNV`, `VkPhysicalDeviceCornerSampledImageFeaturesNV`, `VkPhysicalDeviceCoverageReductionModeFeaturesNV`, `VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV`, `VkPhysicalDeviceDepthClipEnableFeaturesEXT`, `VkPhysicalDeviceDescriptorIndexingFeatures`, `VkPhysicalDeviceExclusiveScissorFeaturesNV`, `VkPhysicalDeviceFeatures2`, `VkPhysicalDeviceFragmentDensityMapFeaturesEXT`, `VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV`, `VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT`, `VkPhysicalDeviceHostQueryResetFeatures`, `VkPhysicalDeviceImagelessFramebufferFeatures`, `VkPhysicalDeviceIndexTypeUint8FeaturesEXT`, `VkPhysicalDeviceInlineUniformBlockFeaturesEXT`, `VkPhysicalDeviceLineRasterizationFeaturesEXT`, `VkPhysicalDeviceMemoryPriorityFeaturesEXT`, `VkPhysicalDeviceMeshShaderFeaturesNV`, `VkPhysicalDeviceMultiviewFeatures`, `VkPhysicalDevicePerformanceQueryFeaturesKHR`, `VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR`, `VkPhysicalDeviceProtectedMemoryFeatures`, `VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV`, `VkPhysicalDeviceSamplerYcbcrConversionFeatures`, `VkPhysicalDeviceScalarBlockLayoutFeatures`, `VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures`, `VkPhysicalDeviceShaderAtomicInt64Features`, `VkPhysicalDeviceShaderClockFeaturesKHR`, `VkPhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT`, `VkPhysicalDeviceShaderDrawParametersFeatures`, `VkPhysicalDeviceShaderFloat16Int8Features`, `VkPhysicalDeviceShaderImageFootprintFeaturesNV`, `VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL`, `VkPhysicalDeviceShaderSMBuiltinsFeaturesNV`, `VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures`, `VkPhysicalDeviceShadingRateImageFeaturesNV`, `VkPhysicalDeviceSubgroupSizeControlFeaturesEXT`, `VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT`, `VkPhysicalDeviceTextureCompressionASTCHDRFeaturesEXT`, `VkPhysicalDeviceTimelineSemaphoreFeatures`, `VkPhysicalDeviceTransformFeedbackFeaturesEXT`, `VkPhysicalDeviceUniformBufferStandardLayoutFeatures`, `VkPhysicalDeviceVariablePointersFeatures`, `VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT`, `VkPhysicalDeviceVulkan11Features`, `VkPhysicalDeviceVulkan12Features`, `VkPhysicalDeviceVulkanMemoryModelFeatures`, or `VkPhysicalDeviceYcbcrImageArraysFeaturesEXT`

* The `sType` value of each struct in the `pNext` chain must be unique

* `flags` must be `0`

* `pQueueCreateInfos` must be a valid pointer to an array of `queueCreateInfoCount` valid `VkDeviceQueueCreateInfo` structures

* If `enabledLayerCount` is not `0`, `ppEnabledLayerNames` must be a valid pointer to an array of `enabledLayerCount` null-terminated UTF-8 strings

* If `enabledExtensionCount` is not `0`, `ppEnabledExtensionNames` must be a valid pointer to an array of `enabledExtensionCount` null-terminated UTF-8 strings

* If `pEnabledFeatures` is not `NULL`, `pEnabledFeatures` must be a valid pointer to a valid `VkPhysicalDeviceFeatures` structure

* `queueCreateInfoCount` must be greater than `0`

\valboxend

Validations for `VkDeviceQueueCreateInfo`:

\valbox

* `sType` must be `VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO`

* `pNext` must be `NULL` or a pointer to a valid instance of `VkDeviceQueueGlobalPriorityCreateInfoEXT`

* The `sType` value of each struct in the `pNext` chain must be unique

* `flags` must be a valid combination of `VkDeviceQueueCreateFlagBits` values

* `pQueuePriorities` must be a valid pointer to an array of `queueCount` `float` values

* `queueCount` must be greater than `0`

\valboxend

Validations for `vkCreateSwapchainKHR`:

\valbox

* `device` must be a valid `VkDevice` handle

* `pCreateInfo` must be a valid pointer to a valid `VkSwapchainCreateInfoKHR` structure

* If `pAllocator` is not `NULL`, `pAllocator` must be a valid pointer to a valid `VkAllocationCallbacks` structure

* `pSwapchain` must be a valid pointer to a `VkSwapchainKHR` handle

\valboxend

Validations for `VkSwapchainCreateInfoKHR`:

\valbox

* `sType` must be `VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR`

* Each `pNext` member of any structure (including this one) in the `pNext` chain must be either `NULL` or a pointer to a valid instance of `VkDeviceGroupSwapchainCreateInfoKHR`, `VkImageFormatListCreateInfo`, `VkSurfaceFullScreenExclusiveInfoEXT`, `VkSurfaceFullScreenExclusiveWin32InfoEXT`, `VkSwapchainCounterCreateInfoEXT`, or `VkSwapchainDisplayNativeHdrCreateInfoAMD`

* The `sType` value of each struct in the `pNext` chain must be unique

* `flags` must be a valid combination of `VkSwapchainCreateFlagBitsKHR` values

* `surface` must be a valid `VkSurfaceKHR` handle

* `imageFormat` must be a valid `VkFormat` value

* `imageColorSpace` must be a valid `VkColorSpaceKHR` value

* `imageUsage` must be a valid combination of `VkImageUsageFlagBits` values

* `imageUsage` must not be `0`

* `imageSharingMode` must be a valid `VkSharingMode` value

* `preTransform` must be a valid `VkSurfaceTransformFlagBitsKHR` value

* `compositeAlpha` must be a valid `VkCompositeAlphaFlagBitsKHR` value

* `presentMode` must be a valid `VkPresentModeKHR` value

* If `oldSwapchain` is not `VK_NULL_HANDLE`, `oldSwapchain` must be a valid `VkSwapchainKHR` handle

* If `oldSwapchain` is a valid handle, it must have been created, allocated, or retrieved from `surface`

* Both of `oldSwapchain`, and `surface` that are valid handles of non-ignored parameters must have been created, allocated, or retrieved from the same `VkInstance`

\valboxend

Validations for `vkCreateCommandPool`:

\valbox

* `device` must be a valid `VkDevice` handle

* `pCreateInfo` must be a valid pointer to a valid `VkCommandPoolCreateInfo` structure

* If `pAllocator` is not `NULL`, `pAllocator` must be a valid pointer to a valid `VkAllocationCallbacks` structure

* `pCommandPool` must be a valid pointer to a `VkCommandPool` handle

\valboxend

Validations for `VkCommandPoolCreateInfo`:

\valbox

* `sType` must be `VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO`

* `pNext` must be `NULL`

* `flags` must be a valid combination of `VkCommandPoolCreateFlagBits` values

\valboxend

Validations for `VkCommandBufferAllocateInfo`:

\valbox

* `device` must be a valid `VkDevice` handle

* `pAllocateInfo` must be a valid pointer to a valid `VkCommandBufferAllocateInfo` structure

* `pCommandBuffers` must be a valid pointer to an array of `pAllocateInfo`::commandBufferCount `VkCommandBuffer` handles

* The value referenced by `pAllocateInfo`::`commandBufferCount` must be greater than `0`

\valboxend

