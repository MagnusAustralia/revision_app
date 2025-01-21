<script lang="ts">
	import MarkdownRenderer from '../lib/MarkdownRenderer.svelte';
	import type { Subject } from './+page.ts'; // Import the Subject type

	export let data: { subjects: Subject[] };

	let showCase: 'Subjects' | 'Books' | 'Sections' | 'Topics' | 'Topic' = 'Subjects';
	let subjects = data.subjects;

	let books: { name: string; id: number }[] = [];
	let sections: { name: string; id: number }[] = [];
	let topics: { name: string; id: number; markdown: string }[] = [];

	let subjectID: number | null = null;
	let bookID: number | null = null;
	let sectionID: number | null = null;
	let topicID: number | null = null;

	// Reactive statement to find the selected topic
	$: selectedTopic = topicID !== null ? topics.find((t) => t.id === topicID) : null;

	async function getBooks(id: number | null) {
		let response = await fetch(`http://0.0.0.0:8080/books?subject_id=${id}`, { method: 'GET' });
		if (response.ok) {
			books = await response.json();
			subjectID = id;
			if (books.length === 0) {
				getTopics(null);
			} else {
				showCase = 'Books';
			}
		} else {
			console.error('Failed to fetch books');
		}
	}

	async function getSections(id: number) {
		let response = await fetch(`http://0.0.0.0:8080/sections?book_id=${id}`, { method: 'GET' });
		if (response.ok) {
			bookID = id;
			sections = await response.json();
			showCase = 'Sections';
		} else {
			console.error('Failed to fetch sections');
		}
	}

	async function getTopics(id: number | null) {
		let response: Response;
		sectionID = id;
		try {
			if (id) {
				response = await fetch(
					`http://0.0.0.0:8080/topics?subject_id=${subjectID}&book_id=${bookID}&section_id=${id}`,
					{ method: 'GET' }
				);
			} else {
				response = await fetch(`http://0.0.0.0:8080/topics?subject_id=${subjectID}`, {
					method: 'GET'
				});
			}
			topics = await response.json();
			showCase = 'Topics';
		} catch (error) {
			console.error('Error fetching topics:', error);
		}
	}
</script>

<main class="grid h-screen place-items-center">
	<div class="flex flex-col space-y-5">
		<h1 class="flex items-center justify-center">Topics</h1>
		<ul class="flex items-center justify-center gap-x-5 flex-wrap gap-y-5 flex-row">
			{#if showCase === 'Subjects'}
				{#each subjects as subject}
					<button
						class="card container bg-white rounded-lg text-black"
						on:click={() => getBooks(subject.id)}
						on:keydown={(e) => {
							if (e.key === 'Enter' || e.key === ' ') getBooks(subject.id);
						}}
						aria-label={`View books for ${subject.name}`}
					>
						{subject.name}
					</button>
				{/each}
			{:else if showCase === 'Books'}
				{#each books as book}
					<button
						class="card container bg-white rounded-lg text-black"
						on:click={() => getSections(book.id)}
						on:keydown={(e) => {
							if (e.key === 'Enter' || e.key === ' ') getSections(book.id);
						}}
						aria-label={`View books for ${book.name}`}
					>
						{book.name}
					</button>
				{/each}
			{:else if showCase === 'Sections'}
				{#each sections as section}
					<button
						class="card container bg-white rounded-lg text-black"
						on:click={() => getTopics(section.id)}
						on:keydown={(e) => {
							if (e.key === 'Enter' || e.key === ' ') getTopics(section.id);
						}}
						aria-label={`View sections for ${section.name}`}
					>
						{section.name}
					</button>
				{/each}
			{:else if showCase === 'Topics'}
				{#each topics as topic}
					<button
						class="card container bg-white rounded-lg text-black"
						aria-label={`View topics for ${topic.name}`}
						on:click={() => {
							topicID = topic.id;
							showCase = 'Topic';
						}}
						on:keydown={(e) => {
							if (e.key === 'Enter' || e.key === ' ') {
								topicID = topic.id;
								showCase = 'Topic';
							}
						}}
					>
						{topic.name}
						<br />
						{topic.id}
					</button>
				{/each}
			{:else if showCase === 'Topic'}
				<!-- Use 'markdownContent' as the prop name -->
				{#if selectedTopic}
					<MarkdownRenderer markdownContent={selectedTopic.markdown} />
				{:else}
					<p>No topic selected.</p>
				{/if}
			{/if}
		</ul>
	</div>
</main>

<style>
	.card {
		width: 190px;
		height: 100px;
		border-radius: 30px;
		background: #e0e0e0;
	}
</style>
