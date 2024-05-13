<script>
    import { facilities, machineStatuses, machineTypes } from '$stores';
    import { getFacilities, getMachine, getMachineStatuses, getMachineTypes } from '$utils';
    import { onMount } from 'svelte';
    import { navigate, useLocation } from 'svelte-navigator';

    const form = {
        id: '',
        name: '',
        make: '',
        machine_type: '',
        status: '',
        facility: '',
    };

    getMachineTypes();
    getMachineStatuses();
    getFacilities();

    $: location = useLocation();

    $: params = new URLSearchParams($location.search);

    let id;

    $: {
        const segments = $location.pathname.split('/');
        id = segments.length > 2 ? segments.at(-1) : null;
    }

    onMount(() => {
        if (id) getMachine(id);
    });

    $: isCreating = params?.get('new') === 'true';
    $: isEditing = params?.get('edit') === 'true' /*  && id */;
    $: isViewing = !(isCreating || isEditing);

    async function newMachine() {
        navigate('?new=true');
    }

    async function editMachine() {
        navigate('?edit=true');
    }

    async function deleteMachine() {
        navigate('?view=true');
    }
</script>

<tab>
    <div class="action container">
        <button on:click={newMachine}>New</button>
        <button on:click={editMachine}>Edit</button>
        <button on:click={deleteMachine}>Delete</button>
    </div>

    <form>
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
    </form>
</tab>

<tab> </tab>

<tab> </tab>
