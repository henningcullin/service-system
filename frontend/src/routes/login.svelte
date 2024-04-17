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

<div class="ui middle aligned center aligned grid">
    <div class="column">

        <h2 class="ui huge centered header">Log in</h2>

        <form id='emailForm' class="ui large form {type === '' ? '' : 'hidden'}" on:submit|preventDefault={submitEmailForm}>
            <div class="ui stacked">
                <div class="field"><input type='email' placeholder='email' bind:value={email}></div>
                <input class="ui fluid large olive submit button" type='submit' value='Send'>
            </div>
        </form>
        
        <form id='loginForm' class='ui large form {type !== '' ? '' : 'hidden'}' on:submit|preventDefault={submitLoginForm}>
            <div class="ui stacked">
                <div class="field"><input class="ui input" type='email' readonly bind:value={email}></div>
                <div class="field"><input type='text' placeholder='code' id='code-field' class='hidden' bind:value={code}></div>
                <div class="field"><input type='password' placeholder='password' id='password-field' bind:value={password}></div>
                <input class="ui fluid large olive submit button" type='submit' value='Login'>
            </div>
        </form>
    </div>
</div>


<style>

    .ui.middle.aligned.center.aligned.grid {
        min-height: 100dvh;
    }

    .column {
        max-width: 55em;
    }
</style>