<script lang="ts">
	import { goto } from '$app/navigation';
	import { sleep } from '$lib/util';
	import CardCzar from './CardCzar.svelte';
	import Cards from './Cards.svelte';
	import { init } from '$lib/game_state';

	// export let game;
	// export let nicname;

	let blackCard = '_____ is my favourite activity';

	let cards = ['One', 'Two', 'Three', 'Four'];

	let score = 0;

	let screen:
		| 'czar wait'
		| 'czar'
		| 'wait'
		| 'cards'
		| 'placing'
		| 'czar wait 2'
		| 'czar 2'
		| 'cards 2' = 'czar wait';

	(async () => {
		await sleep(1500);
		screen = 'czar';
	})();
</script>

<div class="fixed left-[5vw] top-5 flex w-[90vw] justify-between font-bold text-gray-400">
	<!-- <p>
		{('' + game).substring(0, 3)}
		{('' + game).substring(3)}
	</p> -->
	<p>Score: {score}</p>
	<!-- <p>{nicname}</p> -->
</div>

<div class="pt-10">
	{#if screen == 'czar wait' || screen == 'czar'}
		<CardCzar
			{blackCard}
			cards={screen == 'czar' ? cards : undefined}
			on:selected={() => {
				(async () => {
					screen = 'wait';
					await sleep(1000);
					screen = 'czar wait 2';
					await sleep(2000);
					screen = 'czar 2';
				})();
			}}
		/>
	{:else if screen == 'czar wait 2' || screen == 'czar 2'}
		<CardCzar
			{blackCard}
			cards={screen == 'czar 2'
				? [
						['One', 'One B'],
						['Two', 'Two B'],
						['Three', 'Three B'],
						['Four', 'Four B']
				  ]
				: undefined}
			on:selected={() => {
				(async () => {
					screen = 'wait';
					await sleep(1000);
					screen = 'cards';
				})();
			}}
		/>
	{:else if screen == 'cards'}
		<Cards
			{blackCard}
			{cards}
			on:selected={() => {
				(async () => {
					screen = 'wait';
					await sleep(1000);
					screen = 'cards 2';
				})();
			}}
		/>
	{:else if screen == 'cards 2'}
		<Cards
			two={true}
			{blackCard}
			{cards}
			on:selected={() => {
				(async () => {
					screen = 'placing';
					await sleep(1000);
				})();
			}}
		/>
	{/if}
</div>
