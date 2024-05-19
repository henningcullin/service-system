<script>
    import Actions from './Actions.svelte';
    import { useLocation } from 'svelte-navigator';
    import Separator from '$components/ui/separator/separator.svelte';
    import { id, isCreating, isEditing, isViewing, loadFields, updateUrl } from './common';
    import Form from './Form.svelte';
    import { getRoles } from '$utils';
    import { onMount } from 'svelte';
    import { role, roles } from '$stores';
    import * as Select from '$components/ui/select/index.js';
    import Label from '$components/ui/label/label.svelte';

    $: selected = { value: $role?.id, label: $role?.name };

    function onSelectedChange(opt) {
        if (!opt) return;
        role.set($roles.find((r) => r.id === opt?.value));
        updateUrl($role?.id);
        loadFields();
    }

    const location = useLocation();
    $: params = new URLSearchParams($location.search);
    $: {
        const segments = $location.pathname.split('/');
        id.set(segments.length > 2 ? segments.at(-1) : null);
    }
    $: isCreating.set(params?.get('new') === 'true');
    $: isEditing.set(params?.get('edit') === 'true' && !!$id);
    $: isViewing.set(!($isCreating || $isEditing));

    onMount(async () => {
        await getRoles();
        if ($id) {
            const selected = $roles.find((r) => r.id === $id);
            role.set({ ...selected });
            loadFields();
        }
    });
</script>

<div class="space-y-0.5">
    <h2 class="text-2xl font-bold tracking-tight pb-2">Role</h2>
    <Actions />
    <Label>Selected role</Label>
    <Select.Root {selected} {onSelectedChange} class="mt-2">
        <Select.Trigger>
            <Select.Value placeholder="Select a role" />
        </Select.Trigger>
        <Select.Content>
            {#each $roles as role}
                <Select.Item value={role.id} label={role.name} />
            {/each}
        </Select.Content>
    </Select.Root>
</div>
<Separator class="my-6" />
<div>
    <Form />
</div>
