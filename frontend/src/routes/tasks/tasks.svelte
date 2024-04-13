<script>
    
    import { DataHandler, Datatable, Th, ThFilter} from '@vincjo/datatables';
    import { Link, navigate } from 'svelte-navigator';
    import { account, tasks, users, machines} from '$lib/stores.js'
    import { sendDelete, getTasks, getUsers, getMachines } from '$lib/utils.js';

    let lastFetch = false;

    if (!lastFetch) {
        lastFetch = Date.now();
        if (!$tasks.size) getTasks();
        if (!$users.size) getUsers();
        if (!$machines.size) getMachines();
    }

    $: tasksArr = $tasks.size ?  Array.from($tasks.values()) : [];

    let currentPage = 1;
    const cardsPerPage = 6;
    $: pageCount = Math.ceil( (tasksArr.length+1) / cardsPerPage);

    const handler = new DataHandler(tasksArr, {rowsPerPage: 10});
    const rows = handler.getRows();
    $: handler.setRows(tasksArr)

    async function deleteTask() {
        
        if ($account.role === 'Worker') return;

        const id = this ? this.id : null;
        if (!id) return;
        
        const choice = confirm(`Are you sure you want to delete this task`);
        if (!choice) return;

        const response = await sendDelete(`/api/auth/task?id=${id}`);
        if (response.status != 204) return alert('Could not delete');

        tasks.update(prev => prev.delete(id));
    }

    async function editTask() {

        if ($account.role === 'Worker') return;

        const id = this ? this.id : null;
        if (!id) return;

        navigate(`/task?id=${id}&edit=true`);
    }

</script>

<h2> Welcome to the Task page!!</h2>

