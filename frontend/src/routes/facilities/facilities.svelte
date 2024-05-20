<script lang="ts">
    document.title = 'Facilities';
    import * as Tabs from '$components/ui/tabs/index.js';
    import FacilityCard from '$components/FacilityCard/FacilityCard.svelte';

    // user stuff

    // machine stuff

    import MachineTable from '$components/MachinesTable/table.svelte';
    import { getMachineStatuses, getMachineTypes, getMachines, getFacilities } from '$utils';
    import MachineCard from '$components/MobileCards/MachineCard.svelte';

    import { machines } from '$stores';

    getMachineStatuses();
    getMachineTypes();
    getMachines();
    getFacilities();
</script>

<Tabs.Root class="flex flex-col items-center min-h-screen pt-8">
    <Tabs.List class="flex space-x-4">
        <Tabs.Trigger value="card">Facilities</Tabs.Trigger>
        <Tabs.Trigger value="users">Users</Tabs.Trigger>
        <Tabs.Trigger value="machines">Machines</Tabs.Trigger>
    </Tabs.List>
    <div class="flex flex-col min-w-full">
        <Tabs.Content value="card">
            <FacilityCard />
        </Tabs.Content>
        <Tabs.Content value="users"></Tabs.Content>
        <Tabs.Content value="machines">
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
        </Tabs.Content>
    </div>
</Tabs.Root>
