<script>
    import { writable } from 'svelte/store';
    import { report, reportTypes, reportStatuses, machines } from '$stores';
    import { sendJSON, getOneTask } from '$utils';
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

    $: selectedType.set(
        $form.report_type
            ? { label: $reportTypes?.find((rt) => rt.id === $form?.report_type)?.name, value: $form?.report_type }
            : null,
    );
    $: selectedStatus.set(
        $form.status
            ? { label: $reportStatuses?.find((rs) => rs.id === $form?.status)?.name, value: $form?.status }
            : null,
    );
    $: selectedMachine.set(
        $form.machine ? { label: $machines?.find((f) => f.id === $form?.machine)?.name, value: $form?.machine } : null,
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
            };
            $hasErrors = false;
        }
    }

    let isSaving = false;

    async function saveReport() {
        if (isSaving) return;
        isSaving = true;
        switch (true) {
            case $isViewing:
                return (isSaving = false);
            case $isCreating:
                await createReport();
                break;
            case $isEditing:
                await updateReport();
                break;
        }
        isSaving = false;
    }

    async function createReport() {
        try {
            const { title, description, report_type, status, machine } = $form;
            const formObj = { title, description, report_type, status };
            if (machine) {
                formObj['machine'] = machine;
            }
            const response = await sendJSON('/api/auth/report', 'POST', formObj);
            if (response.status !== 201) return toast.error('Failed to create the report');
            const data = await response.json();
            report.set(data);
            updateUrl($report.id);
            navigate('?edit=true');
            loadFields();
            toast.success('Created the report');
        } catch (error) {
            toast.error('Failed to create the report');
        }
    }

    async function updateReport() {
        try {
            const changedFields = { id: $form?.id };
            const { title, description, report_type, status, machine } = $form;

            if (Object.keys(changedFields).length < 2) return;
            const response = await sendJSON('/api/auth/report', 'PUT', changedFields);
            if (response.status !== 200) return toast.error('Failed to update the report');
            const data = await response.json();
            report.set(data);
            navigate('?edit=true');
            loadFields();
            toast.success('Saved the report');
        } catch (error) {
            toast.error('Failed to update the report');
        }
    }
</script>

<form on:submit|preventDefault={saveReport} class="space-y-4 w-full md:w-auto">
    <Input properties={{ id: 'id', label: 'Id' }} bind:value={$form.id} disabled={true} />

    <Input properties={{ id: 'title', label: 'Title' }} bind:value={$form.title} errors={$fieldErrors.title} />

    <Input
        properties={{ id: 'description', label: 'Description' }}
        bind:value={$form.description}
        errors={$fieldErrors.description}
    />

    <Select
        properties={{ id: 'report_type', label: 'Type', placeholder: 'Select a type' }}
        bind:selected={$selectedType}
        onSelectedChange={(opt) => {
            opt && ($form.report_type = opt.value);
        }}
        errors={$fieldErrors?.report_type}
    >
        {#each $reportTypes as reportType}
            <SelectItem value={reportType.id} label={reportType.name} />
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
        {#each $reportStatuses as reportStatus}
            <SelectItem value={reportStatus.id} label={reportStatus.name} />
        {/each}
    </Select>

    <Select
        properties={{ id: 'machine', label: 'Machine', placeholder: 'Select a machine' }}
        bind:selected={$selectedMachine}
        onSelectedChange={(opt) => {
            opt && ($form.machine = opt.value);
        }}
    >
        {#each $machines as machine}
            <SelectItem value={machine.id} label={machine.name} />
        {/each}
    </Select>

    <div>
        {#if $report?.creator?.id}
            <HoverCard.Root>
                <HoverCard.Trigger>
                    <Link to="/user/{$report?.creator?.id}" class="ml-auto text-xs text-muted-foreground"
                        >Creator {$report?.creator?.first_name}</Link
                    >
                </HoverCard.Trigger>
                <HoverCard.Content class="w-80">
                    <div class="flex justify-between space-x-4">
                        <div class="space-y-1">
                            <h4 class="text-sm font-semibold">
                                {$report?.creator?.first_name}, {$report?.creator?.last_name}
                            </h4>
                            <a class="text-sm" href="mailto:{$report?.creator?.email}">{$report?.creator?.email}</a>
                            <div class="flex items-center pt-2">
                                <span class="text-xs text-muted-foreground"> {$report?.creator?.id} </span>
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
