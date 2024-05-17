<script lang="ts">
    import * as Card from '$components/ui/card/index.js';
    import * as Select from '$components/ui/select/index.js';
    import * as Tabs from '$components/ui/tabs/index.js';
    import TypeStatusCard from '$components/MachinePanel/Card.svelte';
    import Button from '$components/ui/button/button.svelte';
    import Input from '$components/ui/input/input.svelte';
    import Label from '$components/ui/label/label.svelte';

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
                props={{
                    title: 'Type',
                    desc: 'Manage your machine types',
                    selectPlaceholder: 'Select a type',
                    namePlaceholder: 'Name of type',
                }}
            >
                {#each $machineTypes as machineType}
                    <Select.Item value={machineType.id} label={machineType.name} />
                {/each}
            </TypeStatusCard>
        </Tabs.Content>
        <Tabs.Content value="status">
            <TypeStatusCard
                props={{
                    title: 'Status',
                    desc: 'Manage your machine statuses',
                    selectPlaceholder: 'Select a status',
                    namePlaceholder: 'Name of status',
                }}
            >
                {#each $machineStatuses as machineStatus}
                    <Select.Item value={machineStatus.id} label={machineStatus.name} />
                {/each}
            </TypeStatusCard>
        </Tabs.Content>
    </Tabs.Root>
</div>
