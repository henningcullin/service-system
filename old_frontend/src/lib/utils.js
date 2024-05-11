import { account, tasks, reports, machines, users, machineStatuses, machineTypes } from './stores';
import { navigate } from 'svelte-navigator';

/**
 * Sets the account writable to the currently logged in user
 * @returns {Promise<void>}
 */
export async function getLoggedIn() {
    try {
        const response = await fetch('/api/auth/me');
        const data = await response.json();

        if (response.status !== 200) {
            account.set({});
            return navigate('/login');
        }

        data.last_login = new Date(data.last_login);

        account.set(data);
        return true;
    } catch (error) {
        console.log('Could not get logged in status', error);
        return false;
    }
}

/**
 * Sends json data
 * @param {string} url endpoint
 * @param {string} method http method
 * @param {Array<any> | Object} body data to be sent
 * @returns {Promise<any>}
 */
export async function sendJson(url, method, body) {
    return fetch(url, {
        headers: {
            'Content-Type': 'application/json',
        },
        method: method,
        body: JSON.stringify(body),
    });
}

/**
 * Sends delete requests
 * @param {string} url endpoint
 * @returns {Promise<any>}
 */
export async function sendDelete(url) {
    return fetch(url, { method: 'DELETE' });
}

/**
 * Shorthand for document.querySelector
 * @param {string} selector
 * @returns {Element | null}
 */
export function ƒ(selector) {
    return document.querySelector(selector);
}

async function fetchAll(url) {
    const response = await fetch(url);
    return response.json();
}

// _______________________________________ TASKS

export async function getTasks() {
    try {
        const data = await fetchAll('/api/auth/tasks');
        tasks.set(data);
    } catch (error) {
        console.error('Could not get tasks', error);
    }
}

// ______________________________________ REPORTS

export async function getReports() {
    try {
        const data = await fetchAll('/api/auth/reports');
        reports.set(data);
    } catch (error) {
        console.error('Could not get reports', error);
    }
}

// _________________________________________ MACHINES

export async function getMachines() {
    try {
        const data = await fetchAll('/api/auth/machines');
        machines.set(data);
    } catch (error) {
        console.error('Could not get machines', error);
    }
}

export async function getMachineStatuses() {
    try {
        const data = await fetchAll('/api/auth/machine_statuses');
        machineStatuses.set(data);
    } catch (error) {
        console.error('Could not get machinestatuses', error);
    }
}

export async function getMachineTypes() {
    try {
        const data = await fetchAll('/api/auth/machine_types');
        machineTypes.set(data);
    } catch (error) {
        console.error('Could not get machinetypes', error);
    }
}

// ____________________________________________ USERS

export async function getUsers() {
    try {
        const data = await indexFetch('/api/auth/users');
        users.set(data);
    } catch (error) {
        console.error('Could not fetch products', error);
    }
}
