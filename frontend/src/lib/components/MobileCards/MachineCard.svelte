<script>
    import * as Card from '$components/ui/card';
    import { Link } from 'svelte-navigator';
    export let machine;
</script>

<Card.Root>
    <Card.Header>
        <Card.Title
            ><b class="ml-auto text-xs text-muted-foreground">Name </b><Link to="/machine/{machine?.id}"
                >{machine?.name}</Link
            ></Card.Title
        >
    </Card.Header>
    <Card.Content>
        <p>
            <b class="ml-auto text-xs text-muted-foreground">Type </b>
            <Link to="/machine/panel/?type={machine?.machine_type?.id}">{machine?.machine_type?.name}</Link>
        </p>
        <p>
            <b class="ml-auto text-xs text-muted-foreground">Status </b>
            <Link to="/machine/panel/?status={machine?.status?.id}">{machine?.status?.name}</Link>
        </p>
        {#if machine?.facility?.id}
            <p>
                <b class="ml-auto text-xs text-muted-foreground">Facility </b>

                <Link to="/facility/{machine?.facility?.id}">{machine?.facility?.name}</Link>
            </p>
        {/if}
    </Card.Content>
</Card.Root>
