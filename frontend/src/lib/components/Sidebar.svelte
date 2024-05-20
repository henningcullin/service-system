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

    $: anyTaskPerms =
        $account?.role?.task_view ||
        $account?.role?.task_create ||
        $account?.role?.task_edit ||
        $account?.role?.task_delete;

    $: anyReportPerms =
        $account?.role?.report_view ||
        $account?.role?.report_create ||
        $account?.role?.report_edit ||
        $account?.role?.report_delete;

    $: anyMachinePerms =
        $account?.role?.machine_view ||
        $account?.role?.machine_create ||
        $account?.role?.machine_edit ||
        $account?.role?.machine_delete;

    $: anyUserPerms =
        $account?.role?.user_view ||
        $account?.role?.user_create ||
        $account?.role?.user_edit ||
        $account?.role?.user_delete;

    $: anyFacilityPerms =
        $account?.role?.facility_view ||
        $account?.role?.facility_create ||
        $account?.role?.facility_edit ||
        $account?.role?.facility_delete;

    function close() {
        SidebarOpen.set(false);
    }
</script>

<sidebar class={IsSidebarOpen ? 'open' : ''}>
    <Link to="/" on:click={close}><LayoutDashboard style="display:inherit" /> Mainmenu</Link>
    <Accordion.Root class="w-full">
        {#if anyTaskPerms}
            <Accordion.Item value="tasks">
                <Accordion.Trigger><ClipboardList />Tasks</Accordion.Trigger>
                <Accordion.Content>
                    <div class="grid grid-cols-1 gap-2">
                        {#if $account?.role?.task_view}
                            <Link to="/tasks" on:click={close}>Tasks</Link>
                            <Separator />
                        {/if}
                        {#if $account?.role?.task_create}
                            <Link to="/task/?new=true" on:click={close}>Create Task</Link>
                            <Separator />
                        {/if}
                        <Link to="/task/panel/" on:click={close}>Task Panel</Link>
                    </div>
                </Accordion.Content>
            </Accordion.Item>
        {/if}
        {#if anyReportPerms}
            <Accordion.Item value="reports">
                <Accordion.Trigger><Flag /> Reports</Accordion.Trigger>
                <Accordion.Content>
                    <div class="grid grid-cols-1 gap-2">
                        {#if $account?.role?.report_view}
                            <Link to="/reports" on:click={close}>Reports</Link>
                            <Separator />
                        {/if}
                        {#if $account?.role?.report_create}
                            <Link to="/report/?new=true" on:click={close}>Create Report</Link>
                            <Separator />
                        {/if}
                        <Link to="/report/panel/" on:click={close}>Report Panel</Link>
                    </div>
                </Accordion.Content>
            </Accordion.Item>
        {/if}
        {#if anyMachinePerms}
            <Accordion.Item value="machines">
                <Accordion.Trigger><CPU />Machines</Accordion.Trigger>
                <Accordion.Content>
                    <div class="grid grid-cols-1 gap-2">
                        {#if $account?.role?.machine_view}
                            <Link to="/machines" on:click={close}>Machines</Link>
                            <Separator />
                        {/if}
                        {#if $account?.role?.machine_create}
                            <Link to="/machine/?new=true" on:click={close}>Create Machine</Link>
                            <Separator />
                        {/if}
                        <Link to="/machine/panel/" on:click={close}>Machine Panel</Link>
                    </div>
                </Accordion.Content>
            </Accordion.Item>
        {/if}
        {#if anyUserPerms}
            <Accordion.Item value="users">
                <Accordion.Trigger><Users />Users</Accordion.Trigger>
                <Accordion.Content>
                    <div class="grid grid-cols-1 gap-2">
                        {#if $account?.role?.user_view}
                            <Link to="/users" on:click={close}>Users</Link>
                            <Separator />
                        {/if}
                        {#if $account?.role?.user_create}
                            <Link to="/user/?new=true" on:click={close}>Create User</Link>
                            <Separator />
                        {/if}
                        <Link to="/role/" on:click={close}>Roles</Link>
                    </div>
                </Accordion.Content>
            </Accordion.Item>
        {/if}
    </Accordion.Root>
    {#if anyFacilityPerms}
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
