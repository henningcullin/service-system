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
    import { role, roles } from '$stores';
    import { toast } from 'svelte-sonner';
    import { sendDelete } from '$utils';

    async function deleteRole() {
        try {
            const response = await sendDelete(`/api/auth/role?id=${$id}`);
            if (response.status !== 204) return toast.error('Failed to delete role');
            roles.update((prev) => prev.filter((f) => f.id !== $role?.id));
            role.set({});
            clearFields();
            navigate('?view=true');
            updateUrl();
            toast.error('Deleted the role');
        } catch (error) {
            toast.error('Failed to delete role');
        }
    }
</script>

<div class="flex space-x-4">
    <Button
        on:click={() => {
            navigate('?new=true');
            role.set({});
            clearFields();
        }}
        disabled={$isCreating}
        variant="outline">New</Button
    >
    <Button
        on:click={() => {
            navigate('?edit=true');
            if ($id) loadFields();
        }}
        disabled={$isEditing || !$id}
        variant="outline">Edit</Button
    >
    <Button
        on:click={() => {
            $deleteDialogOpen = true;
        }}
        disabled={!$id}
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
                This action cannot be undone. This will permanently delete the role, all users with the role, and all
                work related to those users from our servers.
            </AlertDialog.Description>
        </AlertDialog.Header>
        <AlertDialog.Footer>
            <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
            <AlertDialog.Action on:click={deleteRole}>Continue</AlertDialog.Action>
        </AlertDialog.Footer>
    </AlertDialog.Content>
</AlertDialog.Root>
