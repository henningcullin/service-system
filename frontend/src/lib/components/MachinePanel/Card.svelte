<script lang="ts">
    import * as Card from '$components/ui/card/index.js';
    import * as Select from '$components/ui/select/index.js';
    import Button from '$components/ui/button/button.svelte';
    import Input from '$components/ui/input/input.svelte';
    import Label from '$components/ui/label/label.svelte';

    export let props: { title: string; desc: string; selectPlaceholder: string; namePlaceholder: string };
</script>

<Card.Root>
    <Card.Header>
        <Card.Title>{props.title}</Card.Title>
        <Card.Description>{props.desc}</Card.Description>
    </Card.Header>
    <Card.Content>
        <form>
            <div class="grid w-full items-center gap-4">
                <Select.Root>
                    <Select.Trigger>
                        <Select.Value placeholder={props.selectPlaceholder} />
                    </Select.Trigger>
                    <Select.Content>
                        <slot />
                    </Select.Content>
                </Select.Root>
                <div class="flex flex-col space-y-1.5">
                    <Label for="name">Name</Label>
                    <Input id="name" placeholder={props.namePlaceholder} />
                </div>
                <div class="flex flex-col space-y-1.5">
                    <Label for="id">Id</Label>
                    <Input id="id" placeholder="Id" disabled />
                </div>
            </div>
        </form>
    </Card.Content>
    <Card.Footer class="flex justify-between">
        <Button variant="outline">New</Button>
        <Button variant="destructive">Delete</Button>
        <Button>Save</Button>
    </Card.Footer>
</Card.Root>
