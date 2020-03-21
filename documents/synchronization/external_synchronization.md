\valbox

\valcombox
* The `instance` parameter in `vkDestroyInstance`
	- \valcom Consequence of shared pointer usage

* The `device` parameter in `vkDestroyDevice`
	- \valcom Consequence of shared pointer usage

* The `queue` parameter in `vkQueueSubmit`
	- \valcom Synchronized internally

* The `fence` parameter in `vkQueueSubmit`
	- \valcom Synchronized internally

* The `queue` parameter in `vkQueueWaitIdle`
	- \valcom Synchronized internally

* The `memory` parameter in `vkFreeMemory`
	- \valcom Consequence of shared pointer usage

\valcomboxend

* The `memory` parameter in `vkMapMemory`

* The `memory` parameter in `vkUnmapMemory`

* The `buffer` parameter in `vkBindBufferMemory`

\valcombox
* The `image` parameter in `vkBindImageMemory`
	- \valcom Handled by API design

\valcomboxend

* The `queue` parameter in `vkQueueBindSparse`

* The `fence` parameter in `vkQueueBindSparse`

\valcombox
* The `fence` parameter in `vkDestroyFence`
	- \valcom Consequence of shared pointer usage

* The `semaphore` parameter in `vkDestroySemaphore`
	- \valcom Consequence of shared pointer usage

\valcomboxend

* The `event` parameter in `vkDestroyEvent`

* The `event` parameter in `vkSetEvent`

* The `event` parameter in `vkResetEvent`

* The `queryPool` parameter in `vkDestroyQueryPool`

* The `buffer` parameter in `vkDestroyBuffer`

* The `bufferView` parameter in `vkDestroyBufferView`

\valcombox
* The `image` parameter in `vkDestroyImage`
	- \valcom Consequence of shared pointer usage

\valcomboxend

* The `imageView` parameter in `vkDestroyImageView`

* The `shaderModule` parameter in `vkDestroyShaderModule`

* The `pipelineCache` parameter in `vkDestroyPipelineCache`

* The `dstCache` parameter in `vkMergePipelineCaches`

* The `pipeline` parameter in `vkDestroyPipeline`

* The `pipelineLayout` parameter in `vkDestroyPipelineLayout`

* The `sampler` parameter in `vkDestroySampler`

* The `descriptorSetLayout` parameter in `vkDestroyDescriptorSetLayout`

* The `descriptorPool` parameter in `vkDestroyDescriptorPool`

* The `descriptorPool` parameter in `vkResetDescriptorPool`

* The `descriptorPool` member of the `pAllocateInfo` parameter in `vkAllocateDescriptorSets`

* The `descriptorPool` parameter in `vkFreeDescriptorSets`

* The `framebuffer` parameter in `vkDestroyFramebuffer`

* The `renderPass` parameter in `vkDestroyRenderPass`

\valcombox
* The `commandPool` parameter in `vkDestroyCommandPool`
	- \valcom Consequence of shared pointer usage

* The `commandPool` parameter in `vkResetCommandPool`
	- \valcom Synchronized internally

* The `commandPool` member of the `pAllocateInfo` parameter in `vkAllocateCommandBuffers`
	- \valcom Synchronized internally

* The `commandPool` parameter in `vkFreeCommandBuffers`
	- \valcom Synchronized internally

\valcomboxend

* The `commandBuffer` parameter in `vkBeginCommandBuffer`

* The `commandBuffer` parameter in `vkEndCommandBuffer`

* The `commandBuffer` parameter in `vkResetCommandBuffer`

* The `commandBuffer` parameter in `vkCmdBindPipeline`

* The `commandBuffer` parameter in `vkCmdSetViewport`

* The `commandBuffer` parameter in `vkCmdSetScissor`

* The `commandBuffer` parameter in `vkCmdSetLineWidth`

* The `commandBuffer` parameter in `vkCmdSetDepthBias`

* The `commandBuffer` parameter in `vkCmdSetBlendConstants`

* The `commandBuffer` parameter in `vkCmdSetDepthBounds`

* The `commandBuffer` parameter in `vkCmdSetStencilCompareMask`

* The `commandBuffer` parameter in `vkCmdSetStencilWriteMask`

* The `commandBuffer` parameter in `vkCmdSetStencilReference`

* The `commandBuffer` parameter in `vkCmdBindDescriptorSets`

* The `commandBuffer` parameter in `vkCmdBindIndexBuffer`

* The `commandBuffer` parameter in `vkCmdBindVertexBuffers`

