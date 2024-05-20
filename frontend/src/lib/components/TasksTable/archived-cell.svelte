<script>
    export let value;
    import Check from 'lucide-svelte/icons/check';
    import X from 'lucide-svelte/icons/x';
</script>

<div class="flex space-x-2">
    <span class="max-w-[250px] truncate font-medium">
        {#if value}
            <Check />
        {:else}
            <X />
        {/if}
    </span>
</div>
