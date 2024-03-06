<script>

    import Grid from "gridjs-svelte"
    import { Link, navigate } from "svelte-navigator";
    import { onMount } from 'svelte';
    import { machines } from '../../lib/stores.js'

    onMount(async () => {
        try {
            const response = await fetch('/api/auth/machines');
            const data = await response.json();

            // @ts-ignore
            const formatted = data.map((machine) => {
                return {
                    id: machine.id,
                    name: machine.name,
                    make: machine.make,
                    type: machine.machine_type,
                    status: machine.status,
                    created: new Date(machine.created),
                    edited: new Date(machine.edited),
                };
            })

            machines.set(formatted);
        } catch (error) {
            console.log('Could not fetch products', error);
        }
    });

    const columns = [
        // @ts-ignore
        'Id',
        'Name', 
        'Make', 
        'Type', 
        'Status', 
        // @ts-ignore
        {name:'Created', formatter: (cell) => cell.toLocaleString('en-GB')}, 
        // @ts-ignore
        {name:'Edited', formatter: (cell) => cell.toLocaleString('en-GB')}, 
        'Delete', 
        'Edit'
    ]

    // @ts-ignore
    function handleTableEvent(e) {
        try {
            
            const target = e.detail[0].target;
            if (!target) return;

            const type = target.getAttribute('data-column-id');

            if (!type) return;

            switch (type) {
                case 'id':
                    navigate(`/machines/${target.innerText}`);
                    break;

            }

        } catch (error) {
            console.log(error);
        }
    }

</script>

<div class="segment">
    <h2> Welcome to the Machine page!!</h2>
    
    <div class="menu">
        <Link to="/machines/new">New</Link>
    </div>

    <div class="mobile-grid">
        {#each $machines as machine}
            <div class="mobile-card"> 
                <b>Name: {machine.name}</b>
                <p>Make: {machine.make}</p>
                <p>Type: {machine.machine_type}</p>
                <p>Status: {machine.status}</p>
                <i title={machine.created.toLocaleTimeString()}>Created: {machine.created.toLocaleDateString()}</i><br>
                <i title={machine.edited.toLocaleTimeString()}>Edited: {machine.edited.toLocaleDateString()}</i>
            </div>
        {/each}
    </div>

    <Grid on:rowClick={handleTableEvent} columns={columns} data={$machines} sort={true} search={true}, pagination={{limit: 15}}/>
    
</div>


<style>

    .mobile-grid {
        margin-top: 50px;
        display:grid;
        width:100%;
        grid-template-columns: 1fr 1fr;
        gap:10px;
        padding:0.5%;
    }

    .mobile-card {
        padding: 0.5%;
        width:100%;
        background-color: #353535;
        border-radius: 5px;
    }

    :global([data-column-id="id"]) {
        color: #2d53bd;
        cursor: pointer;
    }

</style>