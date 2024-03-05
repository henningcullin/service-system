<script>

    import { onMount } from 'svelte';
    import { machines } from '../lib/stores.js'

    onMount(async () => {
        try {
            const response = await fetch('/api/auth/machines');
            const data = await response.json();

            const formatted = data.map((/** @type {{ created: string | number | Date; edited: string | number | Date; }} */ machine) => {
                machine.created = new Date(machine.created);
                machine.edited = new Date(machine.edited);
                return machine;
            })

            machines.set(data);
        } catch (error) {
            console.log('Could not fetch products', error);
        }
    });

</script>

<div class="segment">
    <h2> Welcome to the Machine page!!</h2>
    
    <div class="machine-grid">
        {#each $machines as machine}
            <div class="machine-card"> 
                <b>Name: {machine.name}</b>
                <p>Make: {machine.make}</p>
                <p>Type: {machine.machine_type}</p>
                <p>Status: {machine.status}</p>
                <i title={machine.created.toLocaleTimeString()}>Created: {machine.created.toLocaleDateString()}</i><br>
                <i title={machine.edited.toLocaleTimeString()}>Edited: {machine.edited.toLocaleDateString()}</i>
            </div>
        {/each}
    </div>
</div>


<style>
    .machine-grid {
        margin-top: 50px;
        display:grid;
        width:100%;
        grid-template-columns: 1fr, 1fr;
        gap:10px;
        padding:0.5%;
    }

    .machine-card {
        padding: 0.5%;
        width:100%;
        background-color: #353535;
        border-radius: 5px;
    }

</style>