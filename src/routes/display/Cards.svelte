<script lang="ts">
	import Card from '$lib/Card.svelte';
	import { fade } from 'svelte/transition';

	export let blackCard: string;
	export let whiteCards: string[];
	export let shown: boolean;
	export let selected: number = -1;
</script>

<div transition:fade>
	<div class="flex justify-center items-center">
		<Card content={blackCard} color="black" />
	</div>

	<div class="flex gap-3 p-10 flex-wrap justify-center">
		{#each whiteCards as content, index}
			<Card {content} color="white" up={index == selected} {shown} />
		{/each}
	</div>
</div>
