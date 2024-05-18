<script>
    import TaskTable from '$components/TasksTable/table.svelte';
    import { getTasks } from '$utils';
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
