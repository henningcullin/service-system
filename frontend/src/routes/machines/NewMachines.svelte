<script>
    import MachineTable from '$components/MachineTable/machine-table.svelte';
    import { machineStatuses, machineTypes, machines } from '$stores';
    import { getMachineStatuses, getMachineTypes, getMachines } from '$utils';

    getMachineStatuses();
    getMachineTypes();
    getMachines();
</script>

<div class="hidden h-full flex-1 flex-col space-y-8 p-8 md:flex">
    <MachineTable {data} />
</div>