* The `commandBuffer` parameter in `vkCmdDraw`

* The `commandBuffer` parameter in `vkCmdDrawIndexed`

* The `commandBuffer` parameter in `vkCmdDrawIndirect`

* The `commandBuffer` parameter in `vkCmdDrawIndexedIndirect`

* The `commandBuffer` parameter in `vkCmdDispatch`

* The `commandBuffer` parameter in `vkCmdDispatchIndirect`

* The `commandBuffer` parameter in `vkCmdCopyBuffer`

* The `commandBuffer` parameter in `vkCmdCopyImage`

* The `commandBuffer` parameter in `vkCmdBlitImage`

* The `commandBuffer` parameter in `vkCmdCopyBufferToImage`

* The `commandBuffer` parameter in `vkCmdCopyImageToBuffer`

* The `commandBuffer` parameter in `vkCmdUpdateBuffer`

* The `commandBuffer` parameter in `vkCmdFillBuffer`

* The `commandBuffer` parameter in `vkCmdClearColorImage`

* The `commandBuffer` parameter in `vkCmdClearDepthStencilImage`

* The `commandBuffer` parameter in `vkCmdClearAttachments`

* The `commandBuffer` parameter in `vkCmdResolveImage`

* The `commandBuffer` parameter in `vkCmdSetEvent`

* The `commandBuffer` parameter in `vkCmdResetEvent`

* The `commandBuffer` parameter in `vkCmdWaitEvents`

* The `commandBuffer` parameter in `vkCmdPipelineBarrier`

* The `commandBuffer` parameter in `vkCmdBeginQuery`

* The `commandBuffer` parameter in `vkCmdEndQuery`

* The `commandBuffer` parameter in `vkCmdResetQueryPool`

* The `commandBuffer` parameter in `vkCmdWriteTimestamp`

* The `commandBuffer` parameter in `vkCmdCopyQueryPoolResults`

* The `commandBuffer` parameter in `vkCmdPushConstants`

* The `commandBuffer` parameter in `vkCmdBeginRenderPass`

* The `commandBuffer` parameter in `vkCmdNextSubpass`

* The `commandBuffer` parameter in `vkCmdEndRenderPass`

* The `commandBuffer` parameter in `vkCmdExecuteCommands`

* The `commandBuffer` parameter in `vkCmdSetDeviceMask`

* The `commandBuffer` parameter in `vkCmdDispatchBase`

\valcombox
* The `commandPool` parameter in `vkTrimCommandPool`
	- \valcom Internally synchronized

\valcomboxend

* The `ycbcrConversion` parameter in `vkDestroySamplerYcbcrConversion`

* The `descriptorUpdateTemplate` parameter in `vkDestroyDescriptorUpdateTemplate`

* The `descriptorSet` parameter in `vkUpdateDescriptorSetWithTemplate`

* The `commandBuffer` parameter in `vkCmdDrawIndirectCount`

* The `commandBuffer` parameter in `vkCmdDrawIndexedIndirectCount`

* The `commandBuffer` parameter in `vkCmdBeginRenderPass2`

* The `commandBuffer` parameter in `vkCmdNextSubpass2`

* The `commandBuffer` parameter in `vkCmdEndRenderPass2`

\valcombox
* The `surface` parameter in `vkDestroySurfaceKHR`
	- \valcom Consequence of shared pointer usage

* The `surface` member of the `pCreateInfo` parameter in `vkCreateSwapchainKHR`
	- \valcom Handled by a combination of API design and swapchain internal synchronization

* The `oldSwapchain` member of the `pCreateInfo` parameter in `vkCreateSwapchainKHR`
	- \valcom Internally synchronized

* The `swapchain` parameter in `vkDestroySwapchainKHR`
	- \valcom Consequence of shared pointer usage

* The `swapchain` parameter in `vkAcquireNextImageKHR`
	- \valcom Internally synchronized

* The `semaphore` parameter in `vkAcquireNextImageKHR`
	- \valcom Internally synchronized

* The `fence` parameter in `vkAcquireNextImageKHR`
	- \valcom Internally synchronized

* The `queue` parameter in `vkQueuePresentKHR`
	- \valcom Internally synchronized

\valcomboxend

* The `surface` parameter in `vkGetDeviceGroupSurfacePresentModesKHR`

* The `surface` parameter in `vkGetPhysicalDevicePresentRectanglesKHR`

* The `display` parameter in `vkCreateDisplayModeKHR`

* The `mode` parameter in `vkGetDisplayPlaneCapabilitiesKHR`

