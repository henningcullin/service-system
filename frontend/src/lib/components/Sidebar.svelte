<script>
    import * as Accordion from '$components/ui/accordion/index.js';
    import Separator from '$components/ui/separator/separator.svelte';
    import { SidebarOpen, account } from '$lib/stores';
    import { Link } from 'svelte-navigator';
    import LayoutDashboard from 'lucide-svelte/icons/layout-dashboard';
    import ClipboardList from 'lucide-svelte/icons/clipboard-list';
    import Flag from 'lucide-svelte/icons/flag';
    import CPU from 'lucide-svelte/icons/cpu';
    import Users from 'lucide-svelte/icons/users';
    import Factory from 'lucide-svelte/icons/factory';
    $: IsSidebarOpen = $SidebarOpen;

    $: allTaskPerms =
        $account?.role?.task_view &&
        $account?.role?.task_create &&
        $account?.role?.task_edit &&
        $account?.role?.task_delete;

    $: allReportPerms =
        $account?.role?.report_view &&
        $account?.role?.report_create &&
        $account?.role?.report_edit &&
        $account?.role?.report_delete;

    $: allMachinePerms =
        $account?.role?.machine_view &&
        $account?.role?.machine_create &&
        $account?.role?.machine_edit &&
        $account?.role?.machine_delete;

    $: allUserPerms =
        $account?.role?.user_view &&
        $account?.role?.user_create &&
        $account?.role?.user_edit &&
        $account?.role?.user_delete;

    function close() {
        SidebarOpen.set(false);
    }
</script>

<sidebar class={IsSidebarOpen ? 'open' : ''}>
    <Link to="/" on:click={close}><LayoutDashboard style="display:inherit" /> Mainmenu</Link>
    <Accordion.Root class="w-full">
        {#if $account?.role?.task_view}
            <Accordion.Item value="tasks">
                <Accordion.Trigger><ClipboardList />Tasks</Accordion.Trigger>
                <Accordion.Content>
                    <div class="grid grid-cols-1 gap-2">
                        <Link to="/tasks" on:click={close}>Tasks</Link>
                        <Separator />
                        <Link to="/task/?new=true" on:click={close}>Create Task</Link>
                        <Separator />
                        {#if allTaskPerms}
                            <Link to="/task/panel/" on:click={close}>Task Panel</Link>
                        {/if}
                    </div>
                </Accordion.Content>
            </Accordion.Item>
        {/if}
        {#if $account?.role?.report_view}
            <Accordion.Item value="reports">
                <Accordion.Trigger><Flag /> Reports</Accordion.Trigger>
                <Accordion.Content>
                    <div class="grid grid-cols-1 gap-2">
                        <Link to="/reports" on:click={close}>Reports</Link>
                        <Separator />
                        <Link to="/report/?new=true" on:click={close}>Create Report</Link>
                        <Separator />
                        {#if allReportPerms}
                            <Link to="/report/panel/" on:click={close}>Report Panel</Link>
                        {/if}
                    </div>
                </Accordion.Content>
            </Accordion.Item>
        {/if}
        {#if $account?.role?.machine_view}
            <Accordion.Item value="machines">
                <Accordion.Trigger><CPU />Machines</Accordion.Trigger>
                <Accordion.Content>
                    <div class="grid grid-cols-1 gap-2">
                        <Link to="/machines" on:click={close}>Machines</Link>
                        <Separator />
                        <Link to="/machine/?new=true" on:click={close}>Create Machine</Link>
                        <Separator />
                        {#if allMachinePerms}
                            <Link to="/machine/panel/" on:click={close}>Machine Panel</Link>
                        {/if}
                    </div>
                </Accordion.Content>
            </Accordion.Item>
        {/if}
        {#if $account?.role?.user_view}
            <Accordion.Item value="users">
                <Accordion.Trigger><Users />Users</Accordion.Trigger>
                <Accordion.Content>
                    <div class="grid grid-cols-1 gap-2">
                        <Link to="/users" on:click={close}>Users</Link>
                        <Separator />
                        <Link to="/user/?new=true" on:click={close}>Create User</Link>
                        <Separator />
                        {#if allUserPerms}
                            <Link to="/role/" on:click={close}>Roles</Link>
                        {/if}
                    </div>
                </Accordion.Content>
            </Accordion.Item>
        {/if}
    </Accordion.Root>
    {#if $account?.role?.facility_view}
        <Link to="/facility" on:click={close}><Factory style="display:inherit" /> Facilities</Link>
    {/if}
</sidebar>

<style>
    sidebar {
        position: absolute;
        top: 0;
        width: 14em;
        height: 100%;
        transition: left 0.3s;
        background-color: var(--secondary);
        display: block;
    }
</style>
