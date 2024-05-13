<script>
    import { account } from '$lib/stores';
    import { SidebarOpen } from '$lib/stores';
    import { Menu } from 'lucide-svelte';
    import { Link } from 'svelte-navigator';
    import UserNav from './UserNav.svelte';

    function toggleSidebar() {
        SidebarOpen.update((state) => !state);
    }
</script>

<div class="border-b">
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
</div>

<!-- <Link class="account-box" to="/account">{$account.first_name}</Link> -->
