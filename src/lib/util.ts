import type { FadeParams } from 'svelte/transition';
import { cubicIn } from 'svelte/easing';

export const sleep: (ms: number) => Promise<void> = (ms) =>
	new Promise((resolve) => {
		setTimeout(resolve, ms);
	});

const fadeDuration = 300;

export const fadeSettings: { in: FadeParams; out: FadeParams } = {
	in: { duration: fadeDuration, delay: fadeDuration * 1.1, easing: cubicIn },
	out: { duration: fadeDuration, easing: cubicIn }
};
