<script>
    // @ts-nocheck
    import { machine } from '../../lib/stores';
    import { navigate } from 'svelte-navigator';

    /** @type {string} */
    export let id;

    if (!id || id.length != 36) navigate('/notfound');

    async function getMachine() {
        try {
            const response = await fetch('/api/auth/machine?id='+ id);
            const data = await response.json();

            // @ts-ignore
            machine.set({
                id: data.id,
                name: data.name,
                make: data.make,
                type: data.machine_type,
                status: data.status,
                created: new Date(data.created).toLocaleString('en-GB'),
                edited: new Date(data.edited).toLocaleString('en-GB'),
            });

        } catch (error) {
            console.error(error)
        }
    }

    getMachine();

    $: m = $machine;

</script>

<div class='segment'>

    <form>
        <label for='id'>ID</label>
        <input id='id' type='text' value='{m.id}' disabled readonly>

        <label for='name'>Name</label>
        <input id='name' type='text' bind:value={m.name} readonly>
        
        <label for='make'>Make</label>
        <input id='make' type='text' bind:value={m.make} readonly>
        
        <label for='type'>Type</label>
        <input id='type' type='text' bind:value={m.type} readonly>
        
        <label for='status'>Status</label>
        <input id='status' type='text' bind:value={m.status} readonly>
        
        <label for='created'>Created</label>
        <input id='created' type='text' value='{m.created}' disabled readonly>
        
        <label for='edited'>Edited</label>
        <input id='edited' type='text' value='{m.edited}' disabled readonly>
    </form>

</div>

<style>

    label {
        font-size:1.1em;
        text-align: left;
    }

    input {
        margin-bottom: 25px;
        font-size:1.15em;
        line-height: 1.5em;
    }

</style>