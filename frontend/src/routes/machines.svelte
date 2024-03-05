<script>

    import { onMount } from 'svelte';
    import { machines } from '../lib/stores.js'

    onMount(async () => {
        try {
            const response = await fetch('/api/auth/machines');
            const data = await response.json();
            machines.set(data);
        } catch (error) {
            console.log('Could not fetch products', error);
        }
    });

</script>

<h2> Welcome to the Machine page!!</h2>

<div class="machine-grid">
    {#each $machines as machine}
        <div> 
            <b>Name: {machine.name}</b>
            <p>Make: {machine.make}</p>
            <p>Type: {machine.machine_type}</p>
            <p>Status: {machine.status}</p>
            <i>Created: {machine.created}</i><br>
            <i>Edited: {machine.edited}</i>
        </div>
    {/each}
</div>