<script>
    // @ts-nocheck
    import { sendDelete, sendJson } from '../../lib/helpers';

    import { account, machine, machines } from '../../lib/stores';
    import { navigate } from 'svelte-navigator';
    
    const state = {
        edit: false,
        new: false
    };
    
    let currentMachine = {
        id: '',
        name: '',
        make: '',
        machine_type: '',
        status: 'Inactive',
        created: '',
        edited: '',
    };
    
    emptyMachine();

    const urlParams = new URLSearchParams(window.location.search);
    
    let id = urlParams.get('id');
    
    if (Boolean(urlParams.get('new'))) setState('new');
    if (Boolean(urlParams.get('edit'))) setState('edit');
    

    if (!state.new && (!id || id.length != 36)) navigate('/notfound');
    else getMachine();

    if ((state.new || state.edit) && ($account['role'] == 'Worker' || $account == {})) window.history.back();  

    function setState(prop) {
        const path = location.pathname+location.search;

        switch (prop) {
            case 'new':
                state.new = true;
                state.edit = false;
                emptyMachine();
                if (path !== '/machine?new=true') navigate('/machine?new=true');
                break;
            case 'edit':
                state.new = false;
                state.edit = true;
                if (path !== `/machine?id=${id}&edit=true`) navigate(`/machine?id=${id}&edit=true`);
                break;
            default:
                state.new = false;
                state.edit = false;
                if (id && path !== `/machine?id=${id}`) navigate(`/machine?id=${id}`)
                else return navigate('/machines/');
                resetMachine();
                break;
        };
    }

    function resetMachine() {
        machine.set({...currentMachine});
    }

    function emptyMachine() {
        machine.set({
            id: '',
            name: '',
            make: '',
            machine_type: '',
            status: 'Inactive',
            created: '',
            edited: '',
        });
    }

    async function getMachine() {
        try {
            const response = await fetch(`/api/auth/machine?id=${id}`);

            if (response.status != 200) navigate('/notfound');

            const data = await response.json();

            currentMachine = {
                id: data.id,
                name: data.name,
                make: data.make,
                machine_type: data.machine_type,
                status: data.status,
                created: new Date(data.created).toLocaleString('en-GB'),
                edited: new Date(data.edited).toLocaleString('en-GB'),
            };

            machine.set({...currentMachine});

        } catch (error) {
            console.error(error)
        }
    }

    function handleSubmit() {
        switch (true) {
            case (state.new):
                createMachine();
                break;
            case (state.edit):
                updateMachine();
                break;
        };
    }

    async function createMachine() {
        try {

            if (!state.new || $machine.name.length <= 0) return;
            
            const response = await sendJson('/api/auth/machine', 'POST', {
                name: $machine.name,
                make: $machine.make,
                machine_type: $machine.machine_type,
                status: $machine.status,
            });
    
            const data = await response.json();
    
            if (response.status != 201) return alert(data.message);
    
            id = data.id;
    
            setState('edit');
    
            $machine.id = data.id;
            $machine.created = new Date().toLocaleString('en-GB');
            $machine.edited = new Date().toLocaleString('en-GB');

            currentMachine = {...$machine};

        } catch (error) {
            alert('Could not create the machine');
            console.error('createMachine error' + error);
        }
    }

    async function updateMachine() {
        try {

            const changes = {id};

            // adds the NEW values to the changes object
            for (const field in $machine) {
                if ($machine[field] !== currentMachine[field]) changes[field] = $machine[field];
            }            

            if (Object.keys(changes).length < 2) return; 

            const response = await sendJson('/api/auth/machine', 'PUT', changes);

            if (response.status !== 204) {
                const data = await response.json();
                return alert(data.message);
            }

            setState('view');

        } catch (error) {
            alert('Could not update the machine');
            console.error(error);
        }
    }

    async function deleteMachine() {
        if ($account.role == 'Worker') return;

        if (!id) return;
        
        const choice = confirm(`Are you sure you want to delete this machine`);

        if (!choice) return;

        const response = await sendDelete(`/api/auth/machine?id=${id}`);

        if (response.status != 204) return alert('Could not delete');

        machines.update(prev => prev.filter(m => m.id != id));
        
        id = '';
        
        setState('new');
    }

</script>

<div class='segment'>

    <div class="menu">
        <button on:click={() => {setState('new')}}    disabled={state.new || $account.role === 'Worker'}>New</button>

        <button on:click={() => {setState('edit')}}   disabled={(state.new || state.edit) || $account.role === 'Worker'}>Edit</button>

        <button on:click={deleteMachine}              disabled={state.new || $account.role === 'Worker'}>Delete</button>

        <button on:click={() => {setState('cancel')}} disabled={!(state.new || state.edit)} >Cancel</button>
    </div>
    
    <form on:submit|preventDefault={handleSubmit}>
        <label for='id'>ID</label>
        <input id='id' type='text' bind:value={$machine.id} disabled readonly>

        <label for='name'>Name</label>
        <input type='text' bind:value={$machine.name} disabled={!(state.edit || state.new)}>
        
        <label for='make'>Make</label>
        <input type='text' bind:value={$machine.make} disabled={!(state.edit || state.new)}>
        
        <label for='machine_type'>Type</label>
        <input type='text' bind:value={$machine.machine_type} disabled={!(state.edit || state.new)}>

        <label for='status'>Status</label>
        <select bind:value={$machine.status} disabled={!(state.edit || state.new)}>
            <option value='Inactive'>Inactive</option>
            <option value='Active'>Active</option>
        </select>
        
        <label for='created'>Created</label>
        <input type='text' bind:value={$machine.created} disabled readonly>
        
        <label for='edited'>Edited</label>
        <input type='text' bind:value={$machine.edited} disabled readonly>

        <input type="submit" value="Save" disabled={!(state.edit || state.new)}>

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

    @media (min-width: 1200px) {

        .menu>button{
            font-size: 1.5em;
        }

        form {
            margin-top: 50px;
            display:grid;
            grid-template-columns: 1fr 3fr 1fr 3fr;
            grid-template-rows: 1fr 1fr 1fr;
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