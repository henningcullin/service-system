<script lang="ts">
    import * as Tabs from '$components/ui/tabs/index.js';
    import TypeStatusCard from '$components/TypeStatusCard.svelte';
    import { taskTypes, taskType, taskStatuses, taskStatus } from '$stores';
    import { getTaskTypes, getTaskStatuses } from '$utils';

    const url = new URL(location.href);
    const type = url.searchParams?.get('type');
    const status = url.searchParams?.get('status');

    getTaskTypes();
    getTaskStatuses();
</script>

<div class="w-full min-h-[15vh] flex items-center justify-center">
    <h2 class="text-2xl font-bold tracking-tight pb-2">Task Panel</h2>
</div>

<div class="w-full min-h-[65vh] flex items-center justify-center">
    <Tabs.Root value={status ? 'status' : 'type'} class="w-[400px]">
        <Tabs.List class="grid w-full grid-cols-2">
            <Tabs.Trigger value="type">Type</Tabs.Trigger>
            <Tabs.Trigger value="status">Status</Tabs.Trigger>
        </Tabs.List>
        <Tabs.Content value="type">
            <TypeStatusCard
                cardProps={{
                    title: 'Type',
                    desc: 'Manage your task types',
                }}
                formProps={{
                    selectPlaceholder: 'Select a type',
                    namePlaceholder: 'Name of type',
                }}
                formStore={taskType}
                sourceStore={taskTypes}
                initialSelected={type}
                apiEndpoint="task_type"
            ></TypeStatusCard>
        </Tabs.Content>
        <Tabs.Content value="status">
            <TypeStatusCard
                cardProps={{
                    title: 'Status',
                    desc: 'Manage your task statuses',
                }}
                formProps={{
                    selectPlaceholder: 'Select a status',
                    namePlaceholder: 'Name of status',
                }}
                formStore={taskStatus}
                sourceStore={taskStatuses}
                initialSelected={status}
                apiEndpoint="task_status"
            ></TypeStatusCard>
        </Tabs.Content>
    </Tabs.Root>
</div>
