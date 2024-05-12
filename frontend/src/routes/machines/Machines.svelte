<script>
    import { machines } from '$stores';
    import { getMachines } from '$utils';
    import { Link } from 'svelte-navigator';

    getMachines();
</script>

<container>
    {#each $machines as machine}
        <card>
            <div class="header">
                <Link to="/machine/{machine.id}"
                    >{#if machine.make}
                        {machine.make + ','}
                    {/if}
                    {machine.name}</Link
                >
            </div>
            <div>
                <b>Type</b>
                <Link to="/machine/type/{machine.machine_type.id}">{machine.machine_type.name}</Link>
            </div>
            <div>
                <b>Status</b>
                <Link to="/machine/status/{machine.status.id}">{machine.status.name}</Link>
            </div>
            <div>
                <b>Created</b>
                {new Date(machine.created).toLocaleString()}
            </div>
            <div>
                <b>Edited</b>
                {new Date(machine.edited).toLocaleString()}
            </div>
            {#if machine.facility.id}
                <hr />
                <div>
                    <b>Facility</b>
                    <Link to="/facility/{machine.facility.id}">{machine.facility.name}</Link>
                    {#if machine.facility.address}
                        <div>
                            <b>Address</b>
                            {machine.facility.address}
                        </div>
                    {/if}
                </div>
            {/if}
        </card>
    {/each}
</container>

<style>
    container {
        display: grid;
        grid-template-columns: 1fr 1fr 1fr;
        place-items: center;
        padding: 2%;
        gap: 15px;
    }

    card {
        padding: 2%;
        background-color: var(--secondary);
        min-height: 80%;
        width: 80%;
    }
</style>
