<script lang="ts">
	import { sleep } from '$lib/util';
	import Cards from './Cards.svelte';
	import Scores from './Scores.svelte';
	import Waiting from './Waiting.svelte';
	import Winner from './Winner.svelte';

	let shown = false;
	let cards: string[] = [];
	let players = new Map([
		['Connor', 7],
		['Molly', 2],
		['Facility', 8],
		['You Ran', 10]
	]);

	let selected = -1;

	let screen: 'waiting' | 'cards' | 'scores' | 'winner' = 'waiting';

	let packs = ['Base', 'Green'];

	let code = 123456;
</script>

<p class="fixed text-xl font-bold text-gray-500">
	Game Code: {('' + code).substring(0, 3)}
	{('' + code).substring(3)}
</p>

{#if screen == 'waiting'}
	<Waiting
		players={[...players.keys()]}
		{packs}
		on:start={async () => {
			screen = 'cards';

			await sleep(1000);
			cards = [...cards, 'Sandrew'];

			await sleep(400);
			cards = [...cards, 'Matthew'];

			await sleep(1500);
			cards = [...cards, 'Trison'];

			await sleep(600);
			cards = [...cards, 'Sara'];

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
