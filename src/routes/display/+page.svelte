<script lang="ts">
	import { init, gameCode, state } from '$lib/display_state';
	import { onMount } from 'svelte';

	onMount(() => {
		init();
	});
</script>

<p class="fixed text-xl font-bold text-gray-500">
	Game Code: {('' + $gameCode).substring(0, 3)}
	{('' + $gameCode).substring(3)}
</p>

{#if $state == 'select_packs'}{/if}
