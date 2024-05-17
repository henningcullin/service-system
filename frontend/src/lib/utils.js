import {
    account,
    facilities,
    machine,
    machineStatuses,
    machineTypes,
    machines,
    tasks,
    taskStatuses,
    taskTypes,
    reports,
    reportStatuses,
    reportTypes,
} from '$stores';

export async function sendJSON(url, method, body) {
    return fetch(url, {
        headers: {
            'Content-Type': 'application/json',
        },
        method: method,
        body: JSON.stringify(body),
    });
}

export async function sendDelete(url) {
    return fetch(url, { method: 'DELETE' });
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
    try {
        const response = await fetch(url);
        return response.json();
    } catch (error) {
        console.error(error);
    }
}

export async function getMachines() {
    const data = await fetchJson('/api/auth/machines');
    machines.set(data ?? []);
}

export async function getMachine(id) {
    const data = await fetchJson(`/api/auth/machine?id=${id}`);
    machine.set(data ?? {});
}

export async function getMachineTypes() {
    const data = await fetchJson('/api/auth/machine_types');
    machineTypes.set(data ?? []);
}

export async function getMachineStatuses() {
    const data = await fetchJson('/api/auth/machine_statuses');
    machineStatuses.set(data ?? []);
}

export async function getFacilities() {
    const data = await fetchJson('/api/auth/facilities');
    facilities.set(data ?? []);
}

export async function getTasks() {
    const data = await fetchJson('/api/auth/tasks');
    tasks.set(data ?? []);
}

export async function getOneTask(id) {
    const data = await fetchJson(`/api/auth/task?id=${id}`);
    task.set(data ?? {});
}

export async function getTaskTypes() {
    const data = await fetchJson('/api/auth/task_types');
    taskTypes.set(data ?? []);
}

export async function getTaskStatuses() {
    const data = await fetchJson('/api/auth/task_statuses');
    taskStatuses.set(data ?? []);
}

export async function getReports() {
    const data = await fetchJson('/api/auth/reports');
    reports.set(data ?? []);
}

export async function getReportTypes() {
    const data = await fetchJson('/api/auth/report_types');
    reportTypes.set(data ?? []);
}

export async function getReportStatuses() {
    const data = await fetchJson('/api/auth/report_statuses');
    reportStatuses.set(data ?? []);
}
