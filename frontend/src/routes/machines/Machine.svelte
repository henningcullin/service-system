<script>
    import { facilities, machine, machineStatuses, machineTypes } from '$stores';
    import { getFacilities, getMachine, getMachineStatuses, getMachineTypes } from '$utils';
    import { onMount } from 'svelte';
    import { navigate, useLocation } from 'svelte-navigator';

    getMachineTypes();
    getMachineStatuses();
    getFacilities();

    const form = {
        id: '',
        name: '',
        make: '',
        machine_type: '',
        status: '',
        facility: '',
    };

    function clearFields() {
        for (const field in form) {
            form[field] = '';
        }
    }

    function loadFields() {
        form.id = $machine?.id;
        form.name = $machine?.name;
        form.make = $machine?.make;
        form.machine_type = $machine?.machine_type?.id;
        form.status = $machine?.status?.id;
        form.facility = $machine?.facility?.id;
    }

    $: location = useLocation();

    $: params = new URLSearchParams($location.search);

    let id;

    $: {
        const segments = $location.pathname.split('/');
        id = segments.length > 2 ? segments.at(-1) : null;
    }

    onMount(async () => {
        if (id) {
            await getMachine(id);
            loadFields();
        }
    });

    $: isCreating = params?.get('new') === 'true';
    $: isEditing = params?.get('edit') === 'true' && id;
    $: isViewing = !(isCreating || isEditing);

    async function newMachine() {
        navigate('?new=true');
        clearFields();
    }

    async function editMachine() {
        navigate('?edit=true');
        if (id) loadFields();
    }

    async function deleteMachine() {
        navigate('?view=true');
    }

    function cancel() {
        navigate('?view=true');
        if (id) loadFields();
    }

    async function saveMachine() {
        if (isViewing) return;
    }
</script>

<tab>
    <div class="action container">
        <button on:click={newMachine} disabled={isCreating}>New</button>
        <button on:click={editMachine} disabled={isEditing || !id}>Edit</button>
        <button on:click={deleteMachine} disabled={isCreating || !id}>Delete</button>
        <button on:click={cancel}>Cancel</button>
    </div>

    <form on:submit|preventDefault={saveMachine}>
        <label for="id">ID</label>
        <input type="text" id="id" bind:value={form.id} disabled />

        <label for="name">Name</label>
        <input type="text" id="name" bind:value={form.name} placeholder="Name" disabled={isViewing} />

        <label for="make">Make</label>
        <input type="text" id="make" bind:value={form.make} placeholder="Make" disabled={isViewing} />

        <label for="type">Type</label>
        <select id="type" disabled={isViewing} bind:value={form.machine_type}>
            <option value="" class="hidden" disabled selected></option>
            {#each $machineTypes as machineType}
                <option value={machineType.id}>{machineType.name}</option>
            {/each}
        </select>

        <label for="status">Status</label>
        <select id="status" disabled={isViewing} bind:value={form.status}>
            <option value="" class="hidden" disabled selected placeholder></option>
            {#each $machineStatuses as machineStatus}
                <option value={machineStatus.id}>{machineStatus.name}</option>
            {/each}
        </select>

        <label for="facility">Facility</label>
        <select id="facility" disabled={isViewing} bind:value={form.facility}>
            <option value="" class="hidden" disabled selected placeholder></option>
            {#each $facilities as facility}
                <option value={facility.id}>{facility.name}</option>
            {/each}
        </select>

        <button class="saveButton" type="submit" disabled={isViewing}>Save</button>
    </form>
</tab>

<tab> </tab>

<tab> </tab>

<style>
    .saveButton {
        width: 100%;
        margin-top: 1.5em;
    }
</style>
