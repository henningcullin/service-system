<script>
    import { account } from '$lib/stores';
    import { SidebarOpen } from '$lib/stores';
    import Menu from 'lucide-svelte/icons/menu';
    import UserNav from './UserNav.svelte';

    function toggleSidebar() {
        SidebarOpen.update((state) => !state);
    }
</script>

<header class="border-b">
    <div class="flex h-14 items-center px-4">
        <nav class="flex items-center space-x-4 lg:space-x-6">
            <button
                on:click|stopPropagation={toggleSidebar}
                class="text-sm font-medium transition-colors hover:text-primary"
            >
                <Menu />
            </button>
        </nav>
        <div class="ml-auto flex items-center space-x-4">
            {#if $account.id}
                <UserNav />
            {/if}
        </div>
    </div>
</header>

<style>
    header {
        position: fixed;
        top: 0;
        width: 100%;
        background-color: hsl(var(--background) / var(--tw-bg-opacity));
        z-index: 100;
    }
</style>
