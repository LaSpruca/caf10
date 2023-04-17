import { writable } from 'svelte/store';
import WsClient, { type Extras } from './ws_client';

type EventMap = {
	code: { code: string };
	available_packs: { packs: string[] };
	decks: { decks: string[] };
	playerJoin: { name: string };
	playerLeave: { name: string };
} & Extras;

const client = new WsClient<EventMap>();
const states = ['no_conn', 'waiting', 'select_packs'] as const;
type State = (typeof states)[number];

export const state = writable<State>('no_conn');
export const gameCode = writable('');
export const packs = writable<string[]>([]);
export const players = writable<string[]>([]);
export const decks = writable<string[]>([]);

client.on('ready', () => {
	state.set('waiting');
});

client.on('code', ({ code }) => {
	gameCode.set(code);
});

client.on('available_packs', ({ packs: availablePacks }) => {
	packs.set(availablePacks);
});

client.on('playerJoin', ({ name }) => {
	players.update((players) => {
		players.push(name);
		return players;
	});
});

client.on('playerLeave', ({ name }) => {
	players.update((players) => {
		const index = players.indexOf(name);
		if (index !== -1) {
			players.splice(index, 1);
		}
		return players;
	});
});

export const init = () => client.init('display');
