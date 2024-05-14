<script>
    import EyeNone from 'svelte-radix/EyeNone.svelte';
    import ArrowDown from 'svelte-radix/ArrowDown.svelte';
    import ArrowUp from 'svelte-radix/ArrowUp.svelte';
    import CaretSort from 'svelte-radix/CaretSort.svelte';
    import { Button } from '$lib/components/ui/button/index.js';
    import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';

    export let props;
    /**
     * @type TableViewModel<Task>
     */
    export let tableModel;
    /**
     * @type string
     */
    export let cellId;

    const { hiddenColumnIds } = tableModel.pluginStates.hide;

    function handleAscSort(e) {
        if (props.sort.order === 'asc') {
            return;
        }
        props.sort.toggle(e);
    }

    function handleDescSort(e) {
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
        hiddenColumnIds.update((ids) => {
            if (ids.includes(cellId)) {
                return ids;
            }
            return [...ids, cellId];
        });
    }
</script>

{#if !props.sort.disabled}
    <div class={'flex items-center'}>
        <DropdownMenu.Root>
            <DropdownMenu.Trigger asChild let:builder>
                <Button variant="ghost" builders={[builder]} class="-ml-3 h-8 data-[state=open]:bg-accent" size="sm">
                    <slot />
                    {#if props.sort.order === 'desc'}
                        <ArrowDown class="ml-2 h-4 w-4" />
                    {:else if props.sort.order === 'asc'}
                        <ArrowUp class="ml-2 h-4 w-4" />
                    {:else}
                        <CaretSort class="ml-2 h-4 w-4" />
                    {/if}
                </Button>
            </DropdownMenu.Trigger>
            <DropdownMenu.Content align="start">
                <DropdownMenu.Item on:click={handleAscSort}>
                    <ArrowUp class="mr-2 h-3.5 w-3.5 text-muted-foreground/70" />
                    Asc
                </DropdownMenu.Item>
                <DropdownMenu.Item on:click={handleDescSort}>
                    <ArrowDown class="mr-2 h-3.5 w-3.5 text-muted-foreground/70" />
                    Desc
                </DropdownMenu.Item>
                <DropdownMenu.Separator />
                <DropdownMenu.Item on:click={handleHide}>
                    <EyeNone class="mr-2 h-3.5 w-3.5 text-muted-foreground/70" />
                    Hide
                </DropdownMenu.Item>
            </DropdownMenu.Content>
        </DropdownMenu.Root>
    </div>
{:else}
    <slot />
{/if}
