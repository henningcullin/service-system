<script>
    import { SidebarOpen } from '$lib/stores';
    import { Link } from 'svelte-navigator';

    $: IsSidebarOpen = $SidebarOpen;
</script>

<sidebar class={IsSidebarOpen ? 'open' : ''}>
    <Link to="/">Mainmenu</Link>
    <Link to="/login">Login</Link>
    <divider />
    <item>
        <Link to="/machine?new=true">Create Machine</Link>
        <Link to="/machines">Machines</Link>
    </item>
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

    item {
        width: 100%;
        display: flex;
        flex-direction: column;
    }
</style>
