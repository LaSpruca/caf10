import { writable } from 'svelte/store';
import WsClient, { type Extras } from './ws_client';

const states = ['join', 'join-wait', 'waiting'] as const;
type State = (typeof states)[number];

type EventMap = {
	joinSuccess: { name: string; game_code: string };
	noGameCode: {};
	nameTaken: {};
} & Extras;

const client = new WsClient<EventMap>();

export const state = writable<State>('join');
export const gameCode = writable('');
export const name = writable('');
export const score = writable(0);
export const error = writable('');

client.on('joinSuccess', ({ name: newName, game_code: newGameCode }) => {
	name.set(newName);
	gameCode.set(newGameCode);
	state.set('waiting');
	error.set('');
});

client.on('noGameCode', () => {
	error.set('There is no game with that game code');
	state.set('join');
});

client.on('nameTaken', () => {
	error.set('That name is already taken');
	state.set('join');
});

export const init = (code: string, name: string) => {
	console.log('Initializing state');
	state.set('join-wait');
	client.init(`game?code=${code}&name=${name}`);
};
