<script>
    import { tasks } from "$lib/stores";
</script>

{#each $tasks as task}
    <div>{task}</div>
{/each}
