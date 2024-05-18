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
        due_at_converter,
    } from './common';
    import Button from '$components/ui/button/button.svelte';
    import { z } from 'zod';
    import Select from './Select.svelte';
    import SelectItem from '$components/ui/select/select-item.svelte';
    import * as HoverCard from '$lib/components/ui/hover-card/index.js';
    import { toast } from 'svelte-sonner';
    import Label from '$components/ui/label/label.svelte';
    import Checkbox from '$components/ui/checkbox/checkbox.svelte';
    import { Calendar } from '$lib/components/ui/calendar/index.js';
    import CalendarDays from 'lucide-svelte/icons/calendar-days';
    import * as Popover from '$lib/components/ui/popover/index.js';

    const selectedType = writable({ label: '', value: '' });
    const selectedStatus = writable({ label: '', value: '' });
    const selectedMachine = writable({ label: '', value: '' });
    const selectedExecutors = writable([{ label: '', value: '' }]);

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
    $: selectedMachine.set(
        $form.machine ? { label: $machines?.find((f) => f.id === $form?.machine)?.name, value: $form?.machine } : null,
    );
    $: selectedExecutors.set(
        $form?.executors?.length
            ? $form?.executors?.map((userId) => {
                  return { label: $users?.find((u) => u.id === userId)?.first_name, value: userId };
              })
            : null,
    );

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
            const { title, description, task_type, status, machine, executors, due_at, archived } = $form;
            const formObj = { title, description, task_type, status };
            if (machine) {
                formObj['machine'] = machine;
            }
            if (executors && executors instanceof Array) {
                formObj['executors'] = executors;
            }
            if (archived !== undefined && archived !== null) {
                formObj['archived'] = archived;
            }
            if (due_at !== undefined && due_at !== null) {
                formObj['due_at'] = due_at_converter(due_at);
            }
            const response = await sendJSON('/api/auth/task', 'POST', formObj);
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
            const { title, description, task_type, status, archived, executors, machine, due_at } = $form;
            console.log(changedFields);
            return;
            if (Object.keys(changedFields).length < 2) return;
            const response = await sendJSON('/api/auth/task', 'PUT', changedFields);
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

    <Input
        properties={{ id: 'description', label: 'Description' }}
        bind:value={$form.description}
        errors={$fieldErrors.description}
    />

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

    <Select
        properties={{ id: 'machine', label: 'Machine', placeholder: 'Select a machine' }}
        bind:selected={$selectedMachine}
        onSelectedChange={(opt) => {
            opt && ($form.machine = opt.value);
        }}
        errors={$fieldErrors?.machine}
    >
        {#each $machines as machine}
            <SelectItem value={machine.id} label={machine.name} />
        {/each}
    </Select>

    <Select
        properties={{ id: 'executors', label: 'Executors', placeholder: 'Pick executors' }}
        multiple={true}
        bind:selected={$selectedExecutors}
        onSelectedChange={(items) => {
            if (items) {
                $form.executors = items?.map((user) => user.value);
            } else {
                $form.executors = [];
            }
        }}
        errors={$fieldErrors?.machine}
    >
        {#each $users as user}
            <SelectItem value={user.id} label={user.first_name} />
        {/each}
    </Select>

    <div>
        <Label class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
            >Due At</Label
        ><br />
        <Popover.Root>
            <Popover.Trigger asChild let:builder>
                <Button
                    variant="outline"
                    class="w-[240px] justify-start text-left font-normal{!$form?.due_at ? 'text-muted-foreground' : ''}"
                    builders={[builder]}
                >
                    <CalendarDays class="mr-2 h-4 w-4" />
                    {$form?.due_at ? new Date($form?.due_at)?.toDateString() : 'Pick a date'}
                </Button>
            </Popover.Trigger>
            <Popover.Content class="w-auto p-0" align="start">
                <Calendar bind:value={$form.due_at} />
            </Popover.Content>
        </Popover.Root>
    </div>

    <div>
        <Checkbox id="archived" bind:checked={$form.archived} disabled={$isViewing} />
        <Label
            for="archived"
            class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
            >Archived</Label
        >
    </div>

    <div>
        {#if $task?.creator?.id}
            <HoverCard.Root>
                <HoverCard.Trigger>
                    <Link to="/user/{$task?.creator?.id}" class="ml-auto text-xs text-muted-foreground"
                        >Creator {$task?.creator?.first_name}</Link
                    >
                </HoverCard.Trigger>
                <HoverCard.Content class="w-80">
                    <div class="flex justify-between space-x-4">
                        <div class="space-y-1">
                            <h4 class="text-sm font-semibold">
                                {$task?.creator?.first_name}, {$task?.creator?.last_name}
                            </h4>
                            <a class="text-sm" href="mailto:{$task?.creator?.email}">{$task?.creator?.email}</a>
                            <div class="flex items-center pt-2">
                                <span class="text-xs text-muted-foreground"> {$task?.creator?.id} </span>
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
