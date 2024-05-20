<script>
    import MachineTable from '$components/MachinesTable/table.svelte';
    import { getMachineStatuses, getMachineTypes, getMachines, getFacilities } from '$utils';

    import { machines } from '$stores';
    import * as Card from '$components/ui/card';
    import { Link } from 'svelte-navigator';

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
        <Card.Root class="w-[100%]">
            <Card.Header>
                <Card.Title><Link to="/machine/{machine?.id}">{machine?.name}</Link></Card.Title>
                <Card.Description></Card.Description>
                <Card.Content></Card.Content>
            </Card.Header>
        </Card.Root>
    {/each}
</div>
