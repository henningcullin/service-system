<script>
    import Ellipsis from 'lucide-svelte/icons/ellipsis';
    import { Button } from '$components/ui/button/index.js';
    import * as DropdownMenu from '$components/ui/dropdown-menu/index.js';
    import * as AlertDialog from '$components/ui/alert-dialog/index.js';
    import { toast } from 'svelte-sonner';
    import { sendDelete } from '$utils';
    import { account, machines } from '$stores';
    import { Link } from 'svelte-navigator';

    async function deleteMachine() {
        try {
            const response = await sendDelete(`/api/auth/machine?id=${row.id}`);
            if (response.status !== 204) return toast.error('Could not delete the machine');
            machines.update((prev) => prev.filter((m) => m.id !== row.id));
            toast.success('Deleted the machine');
        } catch (error) {
            toast.error('Could not delete the machine');
            console.error(error);
        }
    }

    let deleteDialogOpen = false;

    export let row;
</script>

<DropdownMenu.Root>
    <DropdownMenu.Trigger asChild let:builder>
        <Button variant="ghost" builders={[builder]} class="flex h-8 w-8 p-0 data-[state=open]:bg-muted">
            <Ellipsis class="h-4 w-4" />
            <span class="sr-only">Open Menu</span>
        </Button>
    </DropdownMenu.Trigger>
    <DropdownMenu.Content class="w-[160px]" align="end">
        <DropdownMenu.Item>
            <Link style="height:100%;width:100%;" to="/machine/{row.id}">View</Link>
        </DropdownMenu.Item>
        {#if $account?.role?.machine_edit}
            <DropdownMenu.Item>
                <Link style="height:100%;width:100%;" to="/machine/{row.id}?edit=true">Edit</Link>
            </DropdownMenu.Item>
        {/if}
        <DropdownMenu.Separator />
        {#if $account?.role?.machine_delete}
            <DropdownMenu.Item on:click={() => (deleteDialogOpen = true)}>Delete</DropdownMenu.Item>
        {/if}
    </DropdownMenu.Content>
</DropdownMenu.Root>

<AlertDialog.Root bind:open={deleteDialogOpen}>
    <AlertDialog.Content>
        <AlertDialog.Header>
            <AlertDialog.Title>Are you absolutely sure?</AlertDialog.Title>
            <AlertDialog.Description>
                This action cannot be undone. This will permanently delete the machine from our servers.
            </AlertDialog.Description>
        </AlertDialog.Header>
        <AlertDialog.Footer>
            <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
            <AlertDialog.Action on:click={deleteMachine}>Continue</AlertDialog.Action>
        </AlertDialog.Footer>
    </AlertDialog.Content>
</AlertDialog.Root>
