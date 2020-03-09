// Extracts valid usage from HTML vulkan spec into markdown
function extract_valid_usage(selector) {
	const ul = document.querySelector(selector)

	let result = "\\valbox\n\n"

	ul.querySelectorAll("li > p").forEach(usage => {
		let usage_text = "*"

		usage.childNodes.forEach(child => {
			if ("tagName" in child) {
				if (child.tagName == "A") {
					if (child.innerText != "") {
						if (child.innerText.indexOf(" ") == -1) {
							usage_text += "`" + child.innerText + "`"
						} else {
							usage_text += child.innerText
						}
					}
				} else if (child.tagName == "STRONG") {
					usage_text += child.innerText
				} else if (child.tagName == "CODE") {
					usage_text += "`" + child.innerText + "`"
				} else if (child.tagName == "SPAN") {
					usage_text += "`" + child.innerText + "`"
				} else if (child.tagName == "EM") {
					usage_text += "_" + child.innerText + "_"
				} else {
					console.error("Unexpected child", child)
				}
			} else {
				usage_text += child.data
			}
		})

		usage_text = usage_text.replace(/\n/g, " ")
		result += usage_text + "\n\n"
	})

	result = result + "\\valboxend"
	
	const objectURL = URL.createObjectURL(
		new Blob(
			[result],
			{ type: "text/plain" }
		)
	)
	window.open(objectURL, "_blank")
	return objectURL
}

// vkCreateInstance: extract_valid_usage("div.sect1:nth-child(4) > div:nth-child(2) > div:nth-child(3) > div:nth-child(4) > div:nth-child(1) > div:nth-child(5) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")
// vkCreateDevice: extract_valid_usage("div.sect1:nth-child(5) > div:nth-child(2) > div:nth-child(4) > div:nth-child(9) > div:nth-child(3) > div:nth-child(1) > div:nth-child(7) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")
// VkDeviceCreateInfo: extract_valid_usage("div.sect1:nth-child(5) > div:nth-child(2) > div:nth-child(4) > div:nth-child(9) > div:nth-child(4) > div:nth-child(1) > div:nth-child(4) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")
// VkDeviceQueueCreateInfo: extract_valid_usage("div.sect1:nth-child(5) > div:nth-child(2) > div:nth-child(5) > div:nth-child(3) > div:nth-child(3) > div:nth-child(1) > div:nth-child(4) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")

// VkSwapchainCreateInfoKHR: extract_valid_usage("div.sect1:nth-child(33) > div:nth-child(2) > div:nth-child(10) > div:nth-child(13) > div:nth-child(1) > div:nth-child(11) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")

// vkCreateCommandPool: extract_valid_usage("div.sect1:nth-child(6) > div:nth-child(2) > div:nth-child(6) > div:nth-child(3) > div:nth-child(1) > div:nth-child(4) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")
// VkCommandPoolCreateInfo: extract_valid_usage("div.sect1:nth-child(6) > div:nth-child(2) > div:nth-child(6) > div:nth-child(4) > div:nth-child(1) > div:nth-child(4) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")
// VkCommandBufferAllocateInfo: extract_valid_usage("div.sect1:nth-child(6) > div:nth-child(2) > div:nth-child(7) > div:nth-child(3) > div:nth-child(1) > div:nth-child(4) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")

// VkRenderPassCreateInfo2: extract_valid_usage("div.sect1:nth-child(8) > div:nth-child(2) > div:nth-child(13) > div:nth-child(42) > div:nth-child(1) > div:nth-child(9) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")
// VkAttachmentDescription2: extract_valid_usage("div.sect1:nth-child(8) > div:nth-child(2) > div:nth-child(13) > div:nth-child(43) > div:nth-child(1) > div:nth-child(10) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")
// VkSubpassDescription2: extract_valid_usage("div.sect1:nth-child(8) > div:nth-child(2) > div:nth-child(13) > div:nth-child(45) > div:nth-child(1) > div:nth-child(8) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")
// VkAttachmentReference2: extract_valid_usage("div.sect1:nth-child(8) > div:nth-child(2) > div:nth-child(13) > div:nth-child(49) > div:nth-child(1) > div:nth-child(10) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")
// VkSubpassDependency2: extract_valid_usage("div.sect1:nth-child(8) > div:nth-child(2) > div:nth-child(13) > div:nth-child(51) > div:nth-child(1) > div:nth-child(8) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")

// vkCreateFramebuffer: extract_valid_usage("div.sect1:nth-child(8) > div:nth-child(2) > div:nth-child(15) > div:nth-child(3) > div:nth-child(1) > div:nth-child(4) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")
// VkFramebufferCreateInfo: extract_valid_usage("div.sect1:nth-child(8) > div:nth-child(2) > div:nth-child(15) > div:nth-child(4) > div:nth-child(1) > div:nth-child(8) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")

