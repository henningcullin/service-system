<script>
    import MachineTable from '$components/MachinesTable/table.svelte';
    import { getMachineStatuses, getMachineTypes, getMachines, getFacilities } from '$utils';

    getMachineStatuses();
    getMachineTypes();
    getMachines();
    getFacilities();
</script>

<div class="hidden h-full flex-1 flex-col space-y-8 p-8 md:flex">
    <div class="flex items-center justify-between space-y-2">
        <div>
            <h2 class="text-2xl font-bold tracking-tight">Machines</h2>
        </div>
    </div>
    <MachineTable />
</div>
