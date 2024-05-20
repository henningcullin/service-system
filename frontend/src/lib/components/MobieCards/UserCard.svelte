<script>
    import * as Card from '$components/ui/card';
    import { Link } from 'svelte-navigator';

    export let user;
</script>

<Card.Root class="w-[100%]">
    <Card.Header>
        <Card.Title
            ><b class="ml-auto text-xs text-muted-foreground">Name </b><Link to="/user/{user?.id}"
                >{user?.first_name} {user?.last_name}</Link
            ></Card.Title
        >
        <Card.Description
            ><b class="ml-auto text-xs text-muted-foreground">Email </b><a href="mailto:{user?.email}">{user?.email}</a
            ></Card.Description
        >
    </Card.Header>
    <Card.Content>
        {#if user?.phone}
            <p>
                <b class="ml-auto text-xs text-muted-foreground">Phone</b>
                {user?.phone}
            </p>
        {/if}
        <p>
            <b class="ml-auto text-xs text-muted-foreground">Role</b>
            <Link to="/role/{user?.role?.id}">{user?.role?.name}</Link>
        </p>
        {#if user?.facility?.id}
            <p>
                <b class="ml-auto text-xs text-muted-foreground">Facility</b>

                <Link to="/facility/{user?.facility?.id}">{user?.facility?.name}</Link>
            </p>
        {/if}
    </Card.Content>
</Card.Root>
