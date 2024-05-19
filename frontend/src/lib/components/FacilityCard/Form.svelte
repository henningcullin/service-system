<script>
    import { sendJSON } from '$utils';
    import { navigate } from 'svelte-navigator';
    import Input from './Input.svelte';
    import {
        fieldErrors,
        hasErrors,
        form,
        isCreating,
        isEditing,
        isViewing,
        loadFields,
        updateUrl,
        formSchema,
    } from './common';
    import Button from '$components/ui/button/button.svelte';
    import { z } from 'zod';
    import { toast } from 'svelte-sonner';
    import { facilities, facility } from '$stores';

    $: {
        if (!$isViewing) {
            try {
                formSchema.parse($form);
                $fieldErrors = { name: '' };
                $hasErrors = false;
            } catch (e) {
                if (e instanceof z.ZodError) {
                    // @ts-ignore
                    $fieldErrors = e.flatten().fieldErrors;
                    $hasErrors = true;
                }
            }
        } else {
            $fieldErrors = { name: '' };
            $hasErrors = false;
        }
    }

    let isSaving = false;

    async function saveFacility() {
        if (isSaving) return;
        isSaving = true;
        switch (true) {
            case $isViewing:
                return (isSaving = false);
            case $isCreating:
                await createFacility();
                break;
            case $isEditing:
                await updateFacility();
                break;
        }
        isSaving = false;
    }

    async function createFacility() {
        try {
            const { name, address } = $form;
            const response = await sendJSON('/api/auth/facility', 'POST', {
                name,
                address,
            });
            if (response.status !== 201) return toast.error('Failed to create the facility');
            const data = await response.json();
            facility.set(data);
            facilities.update((prev) => {
                prev.push({ ...$facility });
                return prev;
            });
            updateUrl($facility.id);
            navigate('?view=true');
            loadFields();
            toast.success('Created the facility');
        } catch (error) {
            toast.error('Failed to create the facility');
        }
    }

    async function updateFacility() {
        try {
            const changedFields = { id: $form.id };
            for (const field in $form) {
                if ($form[field] !== $facility[field]) {
                    changedFields[field] = $form[field];
                }
            }
            if (Object.keys(changedFields).length < 2) return;
            const response = await sendJSON('/api/auth/facility', 'PUT', changedFields);
            if (response.status !== 204) return toast.error('Failed to update the facility');
            facility.set({ ...$form });
            const oldIndex = $facilities.findIndex((f) => f.id === $facility.id);
            facilities.update((prev) => {
                prev[oldIndex] = { ...$facility };
                return prev;
            });
            navigate('?view=true');
            loadFields();
            toast.success('Saved the facility');
        } catch (error) {
            toast.error('Failed to update the facility');
        }
    }
</script>

<form on:submit|preventDefault={saveFacility} class="space-y-4 w-full md:w-auto">
    <Input properties={{ id: 'id', label: 'Id' }} bind:value={$form.id} disabled={true} />

    <Input properties={{ id: 'name', label: 'Name' }} bind:value={$form.name} errors={$fieldErrors.name} />

    <Input properties={{ id: 'address', label: 'Address' }} bind:value={$form.address} />

    <Button type="submit" disabled={$isViewing || $hasErrors}>Save</Button>
</form>
