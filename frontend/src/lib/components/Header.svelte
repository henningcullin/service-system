<script>
    import { account } from '$lib/stores';
    import { SidebarOpen } from '$lib/stores';
    import { Link } from 'svelte-navigator';

    function toggleSidebar() {
        SidebarOpen.update((state) => !state);
    }
</script>

<header>
    <button on:click|stopPropagation={toggleSidebar}> Open Sidebar </button>
    {#if $account.id}
        <Link to="/account">{$account.first_name}</Link>
    {/if}
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
