<script lang="ts">
    import * as Tabs from '$components/ui/tabs/index.js';
    import RoleCard from '$components/RoleCard/RoleCard.svelte';

    document.title = 'Roles';
</script>

<Tabs.Root class="flex flex-col items-center min-h-screen pt-8">
    <Tabs.List class="flex space-x-4">
        <Tabs.Trigger value="card">Roles</Tabs.Trigger>
        <Tabs.Trigger value="tasks"></Tabs.Trigger>
        <Tabs.Trigger value="reports"></Tabs.Trigger>
    </Tabs.List>
    <Tabs.Content value="card" class="flex flex-col min-w-full">
        <RoleCard />
    </Tabs.Content>
    <Tabs.Content value="tasks"></Tabs.Content>
    <Tabs.Content value="reports"></Tabs.Content>
</Tabs.Root>
