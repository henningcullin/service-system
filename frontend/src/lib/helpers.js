import { account, tasks, machines, users} from './stores';
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
export function ƒ(selector) {
	return document.querySelector(selector);
}

/**
 * Loads the tasks from the server into the store
 */
export async function getTasks() {
	try {
		const response = await fetch('/api/auth/tasks');
		const data = await response.json();

		const formatted = data.map((t) => {
			const {id, title, description, task_type, status, archived, created, edited, creator, executor, machine} = t;
			return {
				id,
				title,
				description: description ? description : '',
				task_type,
				status,
				archived: Boolean(archived),
				created: new Date(created),
				edited: new Date(edited),
				creator,
				executor: executor ? executor : '',
				machine: machine ? machine : '',
			};
		})

		tasks.set(formatted);
	} catch (error) {
		console.error('Could not get tasks', error);
	}
};

/**
 * Loads the machines from the server into the store
 */
export async function getMachines() {
	try {
		const response = await fetch('/api/auth/machines');
		const data = await response.json();

		const formatted = data.map((m) => {
			const {id, name, make, machine_type, status, created, edited} = m;
			return {
				id,
				name,
				make: make ? make : '',
				type: machine_type ? machine_type : '',
				status,
				created: new Date(created),
				edited: new Date(edited),
			};
		})

		machines.set(formatted);
	} catch (error) {
		console.error('Could not get machines', error);
	}
};

/**
 * Loads the users from the server into the store
 */
export async function getUsers() {
	try {
		const response = await fetch('/api/auth/users');
		const data = await response.json();

		const formatted = data.map((u) => {
			const {id, first_name, last_name, email, phone, role, active, last_login} = u;
			return {
				id,
				first_name,
				last_name,
				email,
				phone: phone ? phone : '',
				role,
				active,
				last_login: new Date(last_login),
			};
		})

		users.set(formatted);
	} catch (error) {
		console.error('Could not fetch products', error);
	}
};