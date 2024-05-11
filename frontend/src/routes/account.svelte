<script>
    import { getLoggedIn } from '$lib/utils';

    async function logout() {
        try {
            const response = await fetch('/api/auth/logout');

            if (response.status !== 200) return alert('Could not logout');

            getLoggedIn();
        } catch (error) {
            console.error('Logout error: ', error);
            alert('Could not logout');
        }
    }
</script>

<button class="ui large red button" on:click={logout}>Logout</button>
