<script lang="ts">
	import { cubicOut } from 'svelte/easing';
	import { tweened } from 'svelte/motion';

	export let player: string;
	export let score: number;

	const percentage = tweened(0, {
		duration: 500,
		easing: cubicOut
	});

	setTimeout(() => {
		percentage.set(score * 10);
	}, 2000);
</script>

<div class="w-full">
	<div class="text-3xl font-bold text-gray-300 text-center">
		{player}
	</div>
	<div class="relative">
		<div class="absolute left-0 top-[-5%] h-11 w-[1px] bg-gray-400" />
		<div class="absolute left-[10%] top-[-5%] h-11 w-[1px] bg-gray-400" />
		<div class="absolute left-[20%] top-[-5%] h-11 w-[1px] bg-gray-400" />
		<div class="absolute left-[30%] top-[-5%] h-11 w-[1px] bg-gray-400" />
		<div class="absolute left-[40%] top-[-5%] h-11 w-[1px] bg-gray-400" />
		<div class="absolute left-[50%] top-[-5%] h-11 w-[1px] bg-gray-400" />
		<div class="absolute left-[60%] top-[-5%] h-11 w-[1px] bg-gray-400" />
		<div class="absolute left-[70%] top-[-5%] h-11 w-[1px] bg-gray-400" />
		<div class="absolute left-[80%] top-[-5%] h-11 w-[1px] bg-gray-400" />
		<div class="absolute left-[90%] top-[-5%] h-11 w-[1px] bg-gray-400" />
		<div class="absolute left-[100%] top-[-5%] h-11 w-[1px] bg-gray-400" />

		<div class="bg-gray-200 h-10" style="width: {$percentage}%;" />
	</div>
</div>
