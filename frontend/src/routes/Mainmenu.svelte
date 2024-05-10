<script>
    import { Grid, Items, Divider } from "svelte-fomantic-ui";
    import TaskItem from "$lib/components/TaskItem.svelte";

    import { tasks, reports } from "$lib/stores";
    import { getReports, getTasks } from "$lib/utils";

    getReports();
    getTasks();
</script>

<Grid ui two column very relaxed>
    <Items column ui divided>
        {#each $tasks as task}
            <TaskItem {task} />
        {/each}
    </Items>
    <Items column ui divided>
        {#each $reports as report}
            <div></div>
        {/each}
    </Items>
</Grid>
<Divider ui vertical />
