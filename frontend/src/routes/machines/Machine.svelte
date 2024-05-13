<script>
    import { facilities, machine, machineStatuses, machineTypes } from '$stores';
    import { getFacilities, getMachine, getMachineStatuses, getMachineTypes } from '$utils';

    export let id;

    if (id) getMachine(id);

    getMachineTypes();
    getMachineStatuses();
    getFacilities();

    const url = new URL(location.href).searchParams;
    const params = url.searchParams;

    $: isCreating = new URL(location?.href)?.searchParams?.get('new');
    $: isEditing = new URL(location?.href)?.searchParams?.get('edit');

    $: isViewing = !(isCreating || isEditing);
</script>

<form>
    <label for="id">ID</label>
    <input type="text" id="id" bind:value={$machine.id} disabled />

    <label for="name">Name</label>
    <input type="text" id="name" bind:value={$machine.name} placeholder="Name" disabled={isViewing} />

    <label for="make">Make</label>
    <input type="text" id="make" bind:value={$machine.make} placeholder="Make" disabled={isViewing} />

    <label for="type">Type</label>
    <select id="type" value="hi" disabled={isViewing}>
        {#each $machineTypes as machineType}
            <option value={machineType.id}>{machineType.name}</option>
        {/each}
    </select>

    <label for="status">Status</label>
    <select id="status" disabled={isViewing}>
        {#each $machineStatuses as machineStatus}
            <option value={machineStatus.id}>{machineStatus.name}</option>
        {/each}
    </select>

    <label for="facility">Facility</label>
    <select id="facility" disabled={isViewing}>
        {#each $facilities as facility}
            <option value={facility.id}>{facility.name}</option>
        {/each}
    </select>
</form>
