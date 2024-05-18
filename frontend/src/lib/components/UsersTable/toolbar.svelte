<script>
    import X from 'lucide-svelte/icons/x';
    import { DTViewOptions } from './index.js';
    import Button from '$lib/components/ui/button/button.svelte';
    import { Input } from '$lib/components/ui/input/index.js';

    export let tableModel;

    const { pluginStates } = tableModel;
    const { filterValue } = pluginStates.filter;

    $: showReset = Object.values({ $filterValue }).some((v) => v.length > 0);
</script>

<div class="flex items-center justify-between">
    <div class="flex flex-1 items-center space-x-2">
        <Input
            placeholder="Filter tasks..."
            class="h-8 w-[150px] lg:w-[250px]"
            type="search"
            bind:value={$filterValue}
        />

        {#if showReset}
            <Button
                on:click={() => {
                    $filterValue = '';
                }}
                variant="ghost"
                class="h-8 px-2 lg:px-3"
            >
                Reset
                <X class="ml-2 h-4 w-4" />
            </Button>
        {/if}
    </div>

    <DTViewOptions {tableModel} />
</div>
