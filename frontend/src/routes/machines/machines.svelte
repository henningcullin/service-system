<script>

    import { DataHandler, Datatable, Th, ThFilter} from '@vincjo/datatables';
    import { Link, navigate } from 'svelte-navigator';
    import { account, machines } from '$lib/stores.js'
    import { sendDelete, getMachines} from '$lib/utils.js';

    let lastFetch = false;

    if (!lastFetch) {
        lastFetch = Date.now();
        if (!$machines.size) getMachines();
    }

    $: machineArr = $machines.size ? Array.from($machines.values()) : [];

    let currentPage = 1;
    const cardsPerPage = 6;
    $: pageCount = Math.ceil( (machineArr.length+1) / cardsPerPage);

    const handler = new DataHandler(machineArr, {rowsPerPage: 10});
    const rows = handler.getRows();
    $: handler.setRows(machineArr)

    async function deleteMachine() {
        if ($account.role === 'Worker') return;

        const id = this ? this.id : null;
        if (!id) return;
        
        const choice = confirm(`Are you sure you want to delete this machine`);
        if (!choice) return;

        const response = await sendDelete(`/api/auth/machine?id=${id}`);
        if (response.status != 204) return alert('Could not delete');

        machines.update(prev => { 
            prev.delete(id);
            return prev;
        });
    }

    function editMachine() {
        if ($account.role === 'Worker') return;

        const id = this ? this.id : null;
        if (!id) return;

        return navigate(`/machine?id=${id}&edit=true`);
    }

</script>

<h2> Welcome to the Machine page!!</h2>

{#if $account.role !== 'Worker'}
    <div class='menu'>
        <Link to='/machine?new=true'>New</Link>
    </div>
{/if}

<div class='mobile-grid'>
    {#each machineArr.slice((currentPage - 1) * cardsPerPage, currentPage * cardsPerPage) as machine}
        <div class='mobile-card'> 
            <Link to='/machine?id={machine.id}' class='itemLink' >{machine.name}</Link>
            <p>{machine.make}</p>
            <p>{machine.machine_type}</p>
            <p class='{machine.status}'>{machine.status}</p>
            <i>{machine.created.toLocaleString('en-GB')}</i><br>
            <i>{machine.edited.toLocaleString('en-GB')}</i><br>
            {#if $account.role != 'Worker'}
                <button on:click={deleteMachine} id='{machine.id}' class='cardDeleteButton'/>
                <button on:click={editMachine} id='{machine.id}' class='cardEditButton'/>
            {/if}
        </div>
    {/each}
</div>

<div class='pagination'>
    {#if currentPage > 1}
        <div><button on:click={() => currentPage -= 1}>Previous</button></div>
        <div><button on:click={() => currentPage = 1}>1</button></div>
    {:else}
        <div><button disabled>Previous</button></div>
    {/if}

    {#if currentPage - 1 > 1}
        <div><button on:click={() => currentPage -= 1}>{currentPage - 1}</button></div>
    {/if}

    <div><button disabled>{currentPage}</button></div>

    {#if currentPage + 1 <= pageCount}
        <div><button on:click={() => currentPage += 1}>{currentPage + 1}</button></div>
    {/if}

    {#if currentPage < pageCount-1}
        <div><button on:click={() => currentPage = pageCount}>{pageCount}</button></div>
    {/if}

    {#if currentPage < pageCount}
        <div><button on:click={() => currentPage += 1}>Next</button></div>
    {:else}
    <div><button disabled>Next</button></div>
    {/if}
</div>

<Datatable {handler}>
    <table class="ui celled table">
        <thead>
            <tr>
                <Th {handler} orderBy='id'>Id</Th>
                <Th {handler} orderBy='name'>Name</Th>
                <Th {handler} orderBy='make'>Make</Th>
                <Th {handler} orderBy='machine_type'>Type</Th>
                <Th {handler} orderBy='status'>Status</Th>
                <Th {handler} orderBy='created'>Created</Th>
                <Th {handler} orderBy='edited'>Edited</Th>
                {#if $account.role != 'Worker'}
                    <Th {handler} orderBy='none'>Delete</Th>
                    <Th {handler} orderBy='none'>Edit</Th>
                {/if}
            </tr>
            <tr>
                <ThFilter {handler} filterBy='id'/>
                <ThFilter {handler} filterBy='name'/>
                <ThFilter {handler} filterBy='make'/>
                <ThFilter {handler} filterBy='machine_type'/>
                <ThFilter {handler} filterBy='status'/>
                <ThFilter {handler} filterBy='created'/>
                <ThFilter {handler} filterBy='edited'/>
                {#if $account.role != 'Worker'}
                    <ThFilter {handler} filterBy='none'/>
                    <ThFilter {handler} filterBy='none'/>
                {/if}
            </tr>
        </thead>
        <tbody>
            {#each $rows as row}
                <tr>
                    <td><Link to='/machine?id={row.id}' class='itemLink'>{row.id}</Link></td>
                    <td>{row.name}</td>
                    <td>{row.make}</td>
                    <td>{row.machine_type}</td>
                    <td class='{row.status}'>{row.status}</td>
                    <td>{row.created.toLocaleString('en-GB')}</td>
                    <td>{row.edited.toLocaleString('en-GB')}</td>
                    {#if $account.role != 'Worker'}
                        <td class='buttonCell'><button class='tableDeleteButton' id='{row.id}' on:click={deleteMachine}></button></td>
                        <td class='buttonCell'><button class='tableEditButton' id='{row.id}' on:click={editMachine}></button></td>
                    {/if}
                </tr>
            {/each}
        </tbody>
    </table>
</Datatable>