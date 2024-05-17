<script lang="ts">
    import Actions from './Actions.svelte';
    import { useLocation } from 'svelte-navigator';
    import { onMount } from 'svelte';
    import Separator from '$components/ui/separator/separator.svelte';
    import { getOneTask, getTaskTypes, getTaskStatuses } from '$utils';
    import { id, isCreating, isEditing, isViewing, loadFields } from './common';
    import Form from './Form.svelte';

    getTaskTypes();
    getTaskStatuses();

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
            await getOneTask($id);
            if (!$isCreating) loadFields();
        }
    });
</script>

<div class="space-y-0.5">
    <h2 class="text-2xl font-bold tracking-tight pb-2">Task</h2>
    <Actions />
</div>
<Separator class="my-6" />
<div>
    <Form />
</div>
