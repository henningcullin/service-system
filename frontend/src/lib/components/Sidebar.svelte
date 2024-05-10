<script>
    import { showSidebar } from "$lib/stores";

    $: isSidebarOpen = $showSidebar;
</script>

<sidebar class={isSidebarOpen ? "open" : ""} id="sidebar"> </sidebar>

<style>
    sidebar {
        min-height: 100dvh;
        width: 12em;
        position: fixed;
        left: 0;
        top: 0;
        background-color: white;
        transform: translateX(-100%);
        transition: transform 0.3s ease-in-out;
    }
</style>
