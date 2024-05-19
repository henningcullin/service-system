<script>
    import UserTable from '$components/UsersTable/table.svelte';
    import { users } from '$stores';
    import * as Card from '$components/ui/card';
    import { getUsers } from '$utils';
    import { Link } from 'svelte-navigator';
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

<div class="h-full flex-1 flex-col space-y-8 p-4 md:hidden">
    <div class="flex items-center justify-between space-y-2">
        <div>
            <h2 class="text-2xl font-bold tracking-tight">Users</h2>
        </div>
    </div>
    {#each $users as user}
        <Card.Root class="w-[100%]">
            <Card.Header>
                <Card.Title><Link to="/user/{user?.id}">{user?.first_name} {user?.last_name}</Link></Card.Title>
                <Card.Description><a href="mailto:{user?.email}">{user?.email}</a></Card.Description>
            </Card.Header>
        </Card.Root>
    {/each}
</div>
