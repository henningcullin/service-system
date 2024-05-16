<script lang="ts">
    import * as Card from '$components/ui/card/index.js';
    import * as Select from '$components/ui/select/index.js';
    import * as Tabs from '$components/ui/tabs/index.js';
    import Button from '$components/ui/button/button.svelte';
    import Input from '$components/ui/input/input.svelte';
    import Label from '$components/ui/label/label.svelte';

    import { machineTypes, machineType, machineStatuses, machineStatus } from '$stores';
    import { getMachineTypes, getMachineStatuses } from '$utils';

    getMachineStatuses();
    getMachineTypes();
</script>

<div class="w-full min-h-[15vh] flex items-center justify-center">
    <h2 class="text-2xl font-bold tracking-tight pb-2">Machine Panel</h2>
</div>

<div class="w-full min-h-[65vh] flex items-center justify-center">
    <Tabs.Root value="type" class="w-[400px]">
        <Tabs.List class="grid w-full grid-cols-2">
            <Tabs.Trigger value="type">Type</Tabs.Trigger>
            <Tabs.Trigger value="status">Status</Tabs.Trigger>
        </Tabs.List>
        <Tabs.Content value="type">
            <Card.Root>
                <Card.Header>
                    <Card.Title>Type</Card.Title>
                    <Card.Description>Create new or edit existing machine types here</Card.Description>
                </Card.Header>
                <Card.Content>
                    <form>
                        <div class="grid w-full items-center gap-4">
                            <Select.Root>
                                <Select.Trigger>
                                    <Select.Value placeholder="Select a type" />
                                </Select.Trigger>
                                <Select.Content>
                                    {#each $machineTypes as machineType}
                                        <Select.Item value={machineType.id} label={machineType.name} />
                                    {/each}
                                </Select.Content>
                            </Select.Root>

                            <div class="flex flex-col space-y-1.5">
                                <Label for="name">Name</Label>
                                <Input id="name" placeholder="Name of type" />
                            </div>
                            <div class="flex flex-col space-y-1.5">
                                <Label for="id">Id</Label>
                                <Input id="id" placeholder="Id" disabled />
                            </div>
                        </div>
                    </form>
                </Card.Content>
                <Card.Footer class="flex justify-between">
                    <Button variant="outline">Cancel</Button>
                    <Button>Deploy</Button>
                </Card.Footer>
            </Card.Root>
        </Tabs.Content>
        <Tabs.Content value="status">
            <Card.Root>
                <Card.Header>
                    <Card.Title>Status</Card.Title>
                    <Card.Description>Create new or edit existing machine statuses here</Card.Description>
                </Card.Header>
                <Card.Content>
                    <form>
                        <div class="grid w-full items-center gap-4">
                            <Select.Root>
                                <Select.Trigger>
                                    <Select.Value placeholder="Select a status" />
                                </Select.Trigger>
                                <Select.Content>
                                    {#each $machineStatuses as machineStatus}
                                        <Select.Item value={machineStatus.id} label={machineStatus.name} />
                                    {/each}
                                </Select.Content>
                            </Select.Root>
                            <div class="flex flex-col space-y-1.5">
                                <Label for="name">Name</Label>
                                <Input id="name" placeholder="Name of type" />
                            </div>
                            <div class="flex flex-col space-y-1.5">
                                <Label for="id">Id</Label>
                                <Input id="id" placeholder="Id" disabled />
                            </div>
                        </div>
                    </form>
                </Card.Content>
                <Card.Footer class="flex justify-between">
                    <Button variant="outline">Cancel</Button>
                    <Button>Deploy</Button>
                </Card.Footer>
            </Card.Root>
        </Tabs.Content>
    </Tabs.Root>
</div>
