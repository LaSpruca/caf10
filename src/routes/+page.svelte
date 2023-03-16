<script lang="ts">
	import { goto } from '$app/navigation';

	const chooseGame = () => {
		let code = prompt('Please enter your game code');
		let nick = prompt('What is your nic name');

		goto(`/play/${code}/${nick}`);
	};
</script>

<h1 class="text-center text-6xl font-black text-white">Cards against Floor 10</h1>
<h2 class="text-center text-5xl font-bold text-gray-300">Don't worry about it</h2>

<div class="m-auto flex w-[40%]  content-center justify-around pt-28">
	<div class="w-50 flex h-20 items-center text-xl font-bold text-gray-400">
		<a href="/display">HOST</a>
	</div>
	<button on:click={chooseGame} class="w-50 h-20 text-xl font-bold text-gray-400">JOIN</button>
</div>
