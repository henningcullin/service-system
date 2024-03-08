<script>

    import { DataHandler, Datatable, Th, ThFilter} from '@vincjo/datatables';

    import { Link, navigate } from 'svelte-navigator';
    import { machines } from '../../lib/stores.js'
    import { sendDelete } from '../../lib/helpers.js';

    machines.subscribe((arr) => {
        if (!arr.length) {
            console.log('machines gotten');
            getMachines();
        }
    });


    // @ts-ignore
    const handler = new DataHandler($machines, {rowsPerPage: 10});
    const rows = handler.getRows();
    
    $: handler.setRows($machines)

    async function getMachines() {
        try {
            const response = await fetch('/api/auth/machines');
            const data = await response.json();

            // @ts-ignore
            const formatted = data.map((machine) => {
                return {
                    id: machine.id,
                    name: machine.name,
                    make: machine.make,
                    type: machine.machine_type,
                    status: machine.status,
                    created: new Date(machine.created),
                    edited: new Date(machine.edited),
                };
            })

            machines.set(formatted);
        } catch (error) {
            console.log('Could not fetch products', error);
        }
    };

    async function deleteMachine() {
        
        // @ts-ignore
        const id = this ? this.id : null;

        if (!id) return;

        const response = await sendDelete(`/api/auth/machine?id=${id}`);

        if (response.status != 204) alert('Could not delete');

        // @ts-ignore
        machines.update(prev => prev.filter(m => m.id != id));
    }

    async function editMachine() {

    }

</script>

