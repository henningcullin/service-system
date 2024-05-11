<script>
    import { getLoggedIn, sendJSON } from '$utils';
    import { navigate } from 'svelte-navigator';

    let processing = false;
    let type = 'email';
    let email = '';
    let password = '';
    let otp = '';

    async function submitForm() {
        if (processing) return;
        processing = true;
        await submit(type);
        processing = false;
    }

    async function submit(type) {
        switch (type) {
            case 'email':
                return await submitEmail();
            case 'password':
                return await submitPassword();
            case 'otp':
                return await submitOtp();
        }
    }

    async function submitEmail() {
        try {
            const response = await sendJSON('/api/login', 'POST', { email });
            const data = await response.json();
            if (response.status === 200) {
                type = data.toLowerCase();
            }
        } catch (error) {
            console.error(error);
        }
    }

    async function submitPassword() {
        try {
            const response = await sendJSON('/api/login/password', 'POST', { email, password });
            console.log(response);
            if (response.status === 200) {
                const loggedIn = await getLoggedIn();
                if (loggedIn) return navigate('/');
            }
        } catch (error) {
            console.error(error);
        }
    }

    async function submitOtp() {
        try {
            const response = await sendJSON('/api/login/otp', 'POST', { code: otp });
            if (response.status === 200) {
                const loggedIn = await getLoggedIn();
                if (loggedIn) return navigate('/');
            }
        } catch (error) {
            console.error(error);
        }
    }
</script>

<segment>
    <form on:submit|preventDefault={submitForm}>
        <input
            type="email"
            placeholder="Email"
            required
            bind:value={email}
            readonly={type !== 'email'}
            disabled={type !== 'email'}
        />
        <input
            type="password"
            placeholder="Password"
            bind:value={password}
            class={type !== 'password' ? 'hidden' : ''}
        />
        <input type="text" placeholder="One Time Password" bind:value={otp} class={type !== 'otp' ? 'hidden' : ''} />
        <button class="teal {processing ? 'disabled' : ''}">{type === 'email' ? 'Send' : 'Login'}</button>
    </form>
</segment>

<style>
    form {
        width: 34vw;
        margin-left: 31vw;
        margin-top: 25dvh;
        background-color: var(--secondary);
        padding: 4%;
        border-radius: 7.5px;
        display: grid;
        place-items: center;
    }

    input {
        margin-top: 5px;
    }

    button {
        width: 10em;
        height: 3em;
        margin-top: 1em;
    }
</style>
