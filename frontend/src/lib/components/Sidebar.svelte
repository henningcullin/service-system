<script>
    import { SidebarOpen } from '$lib/stores';
    import { Link } from 'svelte-navigator';

    $: IsSidebarOpen = $SidebarOpen;
</script>

<sidebar class={IsSidebarOpen ? 'open' : ''}>
    <Link to="/">Mainmenu</Link>
    <Link to="/login">Login</Link>
    <Link to="/machines">Machines</Link>
</sidebar>

<style>
    sidebar {
        position: absolute;
        top: 0;
        width: 14em;
        height: 100%;
        transition: left 0.3s;
        background-color: var(--secondary);
    }
</style>
