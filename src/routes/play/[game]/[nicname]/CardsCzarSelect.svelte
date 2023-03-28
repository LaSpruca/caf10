<script lang="ts">
	import Card from '$lib/Card.svelte';
	import { createEventDispatcher } from 'svelte';

	export let cards: (string | [string, string])[];

	let selectedCard: number | undefined;

	$: if (!cards) {
		selectedCard = undefined;
	}

	let dispatch = createEventDispatcher<{ selected: number }>();
	const selectCard = (index: number) => () => {
		if (selectedCard != index) selectedCard = index;
		else selectedCard = undefined;
	};
</script>

<h1 class="text-center text-2xl font-bold text-gray-100">Pick One.</h1>

<div class="flex flex-row flex-wrap items-center justify-center gap-7 p-10">
	{#each cards as card, index}
		{#if typeof card == 'string'}
			<button on:click={selectCard(index)}>
				<Card color="white" content={card} big={index == selectedCard} />
			</button>
		{:else}
			<div class="flow-wrap flex flex-col gap-3 lg:flex-row">
				<button on:click={selectCard(index)}>
					<Card color="white" content={card[0]} big={index == selectedCard} />
				</button>
				<button on:click={selectCard(index)}>
					<Card color="white" content={card[1]} big={index == selectedCard} />
				</button>
			</div>
		{/if}
	{/each}
</div>

<button
	class="fixed left-[50vw] z-30 w-40 translate-x-[-50%] rounded-full bg-gray-900 p-3 text-center text-xl font-bold text-gray-100 transition-all duration-200"
	class:bottom-5={typeof selectedCard !== 'undefined'}
	class:bottom-[-100%]={typeof selectedCard == 'undefined'}
	on:click={() => {
		if (typeof selectedCard !== 'undefined') {
			dispatch('selected', selectedCard);
			selectedCard == undefined;
		}
	}}
>
	Submit
</button>
