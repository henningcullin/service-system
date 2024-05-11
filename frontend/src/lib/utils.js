export async function sendJSON(url, method, body) {
    return fetch(url, {
        headers: {
            'Content-Type': 'application/json',
        },
        method: method,
        body: JSON.stringify(body),
    });
}
