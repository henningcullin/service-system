import { account } from './stores';
import { navigate } from 'svelte-navigator';

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