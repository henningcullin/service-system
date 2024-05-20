<script>
    import * as Card from '$components/ui/card';
    import { Link } from 'svelte-navigator';
    import { Separator } from '$components/ui/separator';
    export let task;
</script>

<Card.Root>
    <Card.Header>
        <Card.Title
            ><b class="ml-auto text-xs text-muted-foreground">Title </b><Link to="/task/{task?.id}">{task?.title}</Link
            ></Card.Title
        >
    </Card.Header>
    <Card.Content>
        <p>
            <b class="ml-auto text-xs text-muted-foreground">Type</b>
            <Link to="/task/panel/?type={task?.task_type?.id}">{task?.task_type?.name}</Link>
        </p>
        <p>
            <b class="ml-auto text-xs text-muted-foreground">Status</b>
            <Link to="/task/panel/?status={task?.status?.id}">{task?.status?.name}</Link>
        </p>
        {#if task?.machine?.id}
            <p>
                <b class="ml-auto text-xs text-muted-foreground">Machine</b>

                <Link to="/machine/{task?.machine?.id}">{task?.machine?.name}</Link>
            </p>
        {/if}
        <p>
            <b class="ml-auto text-xs text-muted-foreground">Creator</b>
            <Link to="/user/{task?.creator?.id}">
                {task?.creator?.first_name}
                {task?.creator?.last_name}
            </Link>
        </p>
        <Separator class="my-1" />
        <h2 class="ml-auto text-xs text-muted-foreground">Description</h2>
        <span>
            {task?.description}
        </span>
    </Card.Content>
</Card.Root>
