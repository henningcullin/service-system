<script>
    import { writable } from 'svelte/store';
    import { user, roles, facilities } from '$stores';
    import { sendJSON } from '$utils';
    import { navigate } from 'svelte-navigator';
    import Input from './Input.svelte';
    import {
        fieldErrors,
        hasErrors,
        form,
        isCreating,
        isEditing,
        isViewing,
        loadFields,
        updateUrl,
        formSchema,
    } from './common';
    import Button from '$components/ui/button/button.svelte';
    import { z } from 'zod';
    import Select from './Select.svelte';
    import SelectItem from '$components/ui/select/select-item.svelte';
    import Label from '$components/ui/label/label.svelte';
    import Checkbox from '$components/ui/checkbox/checkbox.svelte';
    import { toast } from 'svelte-sonner';

    const selectedRole = writable({ label: '', value: '' });
    const selectedFacility = writable({ label: '', value: '' });

    $: selectedRole.set(
        $form.role ? { label: $roles?.find((r) => r.id === $form?.role)?.name, value: $form?.role } : null,
    );
    $: selectedFacility.set(
        $form.facility
            ? { label: $facilities?.find((f) => f.id === $form?.facility)?.name, value: $form?.facility }
            : null,
    );
    $: {
        if (!$isViewing) {
            try {
                formSchema.parse($form);
                $fieldErrors = {
                    first_name: '',
                    last_name: '',
                    email: '',
                    role: '',
                };
                $hasErrors = false;
            } catch (e) {
                if (e instanceof z.ZodError) {
                    // @ts-ignore
                    $fieldErrors = e.flatten().fieldErrors;
                    $hasErrors = true;
                }
            }
        } else {
            $fieldErrors = {
                first_name: '',
                last_name: '',
                email: '',
                role: '',
            };
            $hasErrors = false;
        }
    }

    let isSaving = false;

    async function saveUser() {
        if (isSaving) return;
        isSaving = true;
        switch (true) {
            case $isViewing:
                return (isSaving = false);
            case $isCreating:
                await createUser();
                break;
            case $isEditing:
                await updateUser();
                break;
        }
        isSaving = false;
    }

    async function createUser() {
        try {
            const { first_name, last_name, email, role, phone, active, occupation, facility } = $form;
            const formObj = { first_name, last_name, email, role };
            if (phone) {
                formObj['phone'] = phone;
            }
            if (occupation) {
                formObj['occupation'] = occupation;
            }
            if (facility) {
                formObj['facility'] = facility;
            }
            if (active !== null && active !== undefined) {
                formObj['active'] = active;
            }
            const response = await sendJSON('/api/auth/user', 'POST', formObj);
            if (response.status !== 201) return toast.error('Failed to create the user');
            const data = await response.json();
            user.set(data);
            updateUrl($user.id);
            navigate('?edit=true');
            loadFields();
            toast.success('Created the user');
        } catch (error) {
            toast.error('Failed to create the user');
        }
    }

    async function updateUser() {
        try {
            const changedFields = { id: $form?.id };
            const { first_name, last_name, email, role, phone, active, occupation, facility } = $form;
            if (first_name !== $user?.first_name) changedFields['title'] = title;
            if (last_name !== $user?.last_name) changedFields['last_name'] = last_name;
            if (email !== $user?.email) changedFields['email'] = email;
            if (role !== $user?.role?.id) changedFields['role'] = role;
            if (phone !== $user?.phone) changedFields['phone'] = phone;
            if (active !== $user?.active) changedFields['active'] = active;
            if (occupation !== $user?.occupation) changedFields['occupation'] = occupation;
            if (facility !== $user?.facility?.id) changedFields['facility'] = facility;
            if (Object.keys(changedFields).length < 2) return;
            const response = await sendJSON('/api/auth/user', 'PUT', changedFields);
            if (response.status !== 200) return toast.error('Failed to update the user');
            const data = await response.json();
            user.set(data);
            navigate('?edit=true');
            loadFields();
            toast.success('Saved the user');
        } catch (error) {
            toast.error('Failed to update the user');
        }
    }
</script>

<form on:submit|preventDefault={saveUser} class="space-y-4 w-full md:w-auto">
    <Input properties={{ id: 'id', label: 'Id' }} bind:value={$form.id} disabled={true} />

    <Input
        properties={{ id: 'first_name', label: 'First Name' }}
        bind:value={$form.first_name}
        errors={$fieldErrors.first_name}
    />
    <Input
        properties={{ id: 'last_name', label: 'Last Name' }}
        bind:value={$form.last_name}
        errors={$fieldErrors.last_name}
    />
    <Input properties={{ id: 'email', label: 'Email' }} bind:value={$form.email} errors={$fieldErrors.email} />
    <Input properties={{ id: 'phone', label: 'Phone' }} bind:value={$form.phone} />

    <Select
        properties={{ id: 'role', label: 'Role', placeholder: 'Select a role' }}
        bind:selected={$selectedRole}
        onSelectedChange={(opt) => {
            opt && ($form.role = opt.value);
        }}
        errors={$fieldErrors?.role}
    >
        {#each $roles as role}
            <SelectItem value={role.id} label={role.name} />
        {/each}
    </Select>

    <Select
        properties={{ id: 'facility', label: 'Facility', placeholder: 'Select a facility' }}
        bind:selected={$selectedFacility}
        onSelectedChange={(opt) => {
            opt && ($form.facility = opt.value);
        }}
    >
        {#each $facilities as facility}
            <SelectItem value={facility.id} label={facility.name} />
        {/each}
    </Select>

    <div>
        <Checkbox id="active" bind:checked={$form.active} disabled={$isViewing} />
        <Label
            for="active"
            class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
            >Active</Label
        >
    </div>

    <Input properties={{ id: 'occupation', label: 'Occupation' }} bind:value={$form.occupation} />

    <div>
        <div class="ml-auto text-xs text-muted-foreground">Last login {$form.last_login}</div>
    </div>

    <Button type="submit" disabled={$isViewing || $hasErrors}>Save</Button>
</form>