{#if $account.role !== 'Worker'}
    <div class='menu'>
        <Link to='/task?new=true'>New</Link>
    </div>
{/if}

<div class='mobile-grid'>
    {#each tasksArr.slice((currentPage - 1) * cardsPerPage, currentPage * cardsPerPage) as task}
        <div class='mobile-card'> 
            <Link to='/task?id={task.id}' class='itemLink' >{task.title}</Link>
            <p>{task.description}</p>
            <p>{task.task_type}</p>
            <p class='{task.status}'>{task.status}</p>
            <p class='{task.archived}'>{task.archived}</p>
            <i>{task.created.toLocaleString('en-GB')}</i><br>
            <i>{task.edited.toLocaleString('en-GB')}</i><br>
            <p>{task.creator}</p>
            <p>{task.executor}</p>
            <p>{task.machine}</p>
            {#if $account.role != 'Worker'}
                <button on:click={deleteTask} id='{task.id}' class='cardDeleteButton'/>
                <button on:click={editTask} id='{task.id}' class='cardEditButton'/>
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
    <table>
        <thead>
            <tr>
                <Th {handler} orderBy='id'>Id</Th>
                <Th {handler} orderBy='title'>Name</Th>
                <Th {handler} orderBy='description'>Description</Th>
                <Th {handler} orderBy='task_type'>Type</Th>
                <Th {handler} orderBy='status'>Status</Th>
                <Th {handler} orderBy='archived'>Archived</Th>
                <Th {handler} orderBy='creator'>Creator</Th>
                <Th {handler} orderBy='executor'>Executor</Th>
                <Th {handler} orderBy='machine'>Machine</Th>
                <Th {handler} orderBy='created'>Created</Th>
                <Th {handler} orderBy='edited'>Edited</Th>
                {#if $account.role != 'Worker'}
                    <Th {handler} orderBy='none'>Delete</Th>
                    <Th {handler} orderBy='none'>Edit</Th>
                {/if}
            </tr>
            <tr>
                <ThFilter {handler} filterBy='id'/>
                <ThFilter {handler} filterBy='title'/>
                <ThFilter {handler} filterBy='description'/>
                <ThFilter {handler} filterBy='task_type'/>
                <ThFilter {handler} filterBy='status'/>
                <ThFilter {handler} filterBy='archived'/>
                <ThFilter {handler} filterBy='creator'/>
                <ThFilter {handler} filterBy='executor'/>
                <ThFilter {handler} filterBy='machine'/>
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
                    <td><Link to='/task?id={row.id}' class='itemLink'>{row.id}</Link></td>
                    <td>{row.title}</td>
                    <td>{row.description}</td>
                    <td>{row.task_type}</td>
                    <td class='{row.status}'>{row.status}</td>
                    <td>{row.archived}</td>
                    <td>{row.creator ? row.creator : 'Not Set'}</td>
                    <td>{row.executor ? row.executor : 'Not Set'}</td>
                    <td>{row.machine ? row.machine : 'Not Set'}</td>
                    <td>{row.created.toLocaleString('en-GB')}</td>
                    <td>{row.edited.toLocaleString('en-GB')}</td>
                    {#if $account.role != 'Worker'}
                        <td class='buttonCell'><button class='tableDeleteButton' id='{row.id}' on:click={deleteTask}></button></td>
                        <td class='buttonCell'><button class='tableEditButton' id='{row.id}' on:click={editTask}></button></td>
                    {/if}
                </tr>
            {/each}
        </tbody>
    </table>
</Datatable>

<style>

    .Active {
        color: #40b129;
    }

    .Inactive {
        color: #d84444;
    }

    /* mobile */

    .mobile-grid {
        margin-top: 30px;
        display:grid;
        width:100%;
        grid-template-columns: 1fr 1fr;
        grid-template-rows: 1fr 1fr 1fr;
        gap:6px;
        padding:0.3%;
        min-height: 72dvh;
    }

    .mobile-card {
        padding: 0.2%;
        width:100%;
        height:100%;
        background-color: #353535;
        border-radius: 5px;
    }

    .pagination {
        display:flex;
        margin-top: 25px;
        background-color: #353535;
        padding:1.5%;
        border-radius: 5px;
    }

    .pagination * {
        font-size:1.2em;
    }

    .pagination>div {
        flex:1;
    }

    .cardDeleteButton {
        height: 32px;
        width: 32px;
        float: left;
        margin-bottom: 10px;
        margin-left: 10px;
        margin-top: 10px;
        cursor: pointer;
        background-color: transparent;
        background-image: url('data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAB8ElEQVR4Xu2bYXvDIAiE1/z/37xOtyRNU5U7hK7tc/nMCLw5QK27fCU91/JEur6UJ9Lf5ivFaXTye7AJEMIBZCWfBSEUwCh5VsKRvkalkw6ATfwcbAvErM/jO1IBRAV6hhDlt4IQAHS0ZDc3NA7UDlWJqYB3S/wMyAIxBPDuySOjswvgU5K3INAALEmhNZpl1/twvbhhAK+euLV+EIDOPkIK6NUiW0tZNc36ZeM2FPBd3r+wMbykPd0DahafMgpHDVwrQVSvj2qoJ14mP9R9uB06tuEMMrekEdl74xMAlL6XMOp/1s4bn0sBtfqXwQktEgxrY9U04q8F2QXg9yjpyQAi3ikADQJSANp8mBrbbdelQqtcEH+IzRY/Y3vMWQpIVcDqXApoTAxEsoiNSmAlwMBSDzgQUBPMb4LXsmpcHkAjkkVs1APUA/4IMGpRE1QTvBHQFMifAu2zA6RmERtNAU0BTQGNQa0DtBDSSlBLYWbNoL2A9gLaC+wEtBnSZggkwHbZo731w0jvl2bLxzF0Nr7tb1NKAGQaaiYAhcDdfAf/xU4KQHXYujRpXVtBfc/azcTmVMDtCu1/Q2DvBp9hwwBae25Pzc1+7d4ZoDcWCoAFISq5GT+sIvMBPPHCOZt8BU0DQCTo/YIzt489ydc4fwCoZ1BugYj1SgAAAABJRU5ErkJggg==');
        background-repeat: no-repeat;
        background-position: center;
        background-size: 32px, 32px;
    }
    
    .cardEditButton {
        height: 32px;
        width: 32px;
        float: right;
        margin-bottom: 10px;
        margin-right: 10px;
        margin-top: 10px;
        cursor: pointer;
        background-color: transparent;
        background-image: url('data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAC8ElEQVR4Xu2biXKDIBCGNXn/R44WQrAcC+yliIMznU5bFvm/PbiadXngs5snlmV/XJfVPKnc7Bej88jFx4pSCI8C0BLvUYQQHgMAKz6F8AgAVPEOwmZqwjsvCqPVgJp4H+pQG1cW3dewDyRsM2peX/8ui/Hvoa8EalgA2LAPCx5kMyQArPi04O2bWR8kiocDQBXfyu8IAL9zX1Jaryv93WeuKUrAas1b0Qvef79LYYjNIsGVJLGDIGDEN0ElEOx7vgD4npfIhGydx1IAFPFNCL8G/h0rSbw00hG8yuKDcE5EQN1ioVUB1PIRoUXcBCsifRHFDgTQW3grLaWFMgSWAbiL+FK2aYq3INb9YxYHdu2IyCtxTCM6oIRv2B3XLogAuAIjxqzWhCuCa+ciIDk+6pUCXBFcuyPi7wCAK4Jrd6siyBXBtUvztWsKZCKCtY52tS8Vqm4AuB7k2uEAGA9ccUzGFcG1q01Tl0cAVwTXrjVHXwqAK2I3qzV30pc/0mn7MgB88ek1l4HwWyeXdo4UKJcA0BR/HGcnJ0fc9czpADTF1/YrwwHQnuctgHAHiU2D1WwGd3+RYL9jDVvVtbSnt4N8CQ8+oXffMgKg8Nf2vIehAOBj+nqfHgElANxaoQjAdXV2CkiPvEuppxABfQDknsddkqQg4n7c1TemTp02DboBxSd71vu1UI/26ZVCOUQRDIW2rxNyUBjvhW1ulwKZp0EKtMuOGpT7A0C4VFKAhwYgEX7babBU7DTEqhZBvxSubTQQ0du9iUIK9L8YkVBUAKC/EJIIotpOAG43fDzYWnPaSpDqQWn7GQEzAmYKzBowi+CcBeY0ONcB1D3NXAhxV1DSlZu2PVfHIyKAegETTZcSY20vcvuTaCgeU2N3U9xBa9lJT57Q5/S0AeenvTT7Ruv2OTv6huuaD0z8eCDG/b1L2cyo4H+IwWGkRC/vIzNFB6Mkwipsn8cnGfn9UMTbgSh9aArQBEHi60K5niredvoHFzyrR767xa8AAAAASUVORK5CYII=');
        cursor: pointer;
        background-repeat: no-repeat;
        background-position: center;
        background-size: 32px, 32px;
    }

    /* desktop */

    td {
        height: 48px;
        border-bottom: 1px solid #afafafbd;
        font-size:1.1em;
    }

    tr>td:first-child {
        width:20%;
    }

    :global(.itemLink) {
        color: #2d53bd;
    }
    
    .tableDeleteButton, .tableEditButton {
        height:48px;
        width:48px;
        background-color: transparent;
        background-repeat: no-repeat;
        background-position: center;
        background-size: 32px, 32px;
    }

    .tableDeleteButton {
        background-image: url('data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAB8ElEQVR4Xu2bYXvDIAiE1/z/37xOtyRNU5U7hK7tc/nMCLw5QK27fCU91/JEur6UJ9Lf5ivFaXTye7AJEMIBZCWfBSEUwCh5VsKRvkalkw6ATfwcbAvErM/jO1IBRAV6hhDlt4IQAHS0ZDc3NA7UDlWJqYB3S/wMyAIxBPDuySOjswvgU5K3INAALEmhNZpl1/twvbhhAK+euLV+EIDOPkIK6NUiW0tZNc36ZeM2FPBd3r+wMbykPd0DahafMgpHDVwrQVSvj2qoJ14mP9R9uB06tuEMMrekEdl74xMAlL6XMOp/1s4bn0sBtfqXwQktEgxrY9U04q8F2QXg9yjpyQAi3ikADQJSANp8mBrbbdelQqtcEH+IzRY/Y3vMWQpIVcDqXApoTAxEsoiNSmAlwMBSDzgQUBPMb4LXsmpcHkAjkkVs1APUA/4IMGpRE1QTvBHQFMifAu2zA6RmERtNAU0BTQGNQa0DtBDSSlBLYWbNoL2A9gLaC+wEtBnSZggkwHbZo731w0jvl2bLxzF0Nr7tb1NKAGQaaiYAhcDdfAf/xU4KQHXYujRpXVtBfc/azcTmVMDtCu1/Q2DvBp9hwwBae25Pzc1+7d4ZoDcWCoAFISq5GT+sIvMBPPHCOZt8BU0DQCTo/YIzt489ydc4fwCoZ1BugYj1SgAAAABJRU5ErkJggg==');
    }

    .tableEditButton {
        background-image: url('data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAC8ElEQVR4Xu2biXKDIBCGNXn/R44WQrAcC+yliIMznU5bFvm/PbiadXngs5snlmV/XJfVPKnc7Bej88jFx4pSCI8C0BLvUYQQHgMAKz6F8AgAVPEOwmZqwjsvCqPVgJp4H+pQG1cW3dewDyRsM2peX/8ui/Hvoa8EalgA2LAPCx5kMyQArPi04O2bWR8kiocDQBXfyu8IAL9zX1Jaryv93WeuKUrAas1b0Qvef79LYYjNIsGVJLGDIGDEN0ElEOx7vgD4npfIhGydx1IAFPFNCL8G/h0rSbw00hG8yuKDcE5EQN1ioVUB1PIRoUXcBCsifRHFDgTQW3grLaWFMgSWAbiL+FK2aYq3INb9YxYHdu2IyCtxTCM6oIRv2B3XLogAuAIjxqzWhCuCa+ciIDk+6pUCXBFcuyPi7wCAK4Jrd6siyBXBtUvztWsKZCKCtY52tS8Vqm4AuB7k2uEAGA9ccUzGFcG1q01Tl0cAVwTXrjVHXwqAK2I3qzV30pc/0mn7MgB88ek1l4HwWyeXdo4UKJcA0BR/HGcnJ0fc9czpADTF1/YrwwHQnuctgHAHiU2D1WwGd3+RYL9jDVvVtbSnt4N8CQ8+oXffMgKg8Nf2vIehAOBj+nqfHgElANxaoQjAdXV2CkiPvEuppxABfQDknsddkqQg4n7c1TemTp02DboBxSd71vu1UI/26ZVCOUQRDIW2rxNyUBjvhW1ulwKZp0EKtMuOGpT7A0C4VFKAhwYgEX7babBU7DTEqhZBvxSubTQQ0du9iUIK9L8YkVBUAKC/EJIIotpOAG43fDzYWnPaSpDqQWn7GQEzAmYKzBowi+CcBeY0ONcB1D3NXAhxV1DSlZu2PVfHIyKAegETTZcSY20vcvuTaCgeU2N3U9xBa9lJT57Q5/S0AeenvTT7Ruv2OTv6huuaD0z8eCDG/b1L2cyo4H+IwWGkRC/vIzNFB6Mkwipsn8cnGfn9UMTbgSh9aArQBEHi60K5niredvoHFzyrR767xa8AAAAASUVORK5CYII=');
    }
    
    .buttonCell {
        width:48px;
        height:48px;
    }

    :global(section) {
        display: none;
    }

    @media (min-width: 1200px) {
        :global(section) {
            display:inherit;
        }
        .mobile-grid {
            display:none;
        }
        .pagination {
            display:none;
        }
    }

</style>