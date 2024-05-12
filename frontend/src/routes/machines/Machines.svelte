<script>
    import { machines } from '$stores';
    import { getMachines } from '$utils';
    import { Link } from 'svelte-navigator';

    getMachines();
</script>

<list>
    {#each $machines as machine}
        <card>
            <Link to="/machine/{machine.id}"
                >{#if machine.make}
                    {machine.make + ','}
                {/if}
                {machine.name}</Link
            >
            <Link to="/machine/type/{machine.machine_type.id}">{machine.machine_type.name}</Link>
            <Link to="/machine/status/{machine.status.id}">{machine.status.name}</Link>
            {#if machine.facility.id}
                <Link to="/facility/{machine.facility.id}">{machine.facility.name}</Link>
            {/if}
        </card>
    {/each}
</list>

<style>
    list {
    }

    card {
        padding: 10%;
    }
</style>
