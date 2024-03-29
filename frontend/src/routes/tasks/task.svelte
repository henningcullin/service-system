<script>
    // @ts-nocheck
    import { sendJson, getMachines, getUsers } from '$lib/utils';
    import { account, task, tasks, users, machines } from '$lib/stores';
    import { navigate } from 'svelte-navigator';
    
    
    const state = {
        edit: false,
        new: false
    };
    
    let currentTask = emptyTask();
    
    task.set(emptyTask());
    
    const urlParams = new URLSearchParams(window.location.search);
    
    let id = urlParams.get('id');
    
    if (Boolean(urlParams.get('new'))) setState('new');
    if (Boolean(urlParams.get('edit'))) setState('edit');
    
    if (!state.new && (!id || id.length !== 36)) navigate('/notfound');
    else if (!state.new) getTask();
    
    if ((state.new || state.edit) && ($account['role'] === 'Worker' || $account === {})) window.history.back();  

    getMachines();
    getUsers();

    function setState(prop) {
        const path = location.pathname+location.search;

        switch (prop) {
            case 'new':
                state.new = true;
                state.edit = false;
                task.set(emptyTask());
                if (path !== '/task?new=true') navigate('/task?new=true');
                break;
            case 'edit':
                state.new = false;
                state.edit = true;
                if (path !== `/task?id=${id}&edit=true`) navigate(`/task?id=${id}&edit=true`);
                break;
            default:
                state.new = false;
                state.edit = false;
                if (id && path !== `/task?id=${id}`) navigate(`/task?id=${id}`)
                else return navigate('/tasks/');
                resetTask();
                break;
        };
    }

    function emptyTask() {
        return {
            id: '',
            title: '',
            description: '',
            task_type: 'Other',
            status: 'Pending',
            archived: false,
            created: '',
            edited: '',
            creator: '',
            executor: '',
            machine: '',
        };
    }

    function resetTask() {
        task.set({...currentTask});
    }

    function setTask(newData) {

        const {id, title, description, task_type, status, archived, created, edited, creator, executor, machine} = newData;

        currentTask = {
            id,
            title,
            description: description ? description : '',
            task_type,
            status,
            archived: Boolean(archived),
            created: created ? new Date(created) : '',
            edited: edited ? new Date(edited) : '',
            creator,
            executor: executor ? executor : '',
            machine: machine ? machine : '',
        };

        resetTask();

    }

    async function getTask() {
        try {
            const response = await fetch(`/api/auth/task?id=${id}`);

            if (response.status !== 200) navigate('/notfound');

            const data = await response.json();

            setTask(data);

        } catch (error) {
            console.error(error)
        }
    }

    function handleSubmit() {
        switch (true) {
            case (state.new):
                createTask();
                break;
            case (state.edit):
                updateTask();
                break;
        };
    }

    async function createTask() {
        try {

            if (!state.new) return;

            const newTask = Object.fromEntries(
                Object.entries($task).filter(([key, value]) => {
                    if (key === 'archived') return true
                    else return !value !== true
                })
            );

            console.log(newTask);

            const response = await sendJson('/api/auth/task', 'POST', newTask);
    
            const data = await response.json();
    
            console.log(data);

            if (response.status != 201) return alert(data.message);

            id = data.id;
    
            data.created = Date.now();
            data.edited = Date.now();

            setTask(data);
            
            setState('edit');

        } catch (error) {
            alert('Could not create the task');
            console.error('createTask error' + error);
        }
    }

    async function updateTask() {
        try {

            const changes = {id};

            // adds the NEW values to the changes object
            for (const field in $task) {
                if ($task[field] !== currentTask[field]) changes[field] = $task[field];
            }     

            if (Object.keys(changes).length <= 1) return; 

            const response = await sendJson('/api/auth/task', 'PUT', changes);

            if (response.status !== 204) {
                const data = await response.json();
                return alert(data.message);
            }
            
            $task.password = '';

            currentTask = {...$task};

            tasks.update(prev => prev.map(u => {
                if (u.id === id) return currentTask;
                else return u;
            }));

            setState('view');

        } catch (error) {
            alert('Could not update the task');
            console.error(error);
        }
    }

</script>

<div class='segment'>

    <div class='menu'>
        <button on:click={() => {setState('new')}} disabled={state.new || $account.role === 'Worker'}>New</button>
        <!-- This one is gonna be fun-->
        <button on:click={() => {setState('edit')}} disabled={(state.new || state.edit) || ($account.role === 'Worker' && $task.id !== $account.id )}>Edit</button>

        <button on:click={() => {setState('cancel')}} disabled={!(state.new || state.edit)}>Cancel</button>
    </div>
    
    <form on:submit|preventDefault={handleSubmit}>
        <label for='title'>Title</label>
        <input id='title' type='text' bind:value={$task.title} disabled={!(state.edit || state.new)} required>
        
        <label for='description'>Description</label>
        <input id='description' type='text' bind:value={$task.description} disabled={!(state.edit || state.new)} required>
        
        <label for='task_type'>Task Type</label>
        <select id='task_type' bind:value={$task.task_type} disabled={!(state.edit || state.new)} required>
            <option value='Suggestion'>Suggestion</option>
            <option value='Issue'>Issue</option>
            <option value='Service'>Service</option>
            <option value='Other'>Other</option>
        </select>
        
        <label for='status'>Task Type</label>
        <select id='status' bind:value={$task.status} disabled={!(state.edit || state.new)} required>
            <option value='Pending'>Pending</option>
            <option value='Active'>Active</option>
            <option value='Completed'>Completed</option>
        </select>

        <label for="creator">Creator</label>
        <input id='creator' bind:value={$task.creator} disabled readonly>

        <label for="executor">Executor</label>
        <select id='executor' bind:value={$task.executor} disabled={!(state.edit || state.new)}>
            <option value=''>Not Set</option>
            {#each $users.values() as u}
                <option value='{u.id}'>{u.first_name}, {u.last_name}</option>
            {/each}
        </select>

        <label for="machine">Machine</label>
        <select id='machine' bind:value={$task.machine} disabled={!(state.edit || state.new)}>
            <option value=''>Not Set</option>
            {#each $machines.values() as m}
                <option value='{m.id}'>{m.make ? `${m.name}, ${m.make}` : m.name}</option>
            {/each}
        </select>

        <label for='archived'>Archived</label>
        <input id='archived' type='checkbox' bind:checked={$task.archived} disabled={!(state.edit || state.new)}>

        <label for='created'>Created</label>
        <input type='text' bind:value={$task.created} disabled readonly>
        
        <label for='edited'>Edited</label>
        <input type='text' bind:value={$task.edited} disabled readonly>
        
        <label for='id'>ID</label>
        <input id='id' type='text' bind:value={$task.id} disabled readonly>

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