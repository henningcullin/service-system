<script lang="ts">
    import Button from '$components/ui/button/button.svelte';
    import Input from '$components/ui/input/input.svelte';
    import Label from '$components/ui/label/label.svelte';
    import * as Select from '$components/ui/select/index.js';
    import * as Tabs from '$components/ui/tabs/index.js';
    import * as AlertDialog from '$components/ui/alert-dialog/index.js';
    import Separator from '$components/ui/separator/separator.svelte';
    import { facilities, machine, machineStatuses, machineTypes } from '$stores';
    import { getFacilities, getMachine, getMachineStatuses, getMachineTypes, sendDelete, sendJSON } from '$utils';
    import { onMount } from 'svelte';

    import { z } from 'zod';

    const formSchema = z.object({
        name: z.string().min(1, 'Name is required'),
        make: z.string().nullable(),
        machine_type: z.string().min(1, 'Machine Type is required').uuid('Must be a valid Machine Type'),
        status: z.string().min(1, 'Status is required').uuid('Must be a valid Status'),
        facility: z.string().uuid('Must be a valid Facility').nullable(),
    });

    getMachineTypes();
    getMachineStatuses();
    getFacilities();

    $: selectedType = form.machine_type
        ? { label: $machineTypes?.find((mt) => mt.id === form.machine_type)?.name, value: form.machine_type }
        : null;
    $: selectedStatus = form.status
        ? { label: $machineStatuses?.find((ms) => ms.id === form.status)?.name, value: form.status }
        : null;
    $: selectedFacility = form.facility
        ? { label: $facilities?.find((f) => f.id === form.facility)?.name, value: form.facility }
        : null;

    const form = {
        id: '',
        name: '',
        make: '',
        machine_type: '',
        status: '',
        created: '',
        edited: '',
        facility: '',
    };

    function clearFields() {
        for (const field in form) {
            form[field] = '';
        }
    }

    function loadFields() {
        form.id = $machine?.id;
        form.name = $machine?.name;
        form.make = $machine?.make;
        form.machine_type = $machine?.machine_type?.id;
        form.status = $machine?.status?.id;
        form.created = new Date($machine?.created).toLocaleString();
        form.edited = new Date($machine?.edited).toLocaleString();
        form.facility = $machine?.facility?.id;
    }

    let fieldErrors = { name: '', make: '', machine_type: '', status: '', facility: '' };
    let hasErrors = false;

    $: {
        if (!isViewing) {
            try {
                formSchema.parse(form);
                fieldErrors = { name: '', make: '', machine_type: '', status: '', facility: '' };
                hasErrors = false;
            } catch (e) {
                if (e instanceof z.ZodError) {
                    // @ts-ignore
                    fieldErrors = e.flatten().fieldErrors;
                    hasErrors = true;
                }
            }
        } else {
            fieldErrors = { name: '', make: '', machine_type: '', status: '', facility: '' };
            hasErrors = false;
        }
    }

    let deleteDialogOpen = false;

    function updateUrl() {
        const url = new URL(window.location.href);
        const pathArray = url.pathname.split('/');
        if (pathArray.length > 2) pathArray.pop();
        if ($machine.id) pathArray.push($machine.id);
        url.pathname = pathArray.join('/');
        const newUrl = url.href;
        navigate(newUrl);
    }

    let isSaving = false;

    async function saveMachine() {
        if (isSaving) return;
        isSaving = true;
        switch (true) {
            case isViewing:
                return (isSaving = false);
            case isCreating:
                await createMachine();
                break;
            case isEditing:
                await updateMachine();
                break;
        }
        isSaving = false;
    }

    async function createMachine() {
        try {
            const { name, make, machine_type, status, facility } = form;
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
            updateUrl();
            navigate('?view=true');
            loadFields();
        } catch (error) {
            console.error(error);
        }
    }

    async function updateMachine() {
        try {
            const changedFields = { id: form.id };
            for (const field in form) {
                if (form[field] !== $machine[field]) {
                    changedFields[field] = form[field];
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

    async function deleteMachine() {
        try {
            const response = await sendDelete(`/api/auth/machine?id=${id}`);
            if (response.status !== 204) return alert('Failed to delete machine');
            machine.set({});
            clearFields();
            navigate('?view=true');
            updateUrl();
        } catch (error) {
            console.error(error);
        }
    }
</script>

<Tabs.Root class="flex flex-col items-center min-h-screen pt-8">
    <Tabs.List class="flex space-x-4">
        <Tabs.Trigger value="card">Machine</Tabs.Trigger>
        <Tabs.Trigger value="tasks">Tasks</Tabs.Trigger>
        <Tabs.Trigger value="reports">Reports</Tabs.Trigger>
    </Tabs.List>
    <Tabs.Content value="card" class="flex flex-col min-w-full"></Tabs.Content>
    <Tabs.Content value="tasks"></Tabs.Content>
    <Tabs.Content value="reports"></Tabs.Content>
</Tabs.Root>

<AlertDialog.Root bind:open={deleteDialogOpen}>
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
