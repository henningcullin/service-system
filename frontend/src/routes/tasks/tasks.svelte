<script>
    import TaskTable from '$components/TasksTable/table.svelte';
    import TaskCard from '$components/MainMenu/TaskCard.svelte';
    import { tasks } from '$stores';
    import { getTasks, evToObj, getOne } from '$utils';
    import { onMount } from 'svelte';
    getTasks();

    document.title = 'Tasks';

    onMount(() => {
        const taskChannel = new EventSource('/api/auth/channel/tasks');

        taskChannel.onmessage = async (e) => {
            const message = evToObj(e);
            if (!message) return;
            const { id, kind } = message;
            switch (kind) {
                case 'INSERT':
                    const insertData = await getOne(`/api/auth/task?task_id=${id}`);
                    if (!insertData) return;
                    tasks.update((prev) => {
                        prev.unshift(insertData);
                        return prev;
                    });
                    break;
                case 'UPDATE':
                    const updateData = await getOne(`/api/auth/task?task_id=${id}`);
                    if (!updateData) return;
                    const index = $tasks.findIndex((t) => t.id === id);
                    tasks.update((prev) => {
                        prev[index] = updateData;
                        return prev;
                    });
                    break;
                case 'DELETE':
                    tasks.update((prev) => prev.filter((t) => t.id !== id));
                    break;
            }
        };

        window.onbeforeunload = () => {
            taskChannel.close();
        };

        return () => {
            taskChannel.close();
        };
    });
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
        <TaskCard {task} />
    {/each}
</div>
