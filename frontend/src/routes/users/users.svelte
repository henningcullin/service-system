<script>
    import UserTable from '$components/UsersTable/table.svelte';
    import { users } from '$stores';
    import UserCard from '$components/MobileCards/UserCard.svelte';
    import { getUsers } from '$utils';
    getUsers();

    document.title = 'Users';
</script>

<div class="hidden h-full flex-1 flex-col space-y-8 p-8 md:flex">
    <div class="flex items-center justify-between space-y-2">
        <div>
            <h2 class="text-2xl font-bold tracking-tight">Users</h2>
        </div>
    </div>
    <UserTable />
</div>

<div class="h-full flex-1 flex-col space-y-8 p-4 md:hidden">
    <div class="flex items-center justify-between space-y-2">
        <div>
            <h2 class="text-2xl font-bold tracking-tight">Users</h2>
        </div>
    </div>
    {#each $users as user}
        <UserCard {user} />
    {/each}
</div>
