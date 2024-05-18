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
    import { report } from '$stores';
    import { toast } from 'svelte-sonner';
    import { sendDelete } from '$utils';

    async function deleteReport() {
        try {
            const response = await sendDelete(`/api/auth/report?id=${$id}`);
            if (response.status !== 204) return toast.error('Failed to delete report');
            report.set({});
            clearFields();
            navigate('?view=true');
            updateUrl();
            toast.success('Deleted the report');
        } catch (error) {
            toast.error('Failed to delete report');
        }
    }
</script>

<div class="flex space-x-4">
    <Button
        on:click={() => {
            navigate('?new=true');
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
                This action cannot be undone. This will permanently delete the task from our servers.
            </AlertDialog.Description>
        </AlertDialog.Header>
        <AlertDialog.Footer>
            <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
            <AlertDialog.Action on:click={deleteReport}>Continue</AlertDialog.Action>
        </AlertDialog.Footer>
    </AlertDialog.Content>
</AlertDialog.Root>
