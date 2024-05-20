<script>
    import MachineTable from '$components/MachinesTable/table.svelte';
    import { getMachineStatuses, getMachineTypes, getMachines, getFacilities } from '$utils';
    import MachineCard from '$components/MobileCards/MachineCard.svelte';

    import { machines } from '$stores';

    getMachineStatuses();
    getMachineTypes();
    getMachines();
    getFacilities();

    document.title = 'Machines';
</script>

<div class="hidden h-full flex-1 flex-col space-y-8 p-8 md:flex">
    <div class="flex items-center justify-between space-y-2">
        <div>
            <h2 class="text-2xl font-bold tracking-tight">Machines</h2>
        </div>
    </div>
    <MachineTable />
</div>

<div class="h-full flex-1 flex-col space-y-8 p-4 md:hidden">
    <div class="flex items-center justify-between space-y-2">
        <div>
            <h2 class="text-2xl font-bold tracking-tight">Machines</h2>
        </div>
    </div>
    {#each $machines as machine}
        <MachineCard {machine} />
    {/each}
</div>
