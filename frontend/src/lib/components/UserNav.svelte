<script>
    import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
    import * as Avatar from '$lib/components/ui/avatar/index.js';
    import { Switch } from '$lib/components/ui/switch/index.js';
    import { Label } from '$lib/components/ui/label/index.js';
    import { Button } from '$lib/components/ui/button/index.js';
    import { account } from '$stores';
    import { navigate } from 'svelte-navigator';
    import { setMode, mode } from 'mode-watcher';

    let initials = '';

    $: {
        initials = $account.first_name.at(0).toUpperCase() + $account.last_name.at(0).toUpperCase();
    }

    $: isDark = $mode === 'dark';

    $: setMode(isDark ? 'dark' : 'light');

    async function logout() {
        const response = await fetch('/api/auth/logout');
        if (response.status !== 200) return alert('Failed to log out');
        account.set({});
        navigate('/login');
    }
</script>

<DropdownMenu.Root>
    <DropdownMenu.Trigger asChild let:builder>
        <Button variant="ghost" builders={[builder]} class="relative h-8 w-8 rounded-full">
            <Avatar.Root class="h-8 w-8">
                <Avatar.Fallback>{initials}</Avatar.Fallback>
            </Avatar.Root>
        </Button>
    </DropdownMenu.Trigger>
    <DropdownMenu.Content class="w-56" align="end">
        <DropdownMenu.Label class="font-normal">
            <div class="flex flex-col space-y-1">
                <p class="text-sm font-medium leading-none">{$account.first_name}</p>
                <p class="text-xs leading-none text-muted-foreground">{$account.email}</p>
            </div>
        </DropdownMenu.Label>
        <DropdownMenu.Separator />
        <div
            class="relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none data-[disabled]:pointer-events-none data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground data-[disabled]:opacity-50"
        >
            <Switch id="toggle-color-mode" style="margin-right: 1em;" bind:checked={isDark} />
            <Label for="toggle-color-mode">Dark mode</Label>
        </div>
        <DropdownMenu.Separator />
        <DropdownMenu.Group>
            <DropdownMenu.Item>Profile</DropdownMenu.Item>
            <DropdownMenu.Item>Settings</DropdownMenu.Item>
        </DropdownMenu.Group>
        <DropdownMenu.Separator />
        <DropdownMenu.Item on:click={logout}>Log out</DropdownMenu.Item>
    </DropdownMenu.Content>
</DropdownMenu.Root>
