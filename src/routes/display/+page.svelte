<script lang="ts">
	import { init, gameCode, state, players, packs } from '$lib/display_state';
	import { onMount } from 'svelte';
	import Waiting from './Waiting.svelte';

	onMount(() => {
		init();
	});
</script>

<p class="fixed text-xl font-bold text-gray-500">
	Game Code: {('' + $gameCode).substring(0, 3)}
	{('' + $gameCode).substring(3)}
</p>

{#if $state == 'select_packs'}
	<Waiting players={$players} packs={$packs} />
{/if}
