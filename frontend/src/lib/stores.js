import { writable } from 'svelte/store';

export const SidebarOpen = writable(false);

export const account = writable({});

export const machines = writable([]);
export const machine = writable({});

export const machineTypes = writable([]);
export const machineType = writable({});

export const machineStatuses = writable([]);
export const machineStatus = writable({});

export const facilities = writable([]);
export const facility = writable({});

export const users = writable([]);
export const user = writable({});

export const roles = writable([]);
export const role = writable({});

export const reports = writable([]);
export const report = writable({});

export const reportTypes = writable([]);
export const reportType = writable({});

export const reportStatuses = writable([]);
export const reportStatus = writable({});

export const tasks = writable([]);
export const task = writable({});

export const taskTypes = writable([]);
export const taskType = writable({});

export const taskStauses = writable([]);
export const taskStatus = writable({});