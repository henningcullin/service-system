<script lang="ts ">
    import Button from '$components/ui/button/button.svelte';
    import * as AlertDialog from '$components/ui/alert-dialog/index.js';
    import { navigate } from 'svelte-navigator';
    import {
        clearFields,
        deleteDialogOpen,
        loadFields,
        isCreating,
        isEditing,
        isViewing,
        id,
        updateUrl,
    } from './common';
    import { account, machine } from '$stores';
    import { toast } from 'svelte-sonner';
    import { sendDelete } from '$utils';

    async function deleteMachine() {
        try {
            const response = await sendDelete(`/api/auth/machine?id=${$id}`);
            if (response.status !== 204) return toast.error('Failed to delete machine');
            machine.set({});
            clearFields();
            navigate('?view=true');
            updateUrl();
            toast.success('Deleted the machine');
        } catch (error) {
            toast.error('Failed to delete machine');
        }
    }
</script>

<div class="flex space-x-4">
    <Button
        on:click={() => {
            navigate('?new=true');
            clearFields();
        }}
        disabled={$isCreating || !$account?.role?.machine_create}
        variant="outline">New</Button
    >
    <Button
        on:click={() => {
            navigate('?edit=true');
            if ($id) loadFields();
        }}
        disabled={$isEditing || !$id || !$account?.role?.machine_edit}
        variant="outline">Edit</Button
    >
    <Button
        on:click={() => {
            $deleteDialogOpen = true;
        }}
        disabled={!$id || !$account?.role?.machine_delete}
        variant="destructive">Delete</Button
    >
    <Button
        on:click={() => {
            navigate('?view=true');
            if ($id) loadFields();
        }}
        disabled={$isViewing}
        variant="outline">Cancel</Button
    >
</div>

<AlertDialog.Root bind:open={$deleteDialogOpen}>
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
