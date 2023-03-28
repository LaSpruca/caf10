<script lang="ts">
	import Card from '$lib/Card.svelte';
	import { fadeSettings } from '$lib/util';
	import { fade } from 'svelte/transition';

	export let blackCard: string;
	export let whiteCards: (string | [string, string])[];
	export let shown: boolean;
	export let selected: number = -1;
</script>

<div in:fade={fadeSettings.in} out:fade={fadeSettings.out}>
	<div class="flex items-center justify-center">
		<Card content={blackCard} color="black" />
	</div>

	<div class="flex flex-wrap justify-center gap-3 p-10">
		{#each whiteCards as content, index}
			{#if typeof content === 'string'}
				<Card {content} color="white" up={index == selected} {shown} />
			{:else}
				<Card content={content[0]} color="white" up={index == selected} {shown} />
				<Card content={content[1]} color="white" up={index == selected} {shown} />
			{/if}
		{/each}
	</div>
</div>
