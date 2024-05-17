<script lang="ts">
    import * as AlertDialog from '$components/ui/alert-dialog/index.js';
    import * as Card from '$components/ui/card/index.js';
    import * as Select from '$components/ui/select/index.js';
    import Button from '$components/ui/button/button.svelte';
    import Input from '$components/ui/input/input.svelte';
    import Label from '$components/ui/label/label.svelte';
    import { sendDelete } from '$utils';

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
        sendDelete(`/api/auth/${apiEndpoint}/?id=${$formStore.id}`);
    }

    function createItem() {
        console.log('we create');
    }

    function updateItem() {
        console.log('we update');
    }

    export let formStore;
    export let sourceStore;
    export let apiEndpoint: string;
    export let cardProps: { title: string; desc: string };
    export let formProps: { selectPlaceholder: string; namePlaceholder: string };
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
            variant="destructive"
            on:click={() => {
                open = true;
            }}>Delete</Button
        >
        <Button
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
