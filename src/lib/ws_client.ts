import { PUBLIC_BACKEND_URL as BACKEND_URL } from '$env/static/public';

export type Extras = { error?: Event; ready?: {}; close: CloseEvent };

export default class WsClient<T extends Extras> {
	// #client?: WebSocket;
	#ready: boolean;
	#events: {
		[V in keyof T]?: (event: T[V] & { type: V }) => unknown;
	};

	constructor() {
		this.#ready = false;
		this.#events = {};
	}

	init() {
		console.log(this);
		let client = new WebSocket(`${BACKEND_URL.replace('http', 'ws')}/display`);
		client.addEventListener('open', () => {
			console.log('Socket opened');
			this.#ready = true;
			this.#events['ready']?.({ type: 'ready' });
		});

		client.addEventListener('error', (e) => {
			console.error('Socket error', e);
			this.#ready = false;
			this.#events['error']?.({ ...e, type: 'error' });
		});

		client.addEventListener('close', (e) => {
			console.log('Socket closed', e);
			this.#ready = false;
			this.#events['close']?.({ ...e, type: 'close' });
		});

		client.addEventListener('message', <V extends keyof T>(e: MessageEvent) => {
			const data = JSON.parse(e.data) as { type: V } & T[V];
			this.#events[data.type]?.(data);
		});
	}

	on<V extends keyof T>(event: V, handler: (event: T[V] & { type: V }) => unknown) {
		this.#events[event] = handler as (event: T[keyof T]) => unknown;
	}
}
