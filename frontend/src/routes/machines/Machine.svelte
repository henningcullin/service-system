<script lang="ts">
    import Button from '$components/ui/button/button.svelte';
    import Input from '$components/ui/input/input.svelte';
    import Label from '$components/ui/label/label.svelte';
    import * as Select from '$components/ui/select/index.js';
    import * as Tabs from '$components/ui/tabs/index.js';
    import Separator from '$components/ui/separator/separator.svelte';
    import { facilities, machine, machineStatuses, machineTypes } from '$stores';
    import { getFacilities, getMachine, getMachineStatuses, getMachineTypes, sendDelete, sendJSON } from '$utils';
    import { onMount } from 'svelte';
    import { navigate, useLocation } from 'svelte-navigator';
    import { z } from 'zod';

    const newMachineSchema = z.object({
        name: z.string().min(1, 'Name is required'),
        make: z.string().nullable(),
        machine_type: z.string().min(1, 'Machine Type is required').uuid('Must be a valid Machine Type'),
        status: z.string().min(1, 'Status is required').uuid('Must be a valid Status'),
        facility: z.string().uuid('Must be a valid Facility').nullable(),
    });

    type NewMachine = z.infer<typeof newMachineSchema>;

    getMachineTypes();
    getMachineStatuses();
    getFacilities();

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

    $: location = useLocation();

    $: params = new URLSearchParams($location.search);

    let id: undefined | string | null;

    $: {
        const segments = $location.pathname.split('/');
        id = segments.length > 2 ? segments.at(-1) : null;
    }

    onMount(async () => {
        if (id) {
            await getMachine(id);
            if (!isCreating) loadFields();
        }
    });

    $: isCreating = params?.get('new') === 'true';
    $: isEditing = (params?.get('edit') === 'true' && id !== null) || id !== undefined;
    $: isViewing = !(isCreating || isEditing);

    $: selectedType = form.machine_type
        ? { label: $machineTypes?.find((mt) => mt.id === form.machine_type)?.name, value: form.machine_type }
        : null;
    $: selectedStatus = form.status
        ? { label: $machineStatuses?.find((ms) => ms.id === form.status)?.name, value: form.status }
        : null;
    $: selectedFacility = form.facility
        ? { label: $facilities?.find((f) => f.id === form.facility)?.name, value: form.facility }
        : null;

    async function newMachine() {
        navigate('?new=true');
        clearFields();
    }

    async function editMachine() {
        navigate('?edit=true');
        if (id) loadFields();
    }

    async function deleteMachine() {
        try {
            const accepted = confirm('Are you sure you want to delete this?');
            if (!accepted) return;
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

    function cancel() {
        navigate('?view=true');
        if (id) loadFields();
    }

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
            loadFields();
        } catch (error) {
            console.error(error);
        }
    }

    async function updateMachine() {}
</script>

<Tabs.Root class="flex flex-col items-center min-h-screen pt-8">
    <Tabs.List class="flex space-x-4">
        <Tabs.Trigger value="card">Machine</Tabs.Trigger>
        <Tabs.Trigger value="tasks">Tasks</Tabs.Trigger>
        <Tabs.Trigger value="reports">Reports</Tabs.Trigger>
    </Tabs.List>
    <Tabs.Content value="card" class="flex flex-col min-w-full">
        <div class="space-y-0.5">
            <h2 class="text-2xl font-bold tracking-tight pb-2">Machine</h2>
            <div class="flex space-x-4">
                <Button on:click={newMachine} disabled={isCreating} variant="outline">New</Button>
                <Button on:click={editMachine} disabled={isEditing || !id} variant="outline">Edit</Button>
                <Button on:click={deleteMachine} disabled={isCreating || !id} variant="destructive">Delete</Button>
                <Button on:click={cancel} disabled={isViewing} variant="outline">Cancel</Button>
            </div>
        </div>
        <Separator class="my-6" />
        <div>
            <form on:submit|preventDefault={saveMachine} class="space-y-4 w-full md:w-auto">
                <div>
                    <Label for="id">Id</Label>
                    <Input type="text" id="id" bind:value={form.id} disabled />
                </div>

                <div>
                    <Label for="name">Name</Label>
                    <Input type="text" id="name" bind:value={form.name} placeholder="Name" disabled={isViewing} />
                </div>

                <div>
                    <Label for="make">Make</Label>
                    <Input type="text" id="make" bind:value={form.make} placeholder="Make" disabled={isViewing} />
                </div>

                <div>
                    <Label for="type">Type</Label>
                    <Select.Root
                        disabled={isViewing}
                        selected={selectedType}
                        onSelectedChange={(opt) => {
                            opt && (form.machine_type = opt.value);
                        }}
                    >
                        <Select.Trigger>
                            <Select.Value placeholder="Select a type" />
                        </Select.Trigger>
                        <Select.Content>
                            {#each $machineTypes as machineType}
                                <Select.Item value={machineType.id} label={machineType.name} />
                            {/each}
                        </Select.Content>
                    </Select.Root>
                </div>

                <div>
                    <Label for="status">Status</Label>
                    <Select.Root
                        disabled={isViewing}
                        selected={selectedStatus}
                        onSelectedChange={(opt) => {
                            opt && (form.status = opt.value);
                        }}
                    >
                        <Select.Trigger>
                            <Select.Value placeholder="Select a status" />
                        </Select.Trigger>
                        <Select.Content>
                            {#each $machineStatuses as machineStatus}
                                <Select.Item value={machineStatus.id} label={machineStatus.name} />
                            {/each}
                        </Select.Content>
                    </Select.Root>
                </div>

                <div>
                    <Label for="facility">Facility</Label>
                    <Select.Root
                        disabled={isViewing}
                        selected={selectedFacility}
                        onSelectedChange={(opt) => {
                            opt && (form.facility = opt.value);
                        }}
                    >
                        <Select.Trigger>
                            <Select.Value placeholder="Select a facility" />
                        </Select.Trigger>
                        <Select.Content>
                            {#each $facilities as facility}
                                <Select.Item value={facility.id} label={facility.name} />
                            {/each}
                        </Select.Content>
                        <Select.Input bind:value={form.facility} />
                    </Select.Root>
                </div>

                <div>
                    <div class="ml-auto text-xs text-muted-foreground">Created {form.created}</div>
                    <div class="ml-auto text-xs text-muted-foreground pt-2">Edited {form.edited}</div>
                </div>

                <Button type="submit" disabled={isViewing}>Save</Button>
            </form>
        </div>
    </Tabs.Content>
    <Tabs.Content value="tasks"></Tabs.Content>
    <Tabs.Content value="reports"></Tabs.Content>
</Tabs.Root>
