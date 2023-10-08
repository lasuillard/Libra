<script>
	import ipc from '$lib/ipc';

	let name = '';
	let greetMsg = '';

	// eslint-disable-next-line jsdoc/require-jsdoc
	async function greet() {
		greetMsg = await ipc.greet(name);
	}

	// eslint-disable-next-line jsdoc/require-jsdoc
	async function get_envs() {
		console.log(await ipc.get_envs());
	}
</script>

<div>
	<h1 id="greeting-message" class="text-3xl">Welcome to SvelteKit</h1>
	<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
	<input id="greet-input" placeholder="Enter name" bind:value={name} />
	<button on:click={greet}>Greet</button>
	<p>{greetMsg}</p>
	<button on:click={get_envs}>GetEnvs</button>
</div>

<style lang="postcss">
	:global(html) {
		background-color: theme(colors.gray.100);
	}
</style>
