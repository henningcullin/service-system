<script lang="ts">
    import EyeOff from 'lucide-svelte/icons/eye-off';
    import ChevronDown from 'lucide-svelte/icons/chevron-down';
    import ChevronUp from 'lucide-svelte/icons/chevron-up';
    import ChevronsUpDown from 'lucide-svelte/icons/chevrons-up-down';
    import type { TableViewModel } from 'svelte-headless-table';
    import type { Task } from './schema.ts';
    import { Button } from '$lib/components/ui/button/index.js';
    import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';

    export let props: {
        select: never;
        sort: {
            order: 'desc' | 'asc' | undefined;
            toggle: (_: Event) => void;
            clear: () => void;
            disabled: boolean;
        };
        filter: never;
    };
    export let tableModel: TableViewModel<Task>;
    export let cellId: string;

    const { hiddenColumnIds } = tableModel.pluginStates.hide;

    function handleAscSort(e: Event) {
        if (props.sort.order === 'asc') {
            return;
        }
        props.sort.toggle(e);
    }

    function handleDescSort(e: Event) {
        if (props.sort.order === 'desc') {
            return;
        }
        if (props.sort.order === undefined) {
            // We can only toggle, so we toggle from undefined to 'asc' first
            props.sort.toggle(e);
        }
        props.sort.toggle(e); // Then we toggle from 'asc' to 'desc'
    }

    function handleHide() {
        hiddenColumnIds.update((ids: string[]) => {
            if (ids.includes(cellId)) {
                return ids;
            }
            return [...ids, cellId];
        });
    }
</script>

{#if !props.sort.disabled}
    <div class="flex items-center">
        <DropdownMenu.Root>
            <DropdownMenu.Trigger asChild let:builder>
                <Button variant="ghost" builders={[builder]} class="-ml-3 h-8 data-[state=open]:bg-accent" size="sm">
                    <slot />
                    {#if props.sort.order === 'desc'}
                        <ChevronDown class="ml-2 h-4 w-4" />
                    {:else if props.sort.order === 'asc'}
                        <ChevronUp class="ml-2 h-4 w-4" />
                    {:else}
                        <ChevronsUpDown class="ml-2 h-4 w-4" />
                    {/if}
                </Button>
            </DropdownMenu.Trigger>
            <DropdownMenu.Content align="start">
                <DropdownMenu.Item on:click={handleAscSort}>
                    <ChevronUp class="mr-2 h-3.5 w-3.5 text-muted-foreground/70" />
                    Asc
                </DropdownMenu.Item>
                <DropdownMenu.Item on:click={handleDescSort}>
                    <ChevronDown class="mr-2 h-3.5 w-3.5 text-muted-foreground/70" />
                    Desc
                </DropdownMenu.Item>
                <DropdownMenu.Separator />
                <DropdownMenu.Item on:click={handleHide}>
                    <EyeOff class="mr-2 h-3.5 w-3.5 text-muted-foreground/70" />
                    Hide
                </DropdownMenu.Item>
            </DropdownMenu.Content>
        </DropdownMenu.Root>
    </div>
{:else}
    <slot />
{/if}
