<script>
    import { SidebarOpen } from '$lib/stores';

    $: IsSidebarOpen = $SidebarOpen;
</script>

<sidebar class={IsSidebarOpen ? 'open' : ''}> </sidebar>

<style>
    sidebar {
        border: 1px solid green;
        position: absolute;
        top: 0;
        width: 14em;
        height: 100%;
        transition: left 0.3s;
    }
</style>
