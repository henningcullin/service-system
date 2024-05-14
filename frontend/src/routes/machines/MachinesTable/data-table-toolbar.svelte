<script lang="ts">
    import type { TableViewModel } from 'svelte-headless-table';
    import X from 'lucide-svelte/icons/x';
    import type { Writable } from 'svelte/store';
    import { machineTypes, machineStatuses, facilities } from '$stores'; // import from $stores
    import type { Machine } from './schema';
    import { DataTableFacetedFilter, DataTableViewOptions } from './index.js';
    import Button from '$lib/components/ui/button/button.svelte';
    import { Input } from '$lib/components/ui/input/index.js';

    export let tableModel: TableViewModel<Machine>;
    export let data: Machine[];

    const counts = data.reduce<{
        machine_type: { [index: string]: number };
        status: { [index: string]: number };
        facility: { [index: string]: number };
    }>(
        (acc, { machine_type, status, facility }) => {
            acc.machine_type[machine_type.name] = (acc.machine_type[machine_type.name] || 0) + 1;
            acc.status[status.name] = (acc.status[status.name] || 0) + 1;
            acc.facility[facility.name] = (acc.facility[facility.name] || 0) + 1;
            return acc;
        },
        {
            machine_type: {},
            status: {},
            facility: {},
        },
    );

    const { pluginStates } = tableModel;
    const {
        filterValue,
    }: {
        filterValue: Writable<string>;
    } = pluginStates.filter;

    const {
        filterValues,
    }: {
        filterValues: Writable<{
            machine_type: string[];
            status: string[];
            facility: string[];
        }>;
    } = pluginStates.colFilter;

    $: showReset = Object.values({ ...$filterValues, $filterValue }).some((v) => v.length > 0);
</script>

<div class="flex items-center justify-between">
    <div class="flex flex-1 items-center space-x-2">
        <Input
            placeholder="Filter tasks..."
            class="h-8 w-[150px] lg:w-[250px]"
            type="search"
            bind:value={$filterValue}
        />

        <DataTableFacetedFilter
            bind:filterValues={$filterValues.machine_type}
            title="Type"
            options={$machineTypes}
            counts={counts.machine_type}
        />
        <DataTableFacetedFilter
            bind:filterValues={$filterValues.status}
            title="Status"
            options={$machineStatuses}
            counts={counts.status}
        />
        <DataTableFacetedFilter
            bind:filterValues={$filterValues.facility}
            title="Facilites"
            options={$facilities}
            counts={counts.facility}
        />
        {#if showReset}
            <Button
                on:click={() => {
                    $filterValue = '';
                    $filterValues.machine_type = [];
                    $filterValues.status = [];
                    $filterValues.facility = [];
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
