<script lang="ts">
    import { writable } from 'svelte/store';
    import Actions from './Actions.svelte';
    import { navigate, useLocation } from 'svelte-navigator';
    import { onMount } from 'svelte';
    import Separator from '$components/ui/separator/separator.svelte';

    const isCreating = writable(false);
    const isEditing = writable(false);
    const isViewing = writable(false);

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

    $: isCreating.set(params?.get('new') === 'true');
    $: isEditing.set(params?.get('edit') === 'true' && !!id);
    $: isViewing.set(!($isCreating || $isEditing));
</script>

<div class="space-y-0.5">
    <h2 class="text-2xl font-bold tracking-tight pb-2">Machine</h2>
    <Actions></Actions>
</div>
<Separator class="my-6" />
<div>
    <form on:submit|preventDefault={saveMachine} class="space-y-4 w-full md:w-auto">
        <div>
            <Label for="id">Id</Label>
            <Input type="text" id="id" bind:value={form.id} disabled />
        </div>

        <div>
            <Label for="name" class={fieldErrors.name ? 'text-red-800' : ''}>Name</Label>
            <Input type="text" id="name" bind:value={form.name} placeholder="Name" disabled={isViewing} />
            {#if fieldErrors.name}
                <p class="text-red-800 ml-auto text-xs pt-1">{fieldErrors.name[0]}</p>
            {/if}
        </div>

        <div>
            <Label for="make" class={fieldErrors.make ? 'text-red-800' : ''}>Make</Label>
            <Input type="text" id="make" bind:value={form.make} placeholder="Make" disabled={isViewing} />
        </div>

        <div>
            <Label for="type" class={fieldErrors.machine_type ? 'text-red-800' : ''}>Type</Label>
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
            {#if fieldErrors.machine_type}
                <p class="text-red-800 ml-auto text-xs pt-1">{fieldErrors.machine_type[0]}</p>
            {/if}
        </div>

        <div>
            <Label for="status" class={fieldErrors.status ? 'text-red-800' : ''}>Status</Label>
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
            {#if fieldErrors.status}
                <p class="text-red-800 ml-auto text-xs pt-1">{fieldErrors.status[0]}</p>
            {/if}
        </div>

        <div>
            <Label for="facility" class={fieldErrors.facility ? 'text-red-800' : ''}>Facility</Label>
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
            {#if fieldErrors.facility}
                <p class="text-red-800 ml-auto text-xs pt-1">{fieldErrors.facility[0]}</p>
            {/if}
        </div>

        <div>
            <div class="ml-auto text-xs text-muted-foreground">Created {form.created}</div>
            <div class="ml-auto text-xs text-muted-foreground pt-2">Edited {form.edited}</div>
        </div>

        <Button type="submit" disabled={isViewing || hasErrors}>Save</Button>
    </form>
</div>
