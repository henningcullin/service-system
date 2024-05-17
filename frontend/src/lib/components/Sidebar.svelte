<script lang="ts">
    import * as Accordion from '$components/ui/accordion/index.js';
    import Separator from '$components/ui/separator/separator.svelte';
    import { SidebarOpen } from '$lib/stores';
    import { Link } from 'svelte-navigator';
    import LayoutDashboard from 'lucide-svelte/icons/layout-dashboard';
    import ClipboardList from 'lucide-svelte/icons/clipboard-list';
    import Flag from 'lucide-svelte/icons/flag';
    import CPU from 'lucide-svelte/icons/cpu';

    $: IsSidebarOpen = $SidebarOpen;

    function close() {
        SidebarOpen.set(false);
    }
</script>

<sidebar class={IsSidebarOpen ? 'open' : ''}>
    <Link to="/" on:click={close}><LayoutDashboard style="display:inherit" /> Mainmenu</Link>
    <Accordion.Root class="w-full">
        <Accordion.Item value="tasks">
            <Accordion.Trigger><ClipboardList />Tasks</Accordion.Trigger>
            <Accordion.Content>
                <div class="grid grid-cols-1 gap-2">
                    <Link to="/tasks" on:click={close}>Tasks</Link>
                    <Separator />
                    <Link to="/task/?new=true" on:click={close}>Create Task</Link>
                    <Separator />
                    <Link to="/task/panel/" on:click={close}>Task Panel</Link>
                </div>
            </Accordion.Content>
        </Accordion.Item>
        <Accordion.Item value="reports">
            <Accordion.Trigger><Flag /> Reports</Accordion.Trigger>
            <Accordion.Content>
                <div class="grid grid-cols-1 gap-2">
                    <Link to="/reports" on:click={close}>Reports</Link>
                    <Separator />
                    <Link to="/report/?new=true" on:click={close}>Create Report</Link>
                    <Separator />
                    <Link to="/report/panel/" on:click={close}>Report Panel</Link>
                </div>
            </Accordion.Content>
        </Accordion.Item>
        <Accordion.Item value="machines">
            <Accordion.Trigger><CPU />Machines</Accordion.Trigger>
            <Accordion.Content>
                <div class="grid grid-cols-1 gap-2">
                    <Link to="/machines" on:click={close}>Machines</Link>
                    <Separator />
                    <Link to="/machine/?new=true" on:click={close}>Create Machine</Link>
                    <Separator />
                    <Link to="/machine/panel/" on:click={close}>Machine Panel</Link>
                </div>
            </Accordion.Content>
        </Accordion.Item>
    </Accordion.Root>
</sidebar>

<style>
    sidebar {
        position: absolute;
        top: 0;
        width: 14em;
        height: 100%;
        transition: left 0.3s;
        background-color: var(--secondary);
        display: flex;
        flex-direction: column;
        text-align: center;
    }
</style>
