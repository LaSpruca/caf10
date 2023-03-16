<script lang="ts">
	import { fadeSettings } from '$lib/util';
	import { fade } from 'svelte/transition';
	import ScoreDisplay from './ScoreDisplay.svelte';
	export let players: Map<string, number>;

	let playersSorted = [...players.entries()];
	playersSorted.sort(([_a, a], [_b, b]) => b - a);
	playersSorted = playersSorted.splice(0, 5);
</script>

<div in:fade={fadeSettings.in} out:fade={fadeSettings.out} class="w-full">
	<h1 class="text-center text-6xl font-bold text-white">Leaderboard</h1>

	<div class="flex w-full flex-col content-center items-center gap-10 p-10">
		{#each playersSorted as [player, score]}
			<ScoreDisplay {player} {score} />
		{/each}
	</div>
</div>
