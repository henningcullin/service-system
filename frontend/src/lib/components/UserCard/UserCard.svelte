<script lang="ts">
    import Actions from './Actions.svelte';
    import { useLocation } from 'svelte-navigator';
    import { onMount } from 'svelte';
    import Separator from '$components/ui/separator/separator.svelte';
    import { getUser, getRoles, getFacilities } from '$utils';
    import { id, isCreating, isEditing, isViewing, loadFields } from './common';
    import Form from './Form.svelte';

    getRoles();
    getFacilities();

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
        if ($id) {
            await getUser($id);
            if (!$isCreating) loadFields();
        }
    });
</script>

<div class="space-y-0.5 min-w-full">
    <h2 class="text-2xl font-bold tracking-tight pb-2">User</h2>
    <Actions />
</div>
<Separator class="my-6" />
<div class="min-w-full">
    <Form />
</div>
