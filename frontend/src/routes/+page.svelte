<script lang="ts">
	type TopicsType = {
		[key: string]: string;
	};
    // Import the JSON file
    url: String = ./files/
    import topicsJson from url + 'index.json';
    const topics: { [key: string]: string } = topicsJson;

    // Extract the keys from the JSON object
    const keys = Object.keys(topics);
</script>

<main class="w-full h-full ">
    <h1 class="flex items-center justify-center">Topics</h1>
    <ul class="flex items-center justify-center gap-x-5">
        {#each keys as key}
            <li class="container bg-white rounded-lg text-black" on:click={url += data[key]}>{key}</li>
        {/each}
    </ul>
</main>
