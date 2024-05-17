<script lang="ts">
    import * as Select from '$components/ui/select/index.js';
    import * as Tabs from '$components/ui/tabs/index.js';
    import TypeStatusCard from '$components/MachinePanel/Card.svelte';
    import { machineTypes, machineType, machineStatuses, machineStatus } from '$stores';
    import { getMachineTypes, getMachineStatuses } from '$utils';

    getMachineStatuses();
    getMachineTypes();
</script>

<div class="w-full min-h-[15vh] flex items-center justify-center">
    <h2 class="text-2xl font-bold tracking-tight pb-2">Machine Panel</h2>
</div>

<div class="w-full min-h-[65vh] flex items-center justify-center">
    <Tabs.Root value="type" class="w-[400px]">
        <Tabs.List class="grid w-full grid-cols-2">
            <Tabs.Trigger value="type">Type</Tabs.Trigger>
            <Tabs.Trigger value="status">Status</Tabs.Trigger>
        </Tabs.List>
        <Tabs.Content value="type">
            <TypeStatusCard
                cardProps={{
                    title: 'Type',
                    desc: 'Manage your machine types',
                }}
                formProps={{
                    selectPlaceholder: 'Select a type',
                    namePlaceholder: 'Name of type',
                }}
                formStore={machineType}
                apiEndpoint="machine_type"
            >
                {#each $machineTypes as machineType}
                    <Select.Item value={machineType.id} label={machineType.name} />
                {/each}
            </TypeStatusCard>
        </Tabs.Content>
        <Tabs.Content value="status">
            <TypeStatusCard
                cardProps={{
                    title: 'Status',
                    desc: 'Manage your machine statuses',
                }}
                formProps={{
                    selectPlaceholder: 'Select a status',
                    namePlaceholder: 'Name of status',
                }}
                formStore={machineStatus}
                apiEndpoint="machine_status"
            >
                {#each $machineStatuses as machineStatus}
                    <Select.Item value={machineStatus.id} label={machineStatus.name} />
                {/each}
            </TypeStatusCard>
        </Tabs.Content>
    </Tabs.Root>
</div>
