<script lang="ts">
	import { state, gameCode, name, score, error, init as initWs } from '$lib/game_state';
	let gameCodeInput = '';
	let nameInput = '';
</script>

{#if $state.startsWith('join')}
	<div class="grid grid-cols-2 items-center gap-2">
		<label for="game_code" class="justify-self-end text-lg font-bold text-white">Game code</label>
		<input
			class="w-[50%] p-1 text-gray-800"
			type="text"
			id="game_code"
			placeholder="Game Code"
			bind:value={gameCodeInput}
		/>
		<label for="name" class="justify-self-end text-lg font-bold text-white">Name</label>
		<input
			class="w-[50%] p-1 text-gray-800"
			type="text"
			id="name"
			placeholder="My Super Cool username"
			bind:value={nameInput}
		/>
		<button
			class="col-span-2 self-center bg-white p-1 font-bold transition-all disabled:bg-gray-400 disabled:text-gray-700"
			disabled={!gameCodeInput || !nameInput || $state != 'join'}
			on:click={() => initWs(gameCodeInput, nameInput)}>Join</button
		>
		{#if $error != ''}
			<p class="col-span-full text-center font-bold text-red-500">{$error}</p>
		{/if}
	</div>
{:else}
	<div class="fixed left-[5vw] top-5 flex w-[90vw] justify-between font-bold text-gray-400">
		<p>
			{('' + $gameCode).substring(0, 3)}
			{('' + $gameCode).substring(3)}
		</p>

		<p>Score: {$score}</p>
		<p>{$name}</p>
	</div>
{/if}
