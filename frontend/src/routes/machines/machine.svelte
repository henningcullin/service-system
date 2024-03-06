<script>
    import { machine } from '../../lib/stores';
    import { navigate } from 'svelte-navigator';

    /** @type {string} */
    export let id;

    if (!id || id.length != 36) navigate('/notfound');

    machine.set([]);

    async function getMachine() {
        try {
            const response = await fetch('/api/auth/machine?id='+ id);
            const data = await response.json();

            // @ts-ignore
            machine.set([{id: data.id,
                name: data.name,
                make: data.make,
                type: data.machine_type,
                status: data.status,
                created: new Date(data.created),
                edited: new Date(data.edited),
            }]);

        } catch (error) {
            console.error(error)
        }
    }

    getMachine();

</script>

<div class="segment">

    {#each $machine as machine}
        <b>Name: {machine.name}</b>
        <p>Make: {machine.make}</p>
        <p>Type: {machine.type}</p>
        <p>Status: {machine.status}</p>
        <i title={machine.created.toLocaleTimeString()}>Created: {machine.created.toLocaleDateString()}</i><br>
        <i title={machine.edited.toLocaleTimeString()}>Edited: {machine.edited.toLocaleDateString()}</i>
    {/each}

</div>