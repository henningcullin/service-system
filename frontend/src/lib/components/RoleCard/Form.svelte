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
    import { role, roles } from '$stores';
    import Separator from '$components/ui/separator/separator.svelte';
    import Checkbox from './Checkbox.svelte';
    import { Label } from '$components/ui/label';

    $: {
        if (!$isViewing) {
            try {
                formSchema.parse($form);
                $fieldErrors = { name: '', level: '' };
                $hasErrors = false;
            } catch (e) {
                if (e instanceof z.ZodError) {
                    // @ts-ignore
                    $fieldErrors = e.flatten().fieldErrors;
                    $hasErrors = true;
                }
            }
        } else {
            $fieldErrors = { name: '', level: '' };
            $hasErrors = false;
        }
    }

    let isSaving = false;

    async function saveRole() {
        if (isSaving) return;
        isSaving = true;
        switch (true) {
            case $isViewing:
                return (isSaving = false);
            case $isCreating:
                await createRole();
                break;
            case $isEditing:
                await updateRole();
                break;
        }
        isSaving = false;
    }

    async function createRole() {
        try {
            typeof $form?.level === 'number' ? null : ($form.level = Number($form?.level));
            const response = await sendJSON('/api/auth/role', 'POST', { $form });
            if (response.status !== 201) return toast.error('Failed to create the role');
            const data = await response.json();
            role.set(data);
            roles.update((prev) => {
                prev.push({ ...$role });
                return prev;
            });
            updateUrl($role.id);
            navigate('?view=true');
            loadFields();
            toast.success('Created the role');
        } catch (error) {
            toast.error('Failed to create the role');
        }
    }

    async function updateRole() {
        try {
            const changedFields = { id: $form.id };
            for (const field in $form) {
                if ($form[field] !== $role[field]) {
                    changedFields[field] = $form[field];
                }
            }
            if (typeof changedFields['level'] === 'string') changedFields['level'] = Number(changedFields['level']);
            if (Object.keys(changedFields).length < 2) return;
            const response = await sendJSON('/api/auth/role', 'PUT', changedFields);
            if (response.status !== 204) return toast.error('Failed to update the role');
            role.set({ ...$form });
            const oldIndex = $roles.findIndex((f) => f.id === $role.id);
            roles.update((prev) => {
                prev[oldIndex] = { ...$role };
                return prev;
            });
            navigate('?view=true');
            loadFields();
            toast.success('Saved the role');
        } catch (error) {
            toast.error('Failed to update the role');
        }
    }
</script>

<form on:submit|preventDefault={saveRole} class="space-y-4 w-full md:w-auto">
    <Input properties={{ id: 'id', label: 'Id' }} bind:value={$form.id} disabled={true} />

    <Input properties={{ id: 'name', label: 'Name' }} bind:value={$form.name} errors={$fieldErrors.name} />

    <Input
        properties={{ id: 'level', label: 'Level' }}
        bind:value={$form.level}
        errors={$fieldErrors.level}
        type="number"
    />

    <Checkbox label="Has Password" bind:checked={$form.has_password} />

    <Separator />

    <Label>User</Label><br />

    <Checkbox label="View" bind:checked={$form.user_view} />
    <Checkbox label="Create" bind:checked={$form.user_create} />
    <Checkbox label="Edit" bind:checked={$form.user_edit} />
    <Checkbox label="Delete" bind:checked={$form.user_delete} />

    <Separator />

    <Label>Machine</Label><br />
    <Checkbox label="View" bind:checked={$form.machine_view} />
    <Checkbox label="Create" bind:checked={$form.machine_create} />
    <Checkbox label="Edit" bind:checked={$form.machine_edit} />
    <Checkbox label="Delete" bind:checked={$form.machine_delete} />

    <Separator />

    <Label>Task</Label><br />
    <Checkbox label="View" bind:checked={$form.task_view} />
    <Checkbox label="Create" bind:checked={$form.task_create} />
    <Checkbox label="Edit" bind:checked={$form.task_edit} />
    <Checkbox label="Delete" bind:checked={$form.task_delete} />
    <Separator />

    <Label>Report</Label><br />
    <Checkbox label="View" bind:checked={$form.report_view} />
    <Checkbox label="Create" bind:checked={$form.report_create} />
    <Checkbox label="Edit" bind:checked={$form.report_edit} />
    <Checkbox label="Delete" bind:checked={$form.report_delete} />
    <Separator />

    <Label>Facility</Label><br />
    <Checkbox label="View" bind:checked={$form.facility_view} />
    <Checkbox label="Create" bind:checked={$form.facility_create} />
    <Checkbox label="Edit" bind:checked={$form.facility_edit} />
    <Checkbox label="Delete" bind:checked={$form.facility_delete} />

    <Button type="submit" disabled={$isViewing || $hasErrors}>Save</Button>
</form>
