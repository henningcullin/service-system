<script>
    import { showSidebar } from "$lib/stores";

    function toggleSidebar() {
        showSidebar.update((state) => !state);
    }

    $: isSidebarOpen = $showSidebar;
</script>

<header class={isSidebarOpen ? "push" : ""}>
    <button on:click={toggleSidebar}> Open Sidebar </button>
    <a href="/login">Login</a>
</header>

<style>
    header {
        position: sticky;
        top: 0;
        height: 5dvh;
        display: flex;
        background-color: #fefefe;
        border-bottom: 1px solid #aaa;
    }

    header > * {
        flex: 1;
    }
</style>
