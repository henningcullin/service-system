<script>
    import { X } from 'lucide-svelte';
    import { DataTableFacetedFilter, DataTableViewOptions } from './index.js';
    import Button from '$lib/components/ui/button/button.svelte';
    import { Input } from '$lib/components/ui/input/index.js';

    export let tableModel;
    export let data;
    export let machineTypes;
    export let machineStatuses;

    const counts = data.reduce(
        (acc, { status, priority }) => {
            acc.status[status] = (acc.status[status] || 0) + 1;
            acc.priority[priority] = (acc.priority[priority] || 0) + 1;
            return acc;
        },
        {
            status: {},
            priority: {},
        },
    );

    const { pluginStates } = tableModel;
    const { filterValue } = pluginStates.filter;

    const { filterValues } = pluginStates.colFilter;

    $: showReset = Object.values({ ...$filterValues, $filterValue }).some((v) => v.length > 0);
</script>

<div class="flex items-center justify-between">
    <div class="flex flex-1 items-center space-x-2">
        <Input
            placeholder="Filter machines..."
            class="h-8 w-[150px] lg:w-[250px]"
            type="search"
            bind:value={$filterValue}
        />

        <DataTableFacetedFilter
            bind:filterValues={$filterValues.status}
            title="Type"
            options={machineTypes}
            counts={counts.type}
        />
        <DataTableFacetedFilter
            bind:filterValues={$filterValues.priority}
            title="Status"
            options={machineStatuses}
            counts={counts.status}
        />
        {#if showReset}
            <Button
                on:click={() => {
                    $filterValue = '';
                    $filterValues.type = [];
                    $filterValues.status = [];
                }}
                variant="ghost"
                class="h-8 px-2 lg:px-3"
            >
                Reset
                <X class="ml-2 h-4 w-4" />
            </Button>
        {/if}
    </div>

    <DataTableViewOptions {tableModel} />
</div>
