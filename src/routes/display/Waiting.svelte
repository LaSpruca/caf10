<script lang="ts">
	import { createEventDispatcher } from "svelte";
	import { cubicIn } from "svelte/easing";
    import { fade, fly } from "svelte/transition";
    
    export let players: string[];

    let dispatch = createEventDispatcher<{start: void}>();
</script>

<div transition:fade="{{duration: 1000, easing: cubicIn}}">
    {#each players as player}
        <div transition:fly="{{x: -10}}">
            {player}
        </div>
    {/each}

    <div class="flex items-center justify-center w-full p-10">
		<button on:click={() => dispatch('start')} class="bg-white rounded p-1 shadow-sm">
			Start
		</button>
	</div>
</div>
