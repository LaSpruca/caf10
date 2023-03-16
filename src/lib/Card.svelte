<script lang="ts">
	export let content = '';
	export let color: 'black' | 'white' = 'white';
	export let shown = true;
	export let up = false;
	export let big = false;
</script>

<div
	class="relative h-64 w-52 animate-slide-up rounded-xl shadow-xl transition-all duration-200"
	class:bg-gray-900={color == 'black'}
	class:text-gray-100={color == 'black'}
	class:bg-gray-100={color == 'white'}
	class:translate-y-[-20%]={up}
	class:scale-105={big}
>
	<div
		class="px-5 py-7 text-left text-lg font-black transition-[opacity] duration-200"
		class:opacity-0={!shown}
	>
		{content}
	</div>
</div>
