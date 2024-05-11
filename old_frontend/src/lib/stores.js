import { writable } from 'svelte/store';

export const machineTypes = writable([]);
export const machineStatuses = writable([]);

export const machines = writable([]);
export const machine = writable({});

export const users = writable([]);
export const user = writable({});

export const tasks = writable([]);
export const task = writable({});

export const reports = writable([]);
export const report = writable({});

export const account = writable({});
