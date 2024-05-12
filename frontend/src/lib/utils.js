import { account, facilities, machine, machineStatuses, machineTypes, machines } from '$stores';

export async function sendJSON(url, method, body) {
    return fetch(url, {
        headers: {
            'Content-Type': 'application/json',
        },
        method: method,
        body: JSON.stringify(body),
    });
}

export async function getLoggedIn() {
    try {
        const response = await fetch('/api/auth/me');
        if (response.status === 200) {
            const data = await response.json();
            account.set(data);
            return true;
        } else return false;
    } catch (error) {
        return false;
    }
}

async function fetchJson(url) {
    const response = await fetch(url);
    return response.json();
}

export async function getMachines() {
    try {
        const data = await fetchJson('/api/auth/machines');
        machines.set(data);
    } catch (error) {
        console.error(error);
    }
}

export async function getMachine(id) {
    try {
        const data = await fetchJson(`/api/auth/machine?id=${id}`);
        machine.set(data);
    } catch (error) {
        console.error(error);
    }
}

export async function getMachineTypes() {
    try {
        const data = await fetchJson('/api/auth/machine_types');
        machineTypes.set(data);
    } catch (error) {
        console.error(error);
    }
}

export async function getMachineStatuses() {
    try {
        const data = await fetchJson('/api/auth/machine_statuses');
        machineStatuses.set(data);
    } catch (error) {
        console.error(error);
    }
}

export async function getFacilities() {
    try {
        const data = await fetchJson('/api/auth/facilities');
        facilities.set(data);
    } catch (error) {
        console.error(error);
    }
}
