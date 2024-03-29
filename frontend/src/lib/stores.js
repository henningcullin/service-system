import { writable } from 'svelte/store';

export const machines = writable(new Map());
export const machine = writable({});

export const users = writable(new Map());
export const user = writable({});

export const tasks = writable(new Map());
export const task = writable({});

export const account = writable({});
