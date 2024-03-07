import { account } from './stores';
import { navigate } from 'svelte-navigator';

/**
 * Sets the account writable to the currently logged in user
 * @returns {Promise<void>}
 */
export async function getUser() {
  try {
    const response = await fetch('/api/auth/user/me');
    const data = await response.json();

    if (response.status != 200) return navigate('/login');

    // @ts-ignore
    data.last_login = new Date(data.last_login);

    account.set(data);
  } catch (error) {
    console.log('Could not fetch products', error);
  }
}

/**
 * Shorthand for document.querySelector
 * @param {string} selector 
 * @returns {Element | null}
 */
export function el(selector) {
  return document.querySelector(selector);
}