<script lang="ts">
    export let value: string;
    const dateObj = new Date(value);
</script>

{#if value}
    <div class="flex space-x-2">
        <span class="max-w-[200px] truncate font-medium" title={dateObj?.toLocaleString('sv-SE')}>
            {dateObj?.toDateString()}
        </span>
    </div>
{/if}
