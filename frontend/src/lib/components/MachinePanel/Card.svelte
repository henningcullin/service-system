<script lang="ts">
    import * as AlertDialog from '$components/ui/alert-dialog/index.js';
    import * as Card from '$components/ui/card/index.js';
    import * as Select from '$components/ui/select/index.js';
    import Button from '$components/ui/button/button.svelte';
    import Input from '$components/ui/input/input.svelte';
    import Label from '$components/ui/label/label.svelte';
    import { toast } from 'svelte-sonner';
    import { sendDelete, sendJSON } from '$utils';
    import { z } from 'zod';

    export let formStore;
    export let sourceStore;
    export let apiEndpoint: string;
    export let cardProps: { title: string; desc: string };
    export let formProps: { selectPlaceholder: string; namePlaceholder: string };

    const formSchema = z.object({
        name: z.string().min(1, 'Name is required').max(255, 'Name must be 255 characters or less'),
    });

    const errors: { name: string[] } = { name: [] };
    $: hasErrors = errors?.name?.length;

    $: {
        if (!VIEWING_STATE) {
            const formStatus = formSchema.safeParse($formStore);
            if (!formStatus.success) {
                errors.name = formStatus?.error?.flatten()?.fieldErrors?.name;
            } else {
                errors.name = [];
            }
        } else {
            errors.name = [];
        }
    }

    const VIEWING_STATE = 0;
    const CREATING_STATE = 1;
    const EDITING_STATE = 2;

    let open = false;
    let state = VIEWING_STATE;

    $: selected = $formStore.id
        ? { label: $sourceStore?.find((item) => item.id === $formStore?.id)?.name, value: $formStore?.id }
        : null;

    function onSelectedChange(newValue) {
        if (!newValue) return;
        $formStore.id = newValue.value;
        $formStore.name = newValue.label;
        state = EDITING_STATE;
    }

    async function deleteItem() {
        try {
            const response = await sendDelete(`/api/auth/${apiEndpoint}?id=${$formStore.id}`);
            if (response.status !== 204) return toast.error(`Could not delete the ${cardProps.title}`);
            sourceStore.update((prev) => prev.filter((item) => item.id !== $formStore.id));
            toast.success(`Deleted the ${cardProps.title}`);
            formStore.set({ id: '', name: '' });
        } catch (error) {
            toast.error(`Could not delete the ${cardProps.title}`);
        }
    }

    async function createItem() {
        try {
            const response = await sendJSON(`/api/auth/${apiEndpoint}`, 'POST', { name: $formStore.name });
            if (response.status !== 201) toast.error(`Could not create the ${cardProps.title}`);
            const data = await response.json();
            sourceStore.update((prev) => {
                prev.push(data);
                return prev;
            });
            formStore.set(data);
            toast.success(`Created the ${cardProps.title}`);
            state = EDITING_STATE;
        } catch (error) {
            toast.error(`Could not create the ${cardProps.title}`);
        }
    }

    async function updateItem() {
        try {
            const response = await sendJSON(`/api/auth/${apiEndpoint}`, 'PUT', {
                id: $formStore.id,
                name: $formStore.name,
            });

            if (response.status !== 204) toast.error(`Could not save the ${cardProps.title}`);
            sourceStore.update((prev) => {
                const index = prev.findIndex((item) => item.id === $formStore.id);
                prev[index] = { ...$formStore };
                return prev;
            });
            toast.success(`Updated the ${cardProps.title}`);
        } catch (error) {
            toast.error(`Could not save the ${cardProps.title}`);
        }
    }
</script>

<Card.Root>
    <Card.Header>
        <Card.Title>{cardProps.title}</Card.Title>
        <Card.Description>{cardProps.desc}</Card.Description>
    </Card.Header>
    <Card.Content>
        <form>
            <div class="grid w-full items-center gap-4">
                <Select.Root {selected} {onSelectedChange}>
                    <Select.Trigger>
                        <Select.Value placeholder={formProps.selectPlaceholder} />
                    </Select.Trigger>
                    <Select.Content>
                        <slot />
                    </Select.Content>
                </Select.Root>
                <div class="flex flex-col space-y-1.5">
                    <Label for="name">Name</Label>
                    <Input id="name" bind:value={$formStore.name} placeholder={formProps.namePlaceholder} />
                </div>
                <div class="flex flex-col space-y-1.5">
                    <Label for="id">Id</Label>
                    <Input id="id" placeholder="Id" bind:value={$formStore.id} disabled />
                </div>
            </div>
        </form>
    </Card.Content>
    <Card.Footer class="flex justify-between">
        <Button
            variant="outline"
            on:click={() => {
                formStore.set({ id: '', name: '' });
                state = CREATING_STATE;
            }}>New</Button
        >
        <Button
            disabled={state === CREATING_STATE || !$formStore?.id}
            variant="destructive"
            on:click={() => {
                open = true;
            }}>Delete</Button
        >
        <Button
            disabled={hasErrors || state === VIEWING_STATE}
            on:click={() => {
                if (state === CREATING_STATE) createItem();
                else if (state === EDITING_STATE) updateItem();
            }}>Save</Button
        >
    </Card.Footer>
</Card.Root>

<AlertDialog.Root bind:open>
    <AlertDialog.Content>
        <AlertDialog.Header>
            <AlertDialog.Title>Are you absolutely sure?</AlertDialog.Title>
            <AlertDialog.Description>
                This action cannot be undone. This will permanently delete the '{$formStore.name}' {cardProps.title} from
                our servers.
            </AlertDialog.Description>
        </AlertDialog.Header>
        <AlertDialog.Footer>
            <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
            <AlertDialog.Action on:click={deleteItem}>Continue</AlertDialog.Action>
        </AlertDialog.Footer>
    </AlertDialog.Content>
</AlertDialog.Root>
