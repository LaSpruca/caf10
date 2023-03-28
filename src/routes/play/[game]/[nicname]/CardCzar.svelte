<script lang="ts">
	import Card from '$lib/Card.svelte';
	import { fadeSettings } from '$lib/util';
	import { fade } from 'svelte/transition';
	import CardsCzarSelect from './CardsCzarSelect.svelte';

	export let blackCard: string;
	export let cards: (string | [string, string])[] | undefined = undefined;
</script>

<div in:fade={fadeSettings.in} out:fade={fadeSettings.out}>
	<h1 class="text-center text-2xl font-bold text-gray-100">YOU ARE THE CARD CZAR</h1>

	<div class="flex items-center justify-center p-10">
		<Card color="black" content={blackCard} />
	</div>
	{#if cards}
		<CardsCzarSelect {cards} on:selected />
	{/if}
</div>
