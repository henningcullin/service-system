import { account } from './stores';
import { navigate } from 'svelte-navigator';

/**
 * Sets the account writable to the currently logged in user
 * @returns {Promise<void>}
 */
export async function getLoggedIn() {
	try {
		const response = await fetch('/api/auth/user/me');
		const data = await response.json();

		if (response.status !== 200) return navigate('/login');

		data.last_login = new Date(data.last_login);

		account.set(data);
	} catch (error) {
		console.log('Could not fetch products', error);
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
export function el(selector) {
	return document.querySelector(selector);
}
