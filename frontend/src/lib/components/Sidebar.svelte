<script>
    import { showSidebar } from "$lib/stores";

    $: isSidebarOpen = $showSidebar;
</script>

<sidebar class={isSidebarOpen ? "open" : ""}>
    <button></button>
</sidebar>

<style>
    sidebar {
        min-height: 100dvh;
        width: 10em;
        background-color: red;
        position: fixed;
        left: 0;
        top: 0;
        transform: translateX(-100%);
        transition: transform 0.4s ease-out;
    }
</style>
