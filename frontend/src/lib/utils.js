import { account } from '$stores';

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
