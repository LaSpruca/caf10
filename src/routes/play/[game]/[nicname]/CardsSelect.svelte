<script lang="ts">
	import Card from '$lib/Card.svelte';
	import { fadeSettings } from '$lib/util';
	import { createEventDispatcher } from 'svelte';
	import { fade } from 'svelte/transition';

	export let cards: string[];

	let selected: number | undefined;

	$: if (!cards) {
		selected = undefined;
	}

	let dispatch = createEventDispatcher<{ selected: number }>();
</script>

<div class="flex flex-col items-center justify-center gap-5 p-10">
	<h1 class="text-center text-2xl font-bold text-gray-100">Pick One.</h1>

	{#each cards as card, index}
		<button
			on:click={() => {
				if (selected != index) selected = index;
				else selected = undefined;
			}}><Card color="white" content={card} big={index == selected} /></button
		>
	{/each}
</div>

<button
	class="fixed left-[50vw] z-30 w-40 translate-x-[-50%] rounded-full bg-gray-900 p-3 text-center text-xl font-bold text-gray-100 transition-all duration-200"
	class:bottom-5={typeof selected !== 'undefined'}
	class:bottom-[-100%]={typeof selected === 'undefined'}
	on:click={() => dispatch('selected', selected)}
>
	Submit
</button>
