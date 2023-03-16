<script lang="ts">
	import { cubicIn } from 'svelte/easing';
	import { fade } from 'svelte/transition';
	import ScoreDisplay from './ScoreDisplay.svelte';
	export let players: Map<string, number>;

	let playersSorted = [...players.entries()];
	playersSorted.sort(([_a, a], [_b, b]) => b - a);
	playersSorted = playersSorted.splice(0, 5);
</script>

<div transition:fade="{{duration: 1000, easing: cubicIn}}" class="w-full">
	<h1 class="text-white text-center font-bold text-6xl">Leaderboard</h1>

	<div class="flex gap-10 content-center items-center p-10 flex-col w-full">
		{#each playersSorted as [player, score]}
			<ScoreDisplay {player} {score} />
		{/each}
	</div>
</div>
