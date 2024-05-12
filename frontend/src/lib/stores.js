import { writable } from 'svelte/store';

export const SidebarOpen = writable(false);

export const account = writable({});

export const machines = writable([]);
export const machine = writable({});
export const machineTypes = writable([]);
export const machineStatuses = writable([]);

export const facilities = writable([]);
