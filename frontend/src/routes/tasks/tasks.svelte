<script>
    import TaskTable from '$components/TasksTable/table.svelte';
    import * as Card from '$components/ui/card';
    import { tasks } from '$stores';
    import { getTasks } from '$utils';
    import { Link } from 'svelte-navigator';
    getTasks();
</script>

<div class="hidden h-full flex-1 flex-col space-y-8 p-8 md:flex">
    <div class="flex items-center justify-between space-y-2">
        <div>
            <h2 class="text-2xl font-bold tracking-tight">Tasks</h2>
        </div>
    </div>
    <TaskTable />
</div>

<div class="h-full flex-1 flex-col space-y-8 p-4 md:hidden">
    <div class="flex items-center justify-between space-y-2">
        <div>
            <h2 class="text-2xl font-bold tracking-tight">Tasks</h2>
        </div>
    </div>
    {#each $tasks as task}
        <Card.Root class="w-[100%]">
            <Card.Header>
                <Card.Title><Link to="/task/{task?.id}">{task?.title}</Link></Card.Title>
                <Card.Description>{task?.description}</Card.Description>
            </Card.Header>
        </Card.Root>
    {/each}
</div>
