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

	let code = 123456;

	$: if (screen == "cards") {
		sleep(1000).then(() => {
			cards = [...cards, 'Brian'];
		});

		sleep(1400).then(() => {
			cards = [...cards, 'Molly'];
		});

		sleep(2000).then(() => {
			cards = [...cards, 'Sandrew'];
		});

		sleep(2600).then(() => {
			cards = [...cards, 'Mathew'];

			sleep(500).then(() => {
				shown = true;
			});

			sleep(1500).then(() => {
				selected = 2;
			});

			sleep(3000).then(() => {
				screen = 'scores';
			});

			sleep(9000).then(() => {
				screen = 'winner';
			});
		});
	}
</script>

<p class="font-bold text-gray-500 text-xl fixed">
	Game Code: {('' + code).substring(0, 3)}
	{('' + code).substring(3)}
</p>

{#if screen == 'waiting'}
	<Waiting players={[...players.keys()]} on:start={() => screen = "cards"}/>
{:else if screen == 'cards'}
	<Cards blackCard="Who flooded the toilet" whiteCards={cards} {selected} {shown} />
{:else if screen == 'scores'}
	<Scores {players} />
{:else if screen == 'winner'}
	<Winner name={'You Ran'} on:restart={() => {cards = []; screen = "waiting";}} />
{/if}
