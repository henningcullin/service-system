<script>
    import { facilities, machine, machineStatuses, machineTypes } from '$stores';
    import { getFacilities, getMachine, getMachineStatuses, getMachineTypes } from '$utils';

    export let id;

    getMachine(id);
    getMachineTypes();
    getMachineStatuses();
    getFacilities();
</script>

<form>
    <label for="id">ID</label>
    <input type="text" id="id" bind:value={$machine.id} disabled />

    <label for="name">Name</label>
    <input type="text" id="name" bind:value={$machine.name} disabled />

    <label for="make">Make</label>
    <input type="text" id="make" bind:value={$machine.make} disabled />

    <label for="type">Type</label>
    <select id="type" value="hi" disabled>
        {#each $machineTypes as machineType}
            <option value={machineType.id}>{machineType.name}</option>
        {/each}
    </select>

    <label for="status">Status</label>
    <select id="status" disabled>
        {#each $machineStatuses as machineStatus}
            <option value={machineStatus.id}>{machineStatus.name}</option>
        {/each}
    </select>

    <label for="facility">Facility</label>
    <select id="facility" disabled>
        {#each $facilities as facility}
            <option value={facility.id}>{facility.name}</option>
        {/each}
    </select>
</form>
