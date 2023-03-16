<script lang="ts">
	import { fadeSettings } from '$lib/util';
	import { createEventDispatcher } from 'svelte';
	import { fade } from 'svelte/transition';

	export let name: string;

	let dispatch = createEventDispatcher<{ restart: void }>();
</script>

<div in:fade={fadeSettings.in} out:fade={fadeSettings.out} class="p-20">
	<h1 class="mb-3 text-center text-6xl font-bold text-gray-200">WINNER</h1>
	<h2 class="text-center text-5xl font-black text-gray-100">{name}</h2>
	<div class="flex w-full items-center justify-center p-10">
		<button on:click={() => dispatch('restart')} class="rounded bg-white p-1 shadow-sm">
			Restart
		</button>
	</div>
</div>
