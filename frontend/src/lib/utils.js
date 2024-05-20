import {
    account,
    facilities,
    machine,
    machineStatuses,
    machineTypes,
    machines,
    task,
    tasks,
    taskStatuses,
    taskTypes,
    report,
    reports,
    reportStatuses,
    reportTypes,
    user,
    users,
    roles,
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
    const data = await fetchJson(`/api/auth/task?task_id=${id}`);
    if (data instanceof Array && data?.length === 1) {
        task.set(data[0]);
    } else task.set({});
}

export async function getTaskTypes() {
    const data = await fetchJson('/api/auth/task_types');
    taskTypes.set(data ?? []);
}

export async function getTaskStatuses() {
    const data = await fetchJson('/api/auth/task_statuses');
    taskStatuses.set(data ?? []);
}

export async function getOneReport(id) {
    const data = await fetchJson(`/api/auth/report?report_id=${id}`);
    if (data instanceof Array && data?.length === 1) {
        report.set(data[0]);
    } else report.set({});
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

export async function getUsers() {
    const data = await fetchJson('/api/auth/users');
    users.set(data ?? []);
}

export async function getUser(id) {
    const data = await fetchJson(`/api/auth/user?id=${id}`);
    user.set(data ?? {});
}

export async function getRoles() {
    const data = await fetchJson('/api/auth/roles');
    roles.set(data ?? []);
}

export async function getMyReports(id) {
    const data = await fetchJson(`/api/auth/report?creator_id=${id}`);
    reports.set(data ?? []);
}

export async function getTasksToExecute(id) {
    const data = await fetchJson(`/api/auth/task?executor_id=${id}`);
    tasks.set(data ?? []);
}