<div class='segment'>
    <h2> Welcome to the Machine page!!</h2>
    
    <div class='menu'>
        <Link to='/machines/new'>New</Link>
    </div>

    <div class='mobile-grid'>
        {#each $machines as machine}
            <div class='mobile-card'> 
                <Link to='/machine/{machine.id}'>{machine.name}</Link>
                <p>Make: {machine.make}</p>
                <p>Type: {machine.type}</p>
                <p>Status: {machine.status}</p>
                <i title={machine.created.toLocaleTimeString()}>Created: {machine.created.toLocaleDateString()}</i><br>
                <i title={machine.edited.toLocaleTimeString()}>Edited: {machine.edited.toLocaleDateString()}</i><br>
                <button on:click={deleteMachine} class='cardDeleteButton'/>
                <button on:click={editMachine} class='cardEditButton'/>
            </div>
        {/each}
    </div>
    
    <Datatable {handler}>
        <table>
            <thead>
                <tr>
                    <Th {handler} orderBy='id'>Id</Th>
                    <Th {handler} orderBy='name'>Name</Th>
                    <Th {handler} orderBy='make'>Make</Th>
                    <Th {handler} orderBy='type'>Type</Th>
                    <Th {handler} orderBy='status'>Status</Th>
                    <Th {handler} orderBy='created'>Created</Th>
                    <Th {handler} orderBy='edited'>Edited</Th>
                    <Th {handler} orderBy='none'>Delete</Th>
                    <Th {handler} orderBy='none'>Edit</Th>
                </tr>
                <tr>
                    <ThFilter {handler} filterBy='id'/>
                    <ThFilter {handler} filterBy='name'/>
                    <ThFilter {handler} filterBy='make'/>
                    <ThFilter {handler} filterBy='type'/>
                    <ThFilter {handler} filterBy='status'/>
                    <ThFilter {handler} filterBy='created'/>
                    <ThFilter {handler} filterBy='edited'/>
                    <ThFilter {handler} filterBy='none'/>
                    <ThFilter {handler} filterBy='none'/>
                </tr>
            </thead>
            <tbody>
                {#each $rows as row}
                    <tr>
                        <td><Link to='/machine/{row.id}' class='itemLink'>{row.id}</Link></td>
                        <td>{row.name}</td>
                        <td>{row.make}</td>
                        <td>{row.type}</td>
                        <td class='{row.status}'>{row.status}</td>
                        <td>{row.created.toLocaleString('en-GB')}</td>
                        <td>{row.edited.toLocaleString('en-GB')}</td>
                        <td class='buttonCell'><button class='tableDeleteButton' id='{row.id}' on:click={deleteMachine}><img src='data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAB8ElEQVR4Xu2bYXvDIAiE1/z/37xOtyRNU5U7hK7tc/nMCLw5QK27fCU91/JEur6UJ9Lf5ivFaXTye7AJEMIBZCWfBSEUwCh5VsKRvkalkw6ATfwcbAvErM/jO1IBRAV6hhDlt4IQAHS0ZDc3NA7UDlWJqYB3S/wMyAIxBPDuySOjswvgU5K3INAALEmhNZpl1/twvbhhAK+euLV+EIDOPkIK6NUiW0tZNc36ZeM2FPBd3r+wMbykPd0DahafMgpHDVwrQVSvj2qoJ14mP9R9uB06tuEMMrekEdl74xMAlL6XMOp/1s4bn0sBtfqXwQktEgxrY9U04q8F2QXg9yjpyQAi3ikADQJSANp8mBrbbdelQqtcEH+IzRY/Y3vMWQpIVcDqXApoTAxEsoiNSmAlwMBSDzgQUBPMb4LXsmpcHkAjkkVs1APUA/4IMGpRE1QTvBHQFMifAu2zA6RmERtNAU0BTQGNQa0DtBDSSlBLYWbNoL2A9gLaC+wEtBnSZggkwHbZo731w0jvl2bLxzF0Nr7tb1NKAGQaaiYAhcDdfAf/xU4KQHXYujRpXVtBfc/azcTmVMDtCu1/Q2DvBp9hwwBae25Pzc1+7d4ZoDcWCoAFISq5GT+sIvMBPPHCOZt8BU0DQCTo/YIzt489ydc4fwCoZ1BugYj1SgAAAABJRU5ErkJggg==' alt='Delete'></button></td>
                        <td class='buttonCell'><button class='tableEditButton' id='{row.id}' on:click={editMachine}><img src='data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAC8ElEQVR4Xu2biXKDIBCGNXn/R44WQrAcC+yliIMznU5bFvm/PbiadXngs5snlmV/XJfVPKnc7Bej88jFx4pSCI8C0BLvUYQQHgMAKz6F8AgAVPEOwmZqwjsvCqPVgJp4H+pQG1cW3dewDyRsM2peX/8ui/Hvoa8EalgA2LAPCx5kMyQArPi04O2bWR8kiocDQBXfyu8IAL9zX1Jaryv93WeuKUrAas1b0Qvef79LYYjNIsGVJLGDIGDEN0ElEOx7vgD4npfIhGydx1IAFPFNCL8G/h0rSbw00hG8yuKDcE5EQN1ioVUB1PIRoUXcBCsifRHFDgTQW3grLaWFMgSWAbiL+FK2aYq3INb9YxYHdu2IyCtxTCM6oIRv2B3XLogAuAIjxqzWhCuCa+ciIDk+6pUCXBFcuyPi7wCAK4Jrd6siyBXBtUvztWsKZCKCtY52tS8Vqm4AuB7k2uEAGA9ccUzGFcG1q01Tl0cAVwTXrjVHXwqAK2I3qzV30pc/0mn7MgB88ek1l4HwWyeXdo4UKJcA0BR/HGcnJ0fc9czpADTF1/YrwwHQnuctgHAHiU2D1WwGd3+RYL9jDVvVtbSnt4N8CQ8+oXffMgKg8Nf2vIehAOBj+nqfHgElANxaoQjAdXV2CkiPvEuppxABfQDknsddkqQg4n7c1TemTp02DboBxSd71vu1UI/26ZVCOUQRDIW2rxNyUBjvhW1ulwKZp0EKtMuOGpT7A0C4VFKAhwYgEX7babBU7DTEqhZBvxSubTQQ0du9iUIK9L8YkVBUAKC/EJIIotpOAG43fDzYWnPaSpDqQWn7GQEzAmYKzBowi+CcBeY0ONcB1D3NXAhxV1DSlZu2PVfHIyKAegETTZcSY20vcvuTaCgeU2N3U9xBa9lJT57Q5/S0AeenvTT7Ruv2OTv6huuaD0z8eCDG/b1L2cyo4H+IwWGkRC/vIzNFB6Mkwipsn8cnGfn9UMTbgSh9aArQBEHi60K5niredvoHFzyrR767xa8AAAAASUVORK5CYII=' alt='Edit'></button></td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </Datatable>

</div>


<style>

    /* mobile */

    .mobile-grid {
        margin-top: 50px;
        display:grid;
        width:100%;
        grid-template-columns: 1fr 1fr;
        gap:10px;
        padding:0.5%;
    }

    .mobile-card {
        padding: 0.5%;
        width:100%;
        background-color: #353535;
        border-radius: 5px;
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

    .Active {
        color: #40b129;
    }

    .Inactive {
        color: #b42525;
    }

    td {
        height: 48px;
        border-bottom: 1px solid #afafafbd;
        font-size:1.1em;
    }

    :global(td a) {
        color: #2d53bd;
    }

    tr :first-child {
        width:20%;
    }
    
    .tableDeleteButton, .tableEditButton {
        height:48px;
        width:48px;
    }

    .tableDeleteButton img, .tableEditButton img {
        height:32px;
        width:32px;
    }
    
    .buttonCell {
        width:32px;
        height:32px;
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
    }

</style>