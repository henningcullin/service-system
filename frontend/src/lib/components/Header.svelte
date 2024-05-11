<script>
    import { SidebarOpen } from '$lib/stores';

    function toggleSidebar() {
        SidebarOpen.update((state) => !state);
    }
</script>

<header>
    <button on:click={toggleSidebar}> Open Sidebar </button>
</header>

<style>
    header {
        width: 100%;
        height: 5dvh;
        border: 1px solid red;
    }
</style>
