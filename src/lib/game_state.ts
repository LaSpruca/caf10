import { writable } from 'svelte/store';
import WsClient, { type Extras } from './ws_client';
import { browser } from '$app/environment';

type EventMap = {
	code: { code: string };
} & Extras;

const client = new WsClient<EventMap>();

export const ready = writable(false);
export const gameCode = writable('');

client.on('ready', () => {
	ready.set(true);
});

client.on('code', ({ code }) => {
	gameCode.set(code);
});

export const init = (code: string, name: string) => client.init(`game?code=${code}&name=${name}`);
