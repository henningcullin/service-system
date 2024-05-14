<script>
    import MachineTable from '../../lib/components/MachinesTable/table.sveltee.svelte';
    import { getMachineStatuses, getMachineTypes, getMachines, getFacilities } from '$utils';

    getMachineStatuses();
    getMachineTypes();
    getMachines();
    getFacilities();
</script>

<div class="hidden h-full flex-1 flex-col space-y-8 p-8 md:flex">
    <MachineTable />
</div>
