<script lang="ts">
	import Card from '$lib/Card.svelte';
	import { createEventDispatcher } from 'svelte';

	export let cards: string[];

	export let numberCards = 1;

	let selectedCards: number[] = [];
	let selectedCard: number | undefined;

	$: if (!cards) {
		selectedCards = [];
		selectedCard = undefined;
	}

	let dispatch = createEventDispatcher<{ selected: number[] }>();

	$: if (selectedCards.length == numberCards) {
		dispatch('selected', selectedCards);
	}

	const selectCard = (index: number) => () => {
		if (!selectedCards.includes(index))
			if (selectedCard != index) selectedCard = index;
			else selectedCard = undefined;
	};
</script>

<h1 class="text-center text-2xl font-bold text-gray-100">Pick {numberCards}.</h1>

<div class="flex flex-row flex-wrap items-center justify-center gap-5 p-10">
	{#each cards as card, index}
		<button on:click={selectCard(index)} class="min-w-min">
			<Card
				color={!selectedCards.includes(index) ? 'white' : 'gray'}
				content={card}
				big={index == selectedCard}
			/>
		</button>
	{/each}
</div>

<button
	class="fixed left-[50vw] z-30 w-40 translate-x-[-50%] rounded-full bg-gray-900 p-3 text-center text-xl font-bold text-gray-100 transition-all duration-200"
	class:bottom-5={typeof selectedCard !== 'undefined'}
	class:bottom-[-100%]={typeof selectedCard == 'undefined'}
	on:click={() => {
		if (typeof selectedCard !== 'undefined') {
			selectedCards = [...selectedCards, selectedCard];
			selectedCard == undefined;
		}
	}}
>
	Submit
</button>
