<script lang="ts">
    export let value: string;
</script>

{#if value}
    <div class="flex space-x-2">
        <span class="max-w-[250px] truncate font-medium">
            <a href="mailto:{value}">{value}</a>
        </span>
    </div>
{/if}