* The `commandBuffer` parameter in `vkCmdSetDeviceMaskKHR`

* The `commandBuffer` parameter in `vkCmdDispatchBaseKHR`

<!-- * The `commandPool` parameter in `vkTrimCommandPoolKHR` -->

* The `commandBuffer` parameter in `vkCmdPushDescriptorSetKHR`

* The `commandBuffer` parameter in `vkCmdPushDescriptorSetWithTemplateKHR`

* The `descriptorUpdateTemplate` parameter in `vkDestroyDescriptorUpdateTemplateKHR`

* The `descriptorSet` parameter in `vkUpdateDescriptorSetWithTemplateKHR`

* The `commandBuffer` parameter in `vkCmdBeginRenderPass2KHR`

* The `commandBuffer` parameter in `vkCmdNextSubpass2KHR`

* The `commandBuffer` parameter in `vkCmdEndRenderPass2KHR`

* The `swapchain` parameter in `vkGetSwapchainStatusKHR`

* The `ycbcrConversion` parameter in `vkDestroySamplerYcbcrConversionKHR`

* The `commandBuffer` parameter in `vkCmdDrawIndirectCountKHR`

* The `commandBuffer` parameter in `vkCmdDrawIndexedIndirectCountKHR`

* The `callback` parameter in `vkDestroyDebugReportCallbackEXT`

* The `object` member of the `pTagInfo` parameter in `vkDebugMarkerSetObjectTagEXT`

* The `object` member of the `pNameInfo` parameter in `vkDebugMarkerSetObjectNameEXT`

* The `commandBuffer` parameter in `vkCmdBindTransformFeedbackBuffersEXT`

* The `commandBuffer` parameter in `vkCmdBeginTransformFeedbackEXT`

* The `commandBuffer` parameter in `vkCmdEndTransformFeedbackEXT`

* The `commandBuffer` parameter in `vkCmdBeginQueryIndexedEXT`

* The `commandBuffer` parameter in `vkCmdEndQueryIndexedEXT`

* The `commandBuffer` parameter in `vkCmdDrawIndirectByteCountEXT`

* The `commandBuffer` parameter in `vkCmdDrawIndirectCountAMD`

* The `commandBuffer` parameter in `vkCmdDrawIndexedIndirectCountAMD`

* The `commandBuffer` parameter in `vkCmdBeginConditionalRenderingEXT`

* The `commandBuffer` parameter in `vkCmdEndConditionalRenderingEXT`

* The `commandBuffer` parameter in `vkCmdProcessCommandsNVX`

* The `commandBuffer` parameter in `vkCmdReserveSpaceForCommandsNVX`

* The `objectTable` parameter in `vkDestroyObjectTableNVX`

* The `objectTable` parameter in `vkRegisterObjectsNVX`

* The `objectTable` parameter in `vkUnregisterObjectsNVX`

* The `commandBuffer` parameter in `vkCmdSetViewportWScalingNV`

* The `swapchain` parameter in `vkGetRefreshCycleDurationGOOGLE`

* The `swapchain` parameter in `vkGetPastPresentationTimingGOOGLE`

* The `commandBuffer` parameter in `vkCmdSetDiscardRectangleEXT`

* The `objectHandle` member of the `pNameInfo` parameter in `vkSetDebugUtilsObjectNameEXT`

* The `objectHandle` member of the `pTagInfo` parameter in `vkSetDebugUtilsObjectTagEXT`

* The `messenger` parameter in `vkDestroyDebugUtilsMessengerEXT`

* The `commandBuffer` parameter in `vkCmdSetSampleLocationsEXT`

* The `validationCache` parameter in `vkDestroyValidationCacheEXT`

* The `dstCache` parameter in `vkMergeValidationCachesEXT`

* The `commandBuffer` parameter in `vkCmdBindShadingRateImageNV`

* The `commandBuffer` parameter in `vkCmdSetViewportShadingRatePaletteNV`

* The `commandBuffer` parameter in `vkCmdSetCoarseSampleOrderNV`

* The `commandBuffer` parameter in `vkCmdWriteBufferMarkerAMD`

* The `commandBuffer` parameter in `vkCmdDrawMeshTasksNV`

* The `commandBuffer` parameter in `vkCmdDrawMeshTasksIndirectNV`

* The `commandBuffer` parameter in `vkCmdDrawMeshTasksIndirectCountNV`

* The `commandBuffer` parameter in `vkCmdSetExclusiveScissorNV`

* The `commandBuffer` parameter in `vkCmdSetLineStippleEXT`

\valboxend