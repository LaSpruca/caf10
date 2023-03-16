<script lang="ts">
	import { fadeSettings } from '$lib/util';
	import { createEventDispatcher } from 'svelte';
	import { fade, fly } from 'svelte/transition';

	export let players: string[];
	export let packs: string[];

	let dispatch = createEventDispatcher<{ start: void }>();

	let enabled = 0;

	const enable =
		(id: string) =>
		(
			event: Event & {
				currentTarget: EventTarget & HTMLInputElement;
			}
		) => {
			let isEnabled = event.currentTarget.checked;
			enabled += isEnabled ? 1 : -1;
			console.log(`${id} was toggled ${isEnabled}`);
		};
</script>

<div in:fade={fadeSettings.in} out:fade={fadeSettings.out}>
	<h2 class="pt-10 text-center text-5xl font-black text-gray-100">Players</h2>
	<div class="flex content-center justify-center drop-shadow-lg">
		{#each players as player}
			<div transition:fly={{ y: -10 }} class="p-10 text-2xl font-bold text-gray-100 ">
				{player}
			</div>
		{/each}
	</div>

	<h2 class="pt-10 text-center text-5xl font-black text-gray-100">Packs</h2>

	<div class="flex items-center justify-center gap-2">
		{#each packs as pack}
			<div class="flex items-center justify-center">
				<input type="checkbox" id="{pack}-cb" class="h-5 w-5" on:change={enable(pack)} /><label
					for="{pack}-cb"
					class="p-1 text-2xl font-bold text-gray-200">{pack}</label
				>
			</div>
		{/each}
	</div>
	<div class="flex w-full items-center justify-center p-1">
		<button
			on:click={() => dispatch('start')}
			class="rounded bg-white p-2 text-2xl font-bold shadow-sm transition-colors duration-150 disabled:bg-gray-300 disabled:text-gray-500"
		>
			Upload
		</button>
	</div>

	<div class="flex w-full items-center justify-center p-10">
		<button
			disabled={enabled == 0}
			on:click={() => dispatch('start')}
			class="rounded bg-white p-2 text-2xl font-bold shadow-sm transition-colors duration-150 disabled:bg-gray-300 disabled:text-gray-500"
		>
			START
		</button>
	</div>
</div>
