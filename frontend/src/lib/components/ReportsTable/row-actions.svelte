<script>
    import Ellipsis from 'lucide-svelte/icons/ellipsis';
    import { Button } from '$components/ui/button/index.js';
    import * as DropdownMenu from '$components/ui/dropdown-menu/index.js';
    import * as AlertDialog from '$components/ui/alert-dialog/index.js';
    import { toast } from 'svelte-sonner';
    import { sendDelete } from '$utils';
    import { reports } from '$stores';
    import { Link } from 'svelte-navigator';

    async function deleteReport() {
        try {
            const response = await sendDelete(`/api/auth/report?id=${row.id}`);
            if (response.status !== 204) return toast.error('Could not delete the report');
            reports.update((prev) => prev.filter((r) => r.id !== row?.id));
            toast.success('Deleted the report');
        } catch (error) {
            toast.error('Could not delete the report');
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
            <Link style="height:100%;width:100%;" to="/report/{row.id}">View</Link>
        </DropdownMenu.Item>
        <DropdownMenu.Item>
            <Link style="height:100%;width:100%;" to="/report/{row.id}?edit=true">Edit</Link>
        </DropdownMenu.Item>
        <DropdownMenu.Separator />
        <DropdownMenu.Item on:click={() => (deleteDialogOpen = true)}>Delete</DropdownMenu.Item>
    </DropdownMenu.Content>
</DropdownMenu.Root>

<AlertDialog.Root bind:open={deleteDialogOpen}>
    <AlertDialog.Content>
        <AlertDialog.Header>
            <AlertDialog.Title>Are you absolutely sure?</AlertDialog.Title>
            <AlertDialog.Description>
                This action cannot be undone. This will permanently delete the report from our servers.
            </AlertDialog.Description>
        </AlertDialog.Header>
        <AlertDialog.Footer>
            <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
            <AlertDialog.Action on:click={deleteReport}>Continue</AlertDialog.Action>
        </AlertDialog.Footer>
    </AlertDialog.Content>
</AlertDialog.Root>
