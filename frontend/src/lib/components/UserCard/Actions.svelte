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
    import { user, account } from '$stores';
    import { toast } from 'svelte-sonner';
    import { sendDelete } from '$utils';

    async function deleteUser() {
        try {
            const response = await sendDelete(`/api/auth/user?id=${$id}`);
            if (response.status !== 204) return toast.error('Failed to delete user');
            user.set({});
            clearFields();
            navigate('?view=true');
            updateUrl();
            toast.success('Deleted the user');
        } catch (error) {
            toast.error('Failed to delete user');
        }
    }
</script>

<div class="flex space-x-4">
    <Button
        on:click={() => {
            navigate('?new=true');
            clearFields();
        }}
        disabled={$isCreating || !$account?.role?.user_create}
        variant="outline">New</Button
    >
    <Button
        on:click={() => {
            navigate('?edit=true');
            if ($id) loadFields();
        }}
        disabled={$isEditing || !$id || !$account?.role?.user_edit}
        variant="outline">Edit</Button
    >
    <Button
        on:click={() => {
            $deleteDialogOpen = true;
        }}
        disabled={!$id || !$account?.role?.user_delete}
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
                This action cannot be undone. This will permanently remove all references to this user.
            </AlertDialog.Description>
        </AlertDialog.Header>
        <AlertDialog.Footer>
            <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
            <AlertDialog.Action on:click={deleteUser}>Continue</AlertDialog.Action>
        </AlertDialog.Footer>
    </AlertDialog.Content>
</AlertDialog.Root>
