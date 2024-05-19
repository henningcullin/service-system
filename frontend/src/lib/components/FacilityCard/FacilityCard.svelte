<script>
    import Actions from './Actions.svelte';
    import { useLocation } from 'svelte-navigator';
    import Separator from '$components/ui/separator/separator.svelte';
    import { id, isCreating, isEditing, isViewing, loadFields, updateUrl } from './common';
    import Form from './Form.svelte';
    import { getFacilities } from '$utils';
    import { onMount } from 'svelte';
    import { facilities, facility } from '$stores';
    import * as Select from '$components/ui/select/index.js';
    import Label from '$components/ui/label/label.svelte';

    $: selected = { value: $facility?.id, label: $facility?.name };

    function onSelectedChange(opt) {
        if (!opt) return;
        facility.set($facilities.find((f) => f.id === opt?.value));
        updateUrl($facility?.id);
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
        await getFacilities();
        if ($id) {
            const selected = $facilities.find((f) => f.id === $id);
            facility.set({ ...selected });
            loadFields();
        }
    });
</script>

<div class="space-y-0.5">
    <h2 class="text-2xl font-bold tracking-tight pb-2">Facility</h2>
    <Actions />
    <Label>Selected facility</Label>
    <Select.Root {selected} {onSelectedChange} class="mt-2">
        <Select.Trigger>
            <Select.Value placeholder="Select a facility" />
        </Select.Trigger>
        <Select.Content>
            {#each $facilities as facility}
                <Select.Item value={facility.id} label={facility.name} />
            {/each}
        </Select.Content>
    </Select.Root>
</div>
<Separator class="my-6" />
<div>
    <Form />
</div>
