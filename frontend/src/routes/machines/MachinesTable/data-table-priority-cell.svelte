<script lang="ts">
    import { priorities } from './data';
    export let value: string;
    const priority = priorities.find((priority) => priority.value === value);
</script>

{#if priority}
    <div class="flex items-center">
        <span>{priority.label}</span>
    </div>
{/if}
