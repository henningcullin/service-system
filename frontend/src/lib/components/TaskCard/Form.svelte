<script lang="ts">
    import { writable } from 'svelte/store';
    import { task, taskTypes, taskStatuses, users, machines } from '$stores';
    import { sendJSON } from '$utils';
    import { navigate, Link } from 'svelte-navigator';
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
    import Select from './Select.svelte';
    import SelectItem from '$components/ui/select/select-item.svelte';
    import * as HoverCard from '$lib/components/ui/hover-card/index.js';
    import { toast } from 'svelte-sonner';

    const selectedType = writable({ label: '', value: '' });
    const selectedStatus = writable({ label: '', value: '' });
    /* const selectedFacility = writable({ label: '', value: '' }); */

    $: selectedType.set(
        $form.task_type
            ? { label: $taskTypes?.find((mt) => mt.id === $form?.task_type)?.name, value: $form?.task_type }
            : null,
    );
    $: selectedStatus.set(
        $form.status
            ? { label: $taskStatuses?.find((ms) => ms.id === $form?.status)?.name, value: $form?.status }
            : null,
    );
    /*     $: selectedFacility.set(
        $form.facility
            ? { label: $facilities?.find((f) => f.id === $form?.facility)?.name, value: $form?.facility }
            : null,
    ); */

    $: {
        if (!$isViewing) {
            try {
                formSchema.parse($form);
                $fieldErrors = {
                    title: '',
                    description: '',
                    task_type: '',
                    status: '',
                    archived: '',
                    executors: '',
                    machine: '',
                    due_at: '',
                };
                $hasErrors = false;
            } catch (e) {
                if (e instanceof z.ZodError) {
                    // @ts-ignore
                    $fieldErrors = e.flatten().fieldErrors;
                    $hasErrors = true;
                }
            }
        } else {
            $fieldErrors = {
                title: '',
                description: '',
                task_type: '',
                status: '',
                archived: '',
                executors: '',
                machine: '',
                due_at: '',
            };
            $hasErrors = false;
        }
    }

    let isSaving = false;

    async function saveTask() {
        if (isSaving) return;
        isSaving = true;
        switch (true) {
            case $isViewing:
                return (isSaving = false);
            case $isCreating:
                await createTask();
                break;
            case $isEditing:
                await updateTask();
                break;
        }
        isSaving = false;
    }

    async function createTask() {
        try {
            const { title, description, task_type, status } = $form;
            const response = await sendJSON('/api/auth/machine', 'POST', {
                title,
                description,
                task_type,
                status,
            });
            if (response.status !== 201) return toast.error('Failed to create the task');
            const data = await response.json();
            task.set(data);
            updateUrl($task.id);
            navigate('?view=true');
            loadFields();
        } catch (error) {
            toast.error('Failed to create the task');
        }
    }

    async function updateTask() {
        try {
            const changedFields = { id: $form.id };
            for (const field in $form) {
                if ($form[field] !== $task[field] && field !== 'executors') {
                    changedFields[field] = $form[field];
                }
            }
            if (Object.keys(changedFields).length < 2) return;
            const response = await sendJSON('/api/auth/machine', 'PUT', changedFields);
            if (response.status !== 200) return toast.error('Failed to update the task');
            const data = await response.json();
            task.set(data);
            navigate('?view=true');
            loadFields();
        } catch (error) {
            toast.error('Failed to update the task');
        }
    }
</script>

<form on:submit|preventDefault={saveTask} class="space-y-4 w-full md:w-auto">
    <Input properties={{ id: 'id', label: 'Id' }} bind:value={$form.id} disabled={true} />

    <Input properties={{ id: 'title', label: 'Title' }} bind:value={$form.title} errors={$fieldErrors.title} />

    <Input properties={{ id: 'description', label: 'Description' }} bind:value={$form.description} />

    <Select
        properties={{ id: 'machine_type', label: 'Type', placeholder: 'Select a type' }}
        bind:selected={$selectedType}
        onSelectedChange={(opt) => {
            opt && ($form.task_type = opt.value);
        }}
        errors={$fieldErrors?.task_type}
    >
        {#each $taskTypes as taskType}
            <SelectItem value={taskType.id} label={taskType.name} />
        {/each}
    </Select>

    <Select
        properties={{ id: 'status', label: 'Status', placeholder: 'Select a status' }}
        bind:selected={$selectedStatus}
        onSelectedChange={(opt) => {
            opt && ($form.status = opt.value);
        }}
        errors={$fieldErrors?.status}
    >
        {#each $taskStatuses as taskStatus}
            <SelectItem value={taskStatus.id} label={taskStatus.name} />
        {/each}
    </Select>

    <div>
        {#if $form?.creator?.id}
            <HoverCard.Root>
                <HoverCard.Trigger>
                    <Link to="/user/{$form?.creator?.id}" class="ml-auto text-xs text-muted-foreground"
                        >Creator {$form?.creator?.first_name}</Link
                    >
                </HoverCard.Trigger>
                <HoverCard.Content class="w-80">
                    <div class="flex justify-between space-x-4">
                        <div class="space-y-1">
                            <h4 class="text-sm font-semibold">
                                {$form?.creator?.first_name}, {$form?.creator?.last_name}
                            </h4>
                            <a class="text-sm" href="mailto:{$form?.creator?.email}">{$form?.creator?.email}</a>
                            <div class="flex items-center pt-2">
                                <span class="text-xs text-muted-foreground"> {$form?.creator?.id} </span>
                            </div>
                        </div>
                    </div>
                </HoverCard.Content>
            </HoverCard.Root>
        {/if}
        <div class="ml-auto text-xs text-muted-foreground">Created {$form.created}</div>
        <div class="ml-auto text-xs text-muted-foreground pt-2">Edited {$form.edited}</div>
    </div>

    <Button type="submit" disabled={$isViewing || $hasErrors}>Save</Button>
</form>
