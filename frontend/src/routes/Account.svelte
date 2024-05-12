<script>
    import { account } from '$stores';
    import { navigate } from 'svelte-navigator';

    async function logout() {
        const response = await fetch('/api/auth/logout');
        if (response.status !== 200) return alert('Failed to log out');
        account.set({});
        navigate('/login');
    }
</script>

<button class="red" on:click={logout}> Logout </button>
