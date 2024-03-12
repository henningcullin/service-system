import { writable } from 'svelte/store';

export const machines = writable([]);
export const machine = writable({});

export const users = writable([]);
export const user = writable({});

export const account = writable({});