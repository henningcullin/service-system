<script>
    // @ts-nocheck
    import { sendJson } from '$lib/utils';
    import { account, user, users } from '$lib/stores';
    import { navigate } from 'svelte-navigator';
    
    const state = {
        edit: false,
        new: false
    };

    let currentUser = emptyUser();

    user.set(emptyUser());

    const urlParams = new URLSearchParams(window.location.search);

    let id = urlParams.get('id');
    
    if (Boolean(urlParams.get('new'))) setState('new');
    if (Boolean(urlParams.get('edit'))) setState('edit');
    
    if (!state.new && (!id || id.length !== 36)) navigate('/notfound');
    else if (!state.new) getUser();

    if ((state.new || state.edit) && ($account['role'] === 'Worker' || $account === {})) window.history.back();  

    function setState(prop) {
        const path = location.pathname+location.search;

        switch (prop) {
            case 'new':
                state.new = true;
                state.edit = false;
                user.set(emptyUser());
                if (path !== '/user?new=true') navigate('/user?new=true');
                break;
            case 'edit':
                state.new = false;
                state.edit = true;
                if (path !== `/user?id=${id}&edit=true`) navigate(`/user?id=${id}&edit=true`);
                break;
            default:
                state.new = false;
                state.edit = false;
                if (id && path !== `/user?id=${id}`) navigate(`/user?id=${id}`)
                else return navigate('/users/');
                resetUser();
                break;
        };
    }

    function emptyUser() {
        return {
            id: '',
            first_name: '',
            last_name: '',
            email: '',
            password: '',
            phone: '',
            role: 'Worker',
            active: false,
            last_login: '',
        };
    }

    function resetUser() {
        user.set({...currentUser});
    }

    function setUser(newData) {

        const {id, first_name, last_name, email, phone, role, active, last_login} = newData;

        currentUser = {
            id,
            first_name,
            last_name,
            email,
            password: '',
            phone,
            role,
            active: Boolean(active),
            last_login: last_login ? new Date(last_login) : '',
        };

        resetUser();

    }

    async function getUser() {
        try {
            const response = await fetch(`/api/auth/user?id=${id}`);
            if (response.status !== 200) navigate('/notfound');

            const data = await response.json();
            setUser(data);

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

            if (!state.new  || $user.role === 'Super') return;

            const newUser = Object.fromEntries(
                Object.entries($user).filter(([key, value]) => {
                    if (key === 'active') return true
                    else return !value !== true
                })
            );

            const response = await sendJson('/api/auth/user', 'POST', newUser);
            const data = await response.json();

            if (response.status != 201) return alert(data.message);
            id = data.id;
    
            setUser(data);
            setState('edit');

        } catch (error) {
            alert('Could not create the user');
            console.error('createUser error' + error);
        }
    }

    async function updateUser() {
        try {

            const changes = {id};

            // adds the NEW values to the changes object
            for (const field in $user) {
                if ($user[field] !== currentUser[field]) changes[field] = $user[field];
            }     

            if (Object.keys(changes).length <= 1) return; 

            const response = await sendJson('/api/auth/user', 'PUT', changes);
            if (response.status !== 204) {
                const data = await response.json();
                return alert(data.message);
            }
            
            $user.password = '';
            currentUser = {...$user};

            users.update(prev => prev.set(id, currentUser))

            setState('view');

        } catch (error) {
            alert('Could not update the user');
            console.error(error);
        }
    }

</script>

<div class='segment'>

    <div class='menu'>
        <button on:click={() => {setState('new')}} disabled={state.new || $account.role === 'Worker'}>New</button>
        <!-- This one is gonna be fun-->
        <button on:click={() => {setState('edit')}} disabled={(state.new || state.edit) || ($account.role === 'Worker' && $user.id !== $account.id )}>Edit</button>

        <button on:click={() => {setState('cancel')}} disabled={!(state.new || state.edit)}>Cancel</button>
    </div>
    
    <form on:submit|preventDefault={handleSubmit}>
        <label for='first_name'>First Name</label>
        <input id='first_name' type='text' bind:value={$user.first_name} disabled={!(state.edit || state.new)} required>
        
        <label for='last_name'>Last Name</label>
        <input id='last_name' type='text' bind:value={$user.last_name} disabled={!(state.edit || state.new)} required>
        
        <label for='email'>Email</label>
        <input id='email' type='text' bind:value={$user.email} disabled={!(state.edit || state.new)} required>

        <label for='password'>Password</label>
        <input id='password' type='password' bind:value={$user.password} disabled={!(state.edit || state.new) || ($user.role === 'Worker' || $user.role === 'Super')} required={!($user.role === 'Worker' || $user.role === 'Super')}>

        <label for='phone'>Phone</label>
        <input id='phone' type='text' bind:value={$user.phone} disabled={!(state.edit || state.new)}>

        <label for='role'>Role</label>
        <select id='role' bind:value={$user.role} disabled={!(state.edit || state.new)} required>
            <option value='Worker'>Worker</option>
            <option value='Basic'>Basic</option>
            <option value='Administrator'>Administrator</option>
            <option value='Super'>Super</option>
        </select>
        
        <label for='active'>Active</label>
        <input id='active' type='checkbox' bind:checked={$user.active} disabled={!(state.edit || state.new)}>
        
        <label for='last_login'>Last Login</label>
        <input id='last_login' type='text' bind:value={$user.last_login} disabled readonly>

        <label for='id'>ID</label>
        <input id='id' type='text' bind:value={$user.id} disabled readonly>

        <input id='save' type='submit' value='Save' disabled={!(state.edit || state.new)}>

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