// vkCreateComputePipelines: extract_valid_usage("div.sect1:nth-child(10) > div:nth-child(2) > div:nth-child(18) > div:nth-child(4) > div:nth-child(1) > div:nth-child(4) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")
// VkComputePipelineCreateInfo: extract_valid_usage("div.sect1:nth-child(10) > div:nth-child(2) > div:nth-child(18) > div:nth-child(5) > div:nth-child(1) > div:nth-child(5) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")
// VkPipelineShaderStageCreateInfo: extract_valid_usage("div.sect1:nth-child(10) > div:nth-child(2) > div:nth-child(18) > div:nth-child(6) > div:nth-child(1) > div:nth-child(4) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")
// vkCreateGraphicsPipelines: extract_valid_usage("div.sect1:nth-child(10) > div:nth-child(2) > div:nth-child(19) > div:nth-child(3) > div:nth-child(1) > div:nth-child(5) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")
// VkGraphicsPipelineCreateInfo: extract_valid_usage("div.sect1:nth-child(10) > div:nth-child(2) > div:nth-child(19) > div:nth-child(4) > div:nth-child(1) > div:nth-child(6) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")

// vkCreateBuffer: extract_valid_usage("div.sect1:nth-child(12) > div:nth-child(2) > div:nth-child(2) > div:nth-child(3) > div:nth-child(1) > div:nth-child(4) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")
// VkBufferCreateInfo: extract_valid_usage("div.sect1:nth-child(12) > div:nth-child(2) > div:nth-child(2) > div:nth-child(4) > div:nth-child(1) > div:nth-child(4) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")
// VkBufferViewCreateInfo: extract_valid_usage("div.sect1:nth-child(12) > div:nth-child(2) > div:nth-child(3) > div:nth-child(4) > div:nth-child(1) > div:nth-child(4) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")

// vkCreateImage: extract_valid_usage("div.sect1:nth-child(12) > div:nth-child(2) > div:nth-child(4) > div:nth-child(3) > div:nth-child(1) > div:nth-child(4) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")
// VkImageCreateInfo: extract_valid_usage("div.sect1:nth-child(12) > div:nth-child(2) > div:nth-child(4) > div:nth-child(4) > div:nth-child(1) > div:nth-child(13) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")
// VkImageViewCreateInfo: extract_valid_usage("div.sect1:nth-child(12) > div:nth-child(2) > div:nth-child(6) > div:nth-child(5) > div:nth-child(1) > div:nth-child(14) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")
// VkImageSubresourceRange: extract_valid_usage("div.sect1:nth-child(12) > div:nth-child(2) > div:nth-child(6) > div:nth-child(9) > div:nth-child(1) > div:nth-child(12) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")

// VkDescriptorSetLayoutCreateInfo: extract_valid_usage("div.sect1:nth-child(14) > div:nth-child(2) > div:nth-child(5) > div:nth-child(3) > div:nth-child(4) > div:nth-child(1) > div:nth-child(4) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")
// VkDescriptorSetLayoutBinding: extract_valid_usage("div.sect1:nth-child(14) > div:nth-child(2) > div:nth-child(5) > div:nth-child(3) > div:nth-child(7) > div:nth-child(1) > div:nth-child(6) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")
// VkPipelineLayoutCreateInfo: extract_valid_usage("div.sect1:nth-child(14) > div:nth-child(2) > div:nth-child(5) > div:nth-child(4) > div:nth-child(4) > div:nth-child(1) > div:nth-child(4) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")
// VkPushConstantRange: extract_valid_usage("div.sect1:nth-child(14) > div:nth-child(2) > div:nth-child(5) > div:nth-child(4) > div:nth-child(6) > div:nth-child(1) > div:nth-child(4) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")

// VkDescriptorPoolCreateInfo: extract_valid_usage("div.sect1:nth-child(14) > div:nth-child(2) > div:nth-child(5) > div:nth-child(5) > div:nth-child(4) > div:nth-child(1) > div:nth-child(9) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")
// VkDescriptorPoolSize: extract_valid_usage("div.sect1:nth-child(14) > div:nth-child(2) > div:nth-child(5) > div:nth-child(5) > div:nth-child(8) > div:nth-child(1) > div:nth-child(5) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")
// VkDescriptorSetAllocateInfo: extract_valid_usage("div.sect1:nth-child(14) > div:nth-child(2) > div:nth-child(5) > div:nth-child(5) > div:nth-child(12) > div:nth-child(1) > div:nth-child(4) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")
// VkDescriptorSetVariableDescriptorCountAllocateInfo: extract_valid_usage("div.sect1:nth-child(14) > div:nth-child(2) > div:nth-child(5) > div:nth-child(5) > div:nth-child(13) > div:nth-child(1) > div:nth-child(8) > div:nth-child(1) > div:nth-child(2) > ul:nth-child(1)")
