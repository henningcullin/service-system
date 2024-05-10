import { account } from "./stores";

/**
 * 
 * @param {{}} object 
 * @returns {Promise}
 */
export async function postJSON(url, object) {
    return fetch(url, {
        method: "POST",
        body: JSON.stringify(object),
        headers: {
            "Content-Type": "application/json",
        },
    });
}
export async function getloggedIn() {
    try {
        const response = await fetch("/api/auth/me");
        const data = await response.json();

        account.set(data);
    } catch (error) {
        
    }
}