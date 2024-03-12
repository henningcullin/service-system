<script>
    // @ts-nocheck
    import { onMount } from 'svelte';
    import { el, sendDelete, sendJson } from '../../lib/helpers';

    import { account, user, users } from '../../lib/stores';
    import { navigate } from 'svelte-navigator';
    
    resetUser();

    const urlParams = new URLSearchParams(window.location.search);
    
    const state = {
        edit: false,
        new: false
    };

    let id = urlParams.get('id');
    
    if (Boolean(urlParams.get('new'))) setState('new', true);
    if (Boolean(urlParams.get('edit'))) setState('edit', true);
    

    if (!state.new && (!id || id.length != 36)) navigate('/notfound');

    if ((state.new || state.edit) && ($account['role'] == 'Worker' || $account == {})) window.history.back();  
    
    if(id) getUser();

    onMount(() => {
        computeState();
    })

    /**
     * true to turn on false to turn off
     * @param {bool} state
     */
     function computeState() {
        const combined = Boolean((state.new || state.edit));

        if (!el('#first_name')) return;

        el('#first_name').disabled = !combined;
        el('#last_name').disabled = !combined;
        el('#email').disabled = !combined;
        el('#phone').disabled = !combined;
        el('#role').disabled = !combined;
        el('#active').disabled = !combined;
        el('#save').disabled = !combined;

        el('#new').disabled = state.new;
        el('#edit').disabled = (state.new || state.edit);
        el('#cancel').disabled = !(state.new || state.edit)
    }

    function setState(prop, newState) {
        if (newState != true && newState != false) return;

        const path = location.pathname+location.search;

        switch (prop) {
            case 'new':
                state.new = newState;
                state.edit = false;
                resetUser();
                if (path !== '/user?new=true') navigate('/user?new=true');
                break;
            case 'edit':
                state.new = false;
                state.edit = newState;
                if (path !== `/user?id=${id}&edit=true`) navigate(`/user?id=${id}&edit=true`);
                break;
            default:
                state.new = false;
                state.edit = false;
                if (id && path !== `/user?id=${id}`) navigate(`/user?id=${id}`)
                else return navigate('/users/');
                getUser();
                break;
        };
        computeState();
    }

    function resetUser() {
        user.set({
            id: '',
            first_name: '',
            last_name: '',
            email: '',
            phone: '',
            role: '',
            active: '',
            last_login: '',
        });
    }

    async function getUser() {
        try {
            const response = await fetch(`/api/auth/user?id=${id}`);

            if (response.status != 200) navigate('/notfound');

            const data = await response.json();

            user.set({
                id: data.id,
                first_name: data.first_name,
                last_name: data.last_name,
                email: data.email,
                phone: data.phone,
                role: data.role,
                active: data.active,
                last_login: new Date(data.last_login),
            });

        } catch (error) {
            console.error(error)
        }
    }

    function handleSubmit() {
        switch (true) {
            case (state.new):
                createUser();
                break;
            case (state.edit):
                updateUser();
                break;
        };
    }

    async function createUser() {
        try {

            if (!state.new  || ($user.role != 'Worker' && $user.role != 'Basic' && $user.role != 'Administrator') || $user.role == 'Super') return;
            
            const response = await sendJson('/api/auth/user', 'POST', {
                first_name: $user.first_name,
                last_name: $user.last_name,
                email: $user.email,
                phone: $user.phone,
                role: $user.role,
                active: $user.active,
            });
    
            const data = await response.json();
    
            if (response.status != 201) return alert(data.message);
    
            id = data.id;
    
            setState('edit', true);
    
            $user.id = data.id;
            $user.last_login = new Date().toLocaleString('en-GB');

        } catch (error) {
            console.error('createUser error' + error);
        }
    }

    async function updateUser() {
        try {

            const response = await sendJson('/api/auth/user', 'PUT', {
                id: $user.id,
                first_name: $user.first_name,
                last_name: $user.last_name,
                email: $user.email,
                phone: $user.phone,
                role: $user.role,
                active: $user.active,
            });

            if (response.status != 204) {
                const data = await response.json();
                return alert(data.message);
            }

            setState('view', true);

        } catch (error) {
            console.error(error);
        }
    }

</script>

<div class='segment'>

    <div class="menu">
        <button id='new' on:click={() => {setState('new', true)}}>New</button>
        <button id='edit' on:click={() => {setState('edit', true)}}>Edit</button>
        <button id='cancel' on:click={() => {setState('cancel', true)}}>Cancel</button>
    </div>
    
    <form on:submit|preventDefault={handleSubmit}>
        <label for='id'>ID</label>
        <input id='id' type='text' bind:value={$user.id} disabled readonly>

        <label for='first_name'>Name</label>
        <input id='first_name' type='text' bind:value={$user.first_name} disabled>
        
        <label for='last_name'>Make</label>
        <input id='last_name' type='text' bind:value={$user.last_name} disabled>
        
        <label for='email'>Type</label>
        <input id='email' type='text' bind:value={$user.email} disabled>

        <label for='phone'>Phone</label>
        <input id='phone' type='text' bind:value={$user.phone} disabled>

        <label for='role'>Role</label>
        <select id='role' bind:value={$user.role} disabled>
            <option value='Worker'>Worker</option>
            <option value='Basic'>Basic</option>
            <option value='Administrator'>Administrator</option>
            <option value='Super'>Super</option>
        </select>
        
        <label for='active'>Active</label>
        <input id='active' type='text' bind:value={$user.active} disabled readonly>
        
        <label for='last_login'>Last Login</label>
        <input id='last_login' type='text' bind:value={$user.last_login} disabled readonly>

        <input id='save' type="submit" value="Save" disabled>

    </form>

</div>

<style>

    label {
        font-size:1.1em;
        text-align: left;
    }

    input, select {
        margin-bottom: 15px;
        font-size:1em;
        line-height: 1.5em;
    }

    form {
        margin-top:15px;
    }

    .menu>button {
        flex:1;
        height:100%;
        width:100%;
        display:grid;
        place-items: center;
        background-color: #242424;
    }

    .menu>button:hover {
        background-color: #343434;
    }

    #save {
        margin-top:15px;
    }

    @media (min-width: 1200px) {

        .menu>button{
            font-size: 1.5em;
        }

        form {
            margin-top: 50px;
            display:grid;
            grid-template-columns: 1fr 3fr 1fr 3fr;
            grid-template-rows: 1fr 1fr 1fr 1fr;
            gap: 35px;
        }

        form input, form select {
            width:100%;
            font-size: 1.4em;
        }

        form label {
            font-size: 1.4em;
        }

    }

</style>