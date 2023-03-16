<script lang="ts">
	import { fadeSettings } from '$lib/util';
	import { createEventDispatcher } from 'svelte';
	import { fade, fly } from 'svelte/transition';

	export let players: string[];

	let dispatch = createEventDispatcher<{ start: void }>();
</script>

<div in:fade={fadeSettings.in} out:fade={fadeSettings.out}>
	<div class="flex content-center justify-center pt-10 drop-shadow-lg">
		{#each players as player}
			<div transition:fly={{ y: -10 }} class="p-10 text-2xl font-bold text-gray-100">
				{player}
			</div>
		{/each}
	</div>

	<div class="flex w-full items-center justify-center p-10">
		<button
			on:click={() => dispatch('start')}
			class="rounded bg-white p-2 text-2xl font-bold shadow-sm"
		>
			START
		</button>
	</div>
</div>
