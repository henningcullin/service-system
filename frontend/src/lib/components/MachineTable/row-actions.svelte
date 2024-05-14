<script>
    import { Ellipsis } from 'lucide-svelte';
    import { Button } from '$lib/components/ui/button/index.js';
    import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';

    export let row;
    const task = taskSchema.parse(row);
</script>

<DropdownMenu.Root>
    <DropdownMenu.Trigger asChild let:builder>
        <Button variant="ghost" builders={[builder]} class="flex h-8 w-8 p-0 data-[state=open]:bg-muted">
            <Ellipsis class="h-4 w-4" />
            <span class="sr-only">Open Menu</span>
        </Button>
    </DropdownMenu.Trigger>
    <DropdownMenu.Content class="w-[160px]" align="end">
        <DropdownMenu.Item>View</DropdownMenu.Item>
        <DropdownMenu.Item>Edit</DropdownMenu.Item>
        <DropdownMenu.Separator />
        <DropdownMenu.Item>
            Delete
            <DropdownMenu.Shortcut>⌘⌫</DropdownMenu.Shortcut>
        </DropdownMenu.Item>
    </DropdownMenu.Content>
</DropdownMenu.Root>
