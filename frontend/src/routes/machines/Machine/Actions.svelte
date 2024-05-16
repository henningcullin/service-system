<script lang="ts ">
</script>

<div class="flex space-x-4">
    <Button
        on:click={() => {
            navigate('?new=true');
            clearFields();
        }}
        disabled={isCreating}
        variant="outline">New</Button
    >
    <Button
        on:click={() => {
            navigate('?edit=true');
            if (id) loadFields();
        }}
        disabled={isEditing || !id}
        variant="outline">Edit</Button
    >
    <Button
        on:click={() => {
            deleteDialogOpen = true;
        }}
        disabled={!id}
        variant="destructive">Delete</Button
    >
    <Button
        on:click={() => {
            navigate('?view=true');
            if (id) loadFields();
        }}
        disabled={isViewing}
        variant="outline">Cancel</Button
    >
</div>
