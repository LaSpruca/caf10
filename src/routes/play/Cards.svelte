<script lang="ts">
	import Card from '$lib/Card.svelte';
	import { fadeSettings } from '$lib/util';
	import { fade } from 'svelte/transition';
	import CardsSelect from './CardsSelect.svelte';

	export let blackCard: string;
	export let cards: string[];
	export let two = false;
</script>

<div in:fade={fadeSettings.in} out:fade={fadeSettings.out}>
	<h1 class="text-center text-2xl font-bold text-gray-100">ANSWER THE CARD</h1>

	<div class="flex items-center justify-center p-10">
		<Card color="black" content={blackCard} />
	</div>
	<CardsSelect {cards} on:selected numberCards={two ? 2 : 1} />
</div>
