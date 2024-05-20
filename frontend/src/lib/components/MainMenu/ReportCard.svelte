<script>
    import * as Card from '$components/ui/card';
    import { Link } from 'svelte-navigator';
    import { Separator } from '$components/ui/separator';
    export let report;
</script>

<Card.Root>
    <Card.Header>
        <Card.Title
            ><b class="ml-auto text-xs text-muted-foreground">Title </b><Link to="/report/{report?.id}"
                >{report?.title}</Link
            ></Card.Title
        >
    </Card.Header>
    <Card.Content>
        <p>
            <b class="ml-auto text-xs text-muted-foreground">Type</b>
            <Link to="/report/panel/?type={report?.report_type?.id}">{report?.report_type?.name}</Link>
        </p>
        <p>
            <b class="ml-auto text-xs text-muted-foreground">Status</b>
            <Link to="/report/panel/?status={report?.status?.id}">{report?.status?.name}</Link>
        </p>
        {#if report?.machine?.id}
            <p>
                <b class="ml-auto text-xs text-muted-foreground">Machine</b>

                <Link to="/machine/{report?.machine?.id}">{report?.machine?.name}</Link>
            </p>
        {/if}
        <p>
            <b class="ml-auto text-xs text-muted-foreground">Creator</b>
            <Link to="/user/{report?.creator?.id}">
                {report?.creator?.first_name}
                {report?.creator?.last_name}
            </Link>
        </p>
        <Separator class="my-1" />
        <h2 class="ml-auto text-xs text-muted-foreground">Description</h2>
        <span>
            {report?.description}
        </span>
    </Card.Content>
</Card.Root>
