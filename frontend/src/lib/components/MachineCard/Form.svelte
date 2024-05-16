<script lang="ts">
    import { writable } from 'svelte/store';
    import { machine, machineTypes, machineStatuses, facilities } from '$stores';
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
    import Select from './Select.svelte';
    import SelectItem from '$components/ui/select/select-item.svelte';

    const selectedType = writable({ label: '', value: '' });
    const selectedStatus = writable({ label: '', value: '' });
    const selectedFacility = writable({ label: '', value: '' });

    $: selectedType.set(
        $form.machine_type
            ? { label: $machineTypes?.find((mt) => mt.id === $form?.machine_type)?.name, value: $form?.machine_type }
            : null,
    );
    $: selectedStatus.set(
        $form.status
            ? { label: $machineStatuses?.find((ms) => ms.id === $form?.status)?.name, value: $form?.status }
            : null,
    );
    $: selectedFacility.set(
        $form.facility
            ? { label: $facilities?.find((f) => f.id === $form?.facility)?.name, value: $form?.facility }
            : null,
    );

    $: {
        if (!$isViewing) {
            try {
                formSchema.parse($form);
                $fieldErrors = { name: '', make: '', machine_type: '', status: '', facility: '' };
                $hasErrors = false;
            } catch (e) {
                if (e instanceof z.ZodError) {
                    // @ts-ignore
                    $fieldErrors = e.flatten().fieldErrors;
                    $hasErrors = true;
                }
            }
        } else {
            $fieldErrors = { name: '', make: '', machine_type: '', status: '', facility: '' };
            $hasErrors = false;
        }
    }

    let isSaving = false;

    async function saveMachine() {
        if (isSaving) return;
        isSaving = true;
        switch (true) {
            case $isViewing:
                return (isSaving = false);
            case $isCreating:
                await createMachine();
                break;
            case $isEditing:
                await updateMachine();
                break;
        }
        isSaving = false;
    }

    async function createMachine() {
        try {
            const { name, make, machine_type, status, facility } = $form;
            const response = await sendJSON('/api/auth/machine', 'POST', {
                name,
                make,
                machine_type,
                status,
                facility,
            });
            if (response.status !== 201) return alert('Failed to create the machine');
            const data = await response.json();
            machine.set(data);
            updateUrl($machine.id);
            navigate('?view=true');
            loadFields();
        } catch (error) {
            console.error(error);
        }
    }

    async function updateMachine() {
        try {
            const changedFields = { id: $form.id };
            for (const field in $form) {
                if ($form[field] !== $machine[field]) {
                    changedFields[field] = $form[field];
                }
            }
            if (Object.keys(changedFields).length < 2) return;
            const response = await sendJSON('/api/auth/machine', 'PUT', changedFields);
            if (response.status !== 200) return alert('Failed to update the machine');
            const data = await response.json();
            machine.set(data);
            navigate('?view=true');
            loadFields();
        } catch (error) {
            console.error(error);
        }
    }
</script>

<form on:submit|preventDefault={saveMachine} class="space-y-4 w-full md:w-auto">
    <Input properties={{ id: 'id', label: 'Id' }} bind:value={$form.id} disabled={true} />

    <Input properties={{ id: 'name', label: 'Name' }} bind:value={$form.name} errors={$fieldErrors.name} />

    <Input properties={{ id: 'make', label: 'Make' }} bind:value={$form.make} />

    <Select
        properties={{ id: 'machine_type', label: 'Type', placeholder: 'Select a type' }}
        bind:selected={$selectedType}
        onSelectedChange={(opt) => {
            opt && ($form.machine_type = opt.value);
        }}
        errors={$fieldErrors?.machine_type}
    >
        {#each $machineTypes as machineType}
            <SelectItem value={machineType.id} label={machineType.name} />
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
        {#each $machineStatuses as machineStatus}
            <SelectItem value={machineStatus.id} label={machineStatus.name} />
        {/each}
    </Select>

    <Select
        properties={{ id: 'facility', label: 'Facility', placeholder: 'Select a facility' }}
        bind:selected={$selectedFacility}
        onSelectedChange={(opt) => {
            opt && ($form.facility = opt.value);
        }}
        errors={$fieldErrors?.facility}
    >
        {#each $facilities as facility}
            <SelectItem value={facility.id} label={facility.name} />
        {/each}
    </Select>

    <div>
        <div class="ml-auto text-xs text-muted-foreground">Created {$form.created}</div>
        <div class="ml-auto text-xs text-muted-foreground pt-2">Edited {$form.edited}</div>
    </div>

    <Button type="submit" disabled={$isViewing || $hasErrors}>Save</Button>
</form>
