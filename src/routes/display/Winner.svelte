<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { cubicIn } from 'svelte/easing';
	import { fade } from 'svelte/transition';

	export let name: string;

	let dispatch = createEventDispatcher<{ restart: void }>();
</script>

<div transition:fade="{{duration: 1000, easing: cubicIn}}" class="p-20">
	<h1 class="text-center text-6xl font-bold text-gray-200 mb-3">WINNER</h1>
	<h2 class="text-center text-5xl font-black text-gray-100">{name}</h2>
	<div class="flex items-center justify-center w-full p-10">
		<button on:click={() => dispatch('restart')} class="bg-white rounded p-1 shadow-sm">
			Restart
		</button>
	</div>
</div>
