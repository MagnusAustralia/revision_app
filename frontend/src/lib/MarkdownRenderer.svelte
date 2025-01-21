<script lang="ts">
	import { onMount } from 'svelte';
	import katex from 'katex';
	import { marked } from 'marked'; // Correctly import `marked`

	export let content: string; // Markdown content passed as a prop
	let renderedHtml: string = '';

	// Configure `marked` with optional sanitization
	marked.setOptions({
		gfm: true, // GitHub-flavored markdown
		breaks: true, // Convert line breaks to <br>
		sanitize: true // Prevent XSS attacks by sanitizing input
	});

	// Reactive statement to update `renderedHtml` when `content` changes
	$: {
		renderedHtml = marked(content);

		// After rendering Markdown, convert LaTeX using KaTeX
		const container = document.querySelector('.content');
		if (container) {
			const elements = container.querySelectorAll('.math');
			elements.forEach((el) => {
				katex.render(el.textContent || '', el, {
					throwOnError: false
				});
			});
		}
	}
</script>

<main class="w-full h-full">
	<h1 class="flex items-center justify-center">Math Content</h1>
	<div class="content">
		<!-- Render dynamic markdown content as HTML -->
		{@html renderedHtml}
	</div>
</main>

<style>
	.content {
		font-size: 18px;
		line-height: 1.6;
	}
	.math {
		font-size: 1.2em;
		color: #333;
	}
</style>
