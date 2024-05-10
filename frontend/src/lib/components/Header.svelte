<script>
    import { showSidebar } from "$lib/stores";

    function toggleSidebar() {
        showSidebar.update((state) => !state);
    }

    $: isSidebarOpen = $showSidebar;
</script>

<header class={isSidebarOpen ? "push" : ""}>
    <button on:click={toggleSidebar}> Open Sidebar </button>
</header>

<style>
    header {
        position: sticky;
        top: 0;
        height: 5dvh;
        display: flex;
        transition: margin-left 0.4s;
    }
</style>
