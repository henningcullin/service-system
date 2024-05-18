<script>
    import UserTable from '$components/UsersTable/table.svelte';
    import { getUsers } from '$utils';
    getUsers();
</script>

<div class="hidden h-full flex-1 flex-col space-y-8 p-8 md:flex">
    <div class="flex items-center justify-between space-y-2">
        <div>
            <h2 class="text-2xl font-bold tracking-tight">Users</h2>
        </div>
    </div>
    <UserTable />
</div>
