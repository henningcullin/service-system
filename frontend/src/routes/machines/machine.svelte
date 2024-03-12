<script>
    // @ts-nocheck
    import { onMount } from 'svelte';
    import { el, sendDelete, sendJson } from '../../lib/helpers';

    import { account, machine, machines } from '../../lib/stores';
    import { navigate } from 'svelte-navigator';
    
    resetMachine();

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
    
    if(id) getMachine();

    onMount(() => {
        computeState();
    })

    /**
     * true to turn on false to turn off
     * @param {bool} state
     */
     function computeState() {
        const combined = Boolean((state.new || state.edit));

        if (!el('#name')) return;

        el('#name').disabled = !combined;
        el('#make').disabled = !combined;
        el('#machine_type').disabled = !combined;
        el('#status').disabled = !combined;
        el('#save').disabled = !combined;

        el('#new').disabled = state.new;
        el('#edit').disabled = (state.new || state.edit);
        el('#delete').disabled = state.new;
        el('#cancel').disabled = !(state.new || state.edit)
    }

    function setState(prop, newState) {
        if (newState != true && newState != false) return;

        const path = location.pathname+location.search;

        switch (prop) {
            case 'new':
                state.new = newState;
                state.edit = false;
                resetMachine();
                if (path !== '/machine?new=true') navigate('/machine?new=true');
                break;
            case 'edit':
                state.new = false;
                state.edit = newState;
                if (path !== `/machine?id=${id}&edit=true`) navigate(`/machine?id=${id}&edit=true`);
                break;
            default:
                state.new = false;
                state.edit = false;
                if (id && path !== `/machine?id=${id}`) navigate(`/machine?id=${id}`)
                else {
                    navigate('/machines/');
                    return;
                }
                getMachine();
                break;
        };
        computeState();
    }

    function resetMachine() {
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

            machine.set({
                id: data.id,
                name: data.name,
                make: data.make,
                machine_type: data.machine_type,
                status: data.status,
                created: new Date(data.created).toLocaleString('en-GB'),
                edited: new Date(data.edited).toLocaleString('en-GB'),
            });

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

            if (!state.new || $machine.name.length <= 0 || ($machine.status != 'Inactive' && $machine.status != 'Active')) return;
            
            const response = await sendJson('/api/auth/machine', 'POST', {
                name: $machine.name,
                make: $machine.make,
                machine_type: $machine.machine_type,
                status: $machine.status,
            });
    
            const data = await response.json();
    
            if (response.status != 201) return alert(data.message);
    
            id = data.id;
    
            setState('edit', true);
    
            $machine.id = data.id;
            $machine.created = new Date().toLocaleString('en-GB');
            $machine.edited = new Date().toLocaleString('en-GB');

        } catch (error) {
            console.error('createMachine error' + error);
        }
    }

    async function updateMachine() {
        try {
            const response = await sendJson('/api/auth/machine', 'PUT', {
                id: $machine.id,
                name: $machine.name,
                make: $machine.make,
                machine_type: $machine.machine_type,
                status: $machine.status,
            });

            if (response.status != 204) {
                const data = await response.json();
                return alert(data.message);
            }

        } catch (error) {
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
        
        setState('new', true);
    }

</script>

<div class='segment'>

    <div class="menu">
        <button id='new' on:click={() => {setState('new', true)}}>New</button>
        <button id='edit' on:click={() => {setState('edit', true)}}>Edit</button>
        <button id='delete' on:click={deleteMachine}>Delete</button>
        <button id='cancel' on:click={() => {setState('cancel', true)}}>Cancel</button>
    </div>
    
    <form on:submit|preventDefault={handleSubmit}>
        <label for='id'>ID</label>
        <input id='id' type='text' bind:value={$machine.id} disabled readonly>

        <label for='name'>Name</label>
        <input id='name' type='text' bind:value={$machine.name} disabled>
        
        <label for='make'>Make</label>
        <input id='make' type='text' bind:value={$machine.make} disabled>
        
        <label for='machine_type'>Type</label>
        <input id='machine_type' type='text' bind:value={$machine.machine_type} disabled>

        <label for='status'>Status</label>
        <select id='status' bind:value={$machine.status} disabled>
            <option value='Inactive'>Inactive</option>
            <option value='Active'>Active</option>
        </select>
        
        <label for='created'>Created</label>
        <input id='created' type='text' bind:value={$machine.created} disabled readonly>
        
        <label for='edited'>Edited</label>
        <input id='edited' type='text' bind:value={$machine.edited} disabled readonly>

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

/*         #save, #edited, #created, #status {
            float:right;
        } */

    }

</style>