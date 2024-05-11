<script>
    import { SidebarOpen } from '$lib/stores';

    function toggleSidebar() {
        SidebarOpen.update((state) => !state);
    }
</script>

<header>
    <button on:click|stopPropagation={toggleSidebar}> Open Sidebar </button>
</header>

<style>
    header {
        width: 100%;
        position: fixed;
        top: 0;
        height: 5dvh;
        background-color: var(--primary);
        box-shadow: 0 0 3px 0 var(--line);
    }
</style>
