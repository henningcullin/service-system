<script lang="ts ">
    import Button from '$components/ui/button/button.svelte';
    import * as AlertDialog from '$components/ui/alert-dialog/index.js';
    import { navigate } from 'svelte-navigator';
    import { clearFields, deleteDialogOpen, form, loadFields, isCreating, isEditing, isViewing, id } from './common';
    import { machine } from '$stores';

    async function deleteMachine() {
        try {
            const response = await sendDelete(`/api/auth/machine?id=${$id}`);
            if (response.status !== 204) return alert('Failed to delete machine');
            machine.set({});
            clearFields($form);
            navigate('?view=true');
            updateUrl();
        } catch (error) {
            console.error(error);
        }
    }
</script>

<div class="flex space-x-4">
    <Button
        on:click={() => {
            navigate('?new=true');
            clearFields($form);
        }}
        disabled={$isCreating}
        variant="outline">New</Button
    >
    <Button
        on:click={() => {
            navigate('?edit=true');
            if ($id) loadFields($form, $machine);
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
                This action cannot be undone. This will permanently delete the machine from our servers.
            </AlertDialog.Description>
        </AlertDialog.Header>
        <AlertDialog.Footer>
            <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
            <AlertDialog.Action on:click={deleteMachine}>Continue</AlertDialog.Action>
        </AlertDialog.Footer>
    </AlertDialog.Content>
</AlertDialog.Root>
