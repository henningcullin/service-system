<script lang="ts">
    import SlidersHorizontal from 'lucide-svelte/icons/sliders-horizontal';
    import type { TableViewModel } from 'svelte-headless-table';
    import type { Task } from './schema';
    import { Button } from '$lib/components/ui/button/index.js';
    import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';

    export let tableModel: TableViewModel<Task>;
    const { pluginStates, flatColumns } = tableModel;
    const { hiddenColumnIds } = pluginStates.hide;

    function handleHide(id: string) {
        hiddenColumnIds.update((ids: string[]) => {
            if (ids.includes(id)) {
                return ids.filter((i) => i !== id);
            }
            return [...ids, id];
        });
    }

    const hidableCols = [
        'title',
        'description',
        'task_type',
        'status',
        'machine',
        'creator',
        'executors',
        'due_at',
        'created',
        'edited',
    ];
</script>

<DropdownMenu.Root>
    <DropdownMenu.Trigger asChild let:builder>
        <Button variant="outline" size="sm" class="ml-auto hidden h-8 lg:flex" builders={[builder]}>
            <SlidersHorizontal class="mr-2 h-4 w-4" />
            View
        </Button>
    </DropdownMenu.Trigger>
    <DropdownMenu.Content>
        <DropdownMenu.Label>Toggle columns</DropdownMenu.Label>
        <DropdownMenu.Separator />
        {#each flatColumns as col}
            {#if hidableCols.includes(col.id)}
                <DropdownMenu.CheckboxItem
                    checked={!$hiddenColumnIds.includes(col.id)}
                    on:click={() => handleHide(col.id)}
                >
                    {col.header}
                </DropdownMenu.CheckboxItem>
            {/if}
        {/each}
    </DropdownMenu.Content>
</DropdownMenu.Root>
