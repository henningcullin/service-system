<script>

    import { navigate } from 'svelte-navigator';
    import { getLoggedIn } from '$lib/utils';
    import { ƒ } from '$lib/utils';

    let email = '';
    let type = '';
    let password = '';
    let code = '';


    async function submitEmailForm() {

        type = '';
        password = '';
        code = '';

        if (email.length <= 0) return;

        const response = await fetch('/api/user/login', {
            headers: {
                'Content-Type':'application/json'
            },
            method: 'POST',
            body: JSON.stringify({email})
        });

        console.log(response);

        const data = await response.json();
        
        if (response.status != 200) return;

        const codeField = ƒ('#code-field');
        const passwordField = ƒ('#password-field');

        if (!codeField || !passwordField) return console.error('Fatal rendering error');

        if (data.message == 'code') {
            codeField.classList.remove('hidden');
            passwordField.classList.add('hidden');
            type = 'code';
        }
        else {
            codeField.classList.add('hidden');
            passwordField.classList.remove('hidden');
            type = 'password';
        }

        ƒ('#emailForm')?.classList.add('hidden');
        ƒ('#loginForm')?.classList.remove('hidden');

    }

    async function submitLoginForm() {

        /**
         * 
         * @param {string} endpoint 
         * @param {object} value
         */
        async function loginFetch(endpoint, value) {
            return fetch(endpoint, {
                headers: {
                    'Content-Type':'application/json'
                },
                method: 'POST',
                body: JSON.stringify(value)
            });
        }

        const response = type === 'password'
            ? await loginFetch('/api/user/internal/login', {email, password})
            : type === 'code'
                ? await loginFetch('/api/user/external/verify', {code})
                : console.error('Unknown login type');


        if (!response) return;

        const data = await response.json();

        if (response.status != 200) return alert(data.message);

        getLoggedIn();

        navigate('/');
    }

</script>

<div class='segment'>
    <h2>Log in</h2>
    
    <form id='emailForm' on:submit|preventDefault={submitEmailForm}>
        <input type='email' placeholder='email' bind:value={email}><br>
        <input type='submit' value='Send'>
    </form>

    <form id='loginForm' class='hidden' on:submit|preventDefault={submitLoginForm}>
        <input type='email' readonly bind:value={email}><br>
        <input type='text' placeholder='code' id='code-field' class='hidden' bind:value={code}><br>
        <input type='password' placeholder='password' id='password-field' bind:value={password}><br>
        <input type='submit' value='Login'>
    </form>
</div>

<style>

</style>