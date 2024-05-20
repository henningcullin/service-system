<script lang="ts">
    import * as Tabs from '$components/ui/tabs/index.js';
    import TaskCard from '$components/TaskCard/TaskCard.svelte';

    document.title = 'Task';
</script>

<Tabs.Root class="flex flex-col items-center min-h-screen pt-8">
    <Tabs.List class="flex space-x-4">
        <Tabs.Trigger value="card">Machine</Tabs.Trigger>
        <Tabs.Trigger value="tasks">Tasks</Tabs.Trigger>
        <Tabs.Trigger value="reports">Reports</Tabs.Trigger>
    </Tabs.List>
    <Tabs.Content value="card" class="flex flex-col min-w-full">
        <TaskCard />
    </Tabs.Content>
    <Tabs.Content value="tasks"></Tabs.Content>
    <Tabs.Content value="reports"></Tabs.Content>
</Tabs.Root>
