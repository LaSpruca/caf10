export const sleep: (ms: number) => Promise<void> = (ms) =>
	new Promise((resolve) => {
		setTimeout(resolve, ms);
	});
