<script lang="ts">
	export let content = '';

	export let color: 'black' | 'white' = 'white';

	export let shown = true;

	export let up = false;
</script>

<div
	class="w-52 h-64 rounded-xl shadow-xl animate-slide-up transition-all duration-200"
	class:bg-gray-900={color == 'black'}
	class:text-gray-100={color == 'black'}
	class:bg-gray-100={color == 'white'}
	class:translate-y-[-20%]={up}
>
	<div
		class="px-5 py-7 text-lg font-black transition-[opacity] duration-200"
		class:opacity-0={!shown}
	>
		{content}
	</div>
</div>
