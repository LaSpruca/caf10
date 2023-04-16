<script lang="ts">
	import { sleep } from '$lib/util';
	import Cards from './Cards.svelte';
	import Scores from './Scores.svelte';
	import Waiting from './Waiting.svelte';
	import Winner from './Winner.svelte';
	import { init, gameCode } from '$lib/display_state';
	import { onMount } from 'svelte';

	onMount(() => {
		init();
	});

	let shown = false;
	let cards: (string | [string, string])[] = [];
	let players = new Map([
		['Connor', 7],
		['Molly', 2],
		['Facility', 8],
		['You Ran', 10]
	]);

	let selected = -1;

	let screen: 'waiting' | 'cards' | 'scores' | 'winner' = 'waiting';

	let packs = ['Base', 'Green'];
</script>

<p class="fixed text-xl font-bold text-gray-500">
	Game Code: {('' + $gameCode).substring(0, 3)}
	{('' + $gameCode).substring(3)}
</p>

{#if screen == 'waiting'}
	<Waiting
		players={[...players.keys()]}
		{packs}
		on:start={async () => {
			screen = 'cards';

			await sleep(1000);
			cards = [...cards, 'AA'];

			await sleep(400);
			cards = [...cards, 'BB'];

			await sleep(1500);
			cards = [...cards, 'CC'];

			await sleep(600);
			cards = [...cards, 'DD'];

			await sleep(500);
			shown = true;

			await sleep(2500);
			selected = 1;

			await sleep(3000);
			shown = false;
			selected = 0;

			screen = 'scores';
			await sleep(3000);

			screen = 'cards';

			cards = [];
			await sleep(1000);
			cards = [...cards, ['AA', 'AB']];

			await sleep(400);
			cards = [...cards, ['BB', 'BC']];

			await sleep(1500);
			cards = [...cards, ['CC', 'CD']];

			await sleep(600);
			cards = [...cards, ['DD', 'DE']];

			await sleep(500);
			shown = true;

			await sleep(2500);
			selected = 1;

			await sleep(3000);
			shown = false;
			screen = 'scores';

			await sleep(4000);
			screen = 'winner';
		}}
	/>
{:else if screen == 'cards'}
	<Cards blackCard="Who flooded the toilet" whiteCards={cards} {selected} {shown} />
{:else if screen == 'scores'}
	<Scores {players} />
{:else if screen == 'winner'}
	<Winner
		name={'You Ran'}
		on:restart={() => {
			cards = [];
			screen = 'waiting';
		}}
	/>
{/if}
