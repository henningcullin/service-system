<script>
    import { Form, Field, Label, Input, Dropdown, Text, Icon, Menu, Item, Select, Option } from 'svelte-fomantic-ui';
    import { machineTypes, machineStatuses } from '$lib/stores';
    import { getMachineStatuses, getMachineTypes } from '$lib/utils';

    let machine = {
        name: '',
        make: '',
        machine_type: '',
        status: '979fb992-1805-40d7-82ba-6b0f4f88d1c7',
    };

    getMachineStatuses();
    getMachineTypes();

    setInterval(() => {
        machine.machine_type = 'cfa63361-ba0d-4514-8104-0c8789f6b9b7';
        console.log(machine);
    }, 1500);
</script>

<Form ui>
    <Field>
        <Input text placeholder="Name" bind:value={machine.name} />
    </Field>
    <Field>
        <Input text placeholder="Make" bind:value={machine.make} />
    </Field>
    <Field>
        <Label>Type</Label>
        <!--         <Select ui clearable selection dropdown bind:value={machine.machine_type}>
            <Option value="">Type</Option>
            {#each $machineTypes as machineType}
                <Option value={machineType.id}>
                    {machineType.name}
                </Option>
            {/each}
        </Select> -->
        <select class="ui clearable selection dropdown" bind:value={machine.machine_type}>
            <option value="">Type</option>
            {#each $machineTypes as machineType}
                <option value={machineType.id}>
                    {machineType.name}
                </option>
            {/each}
        </select>
    </Field>
    <Field>
        <Label>Status</Label>
        <Dropdown ui clearable selection bind:value={machine.status}>
            <Text default>Status</Text>
            <Icon dropdown />
            <Menu>
                {#each $machineStatuses as machineStatus}
                    <Item value={machineStatus.id}>
                        {machineStatus.name}
                    </Item>
                {/each}
            </Menu>
        </Dropdown>
    </Field>
</Form>
