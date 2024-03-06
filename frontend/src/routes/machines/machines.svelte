<script>

    import Grid from "gridjs-svelte"
    import { Link, navigate } from "svelte-navigator";
    import { onMount } from 'svelte';
    import { machines } from '../../lib/stores.js'

    onMount(async () => {
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
    });

    const columns = [
        // @ts-ignore
        'Id',
        'Name', 
        'Make', 
        'Type', 
        'Status', 
        // @ts-ignore
        {name:'Created', formatter: (cell) => cell.toLocaleString('en-GB')}, 
        // @ts-ignore
        {name:'Edited', formatter: (cell) => cell.toLocaleString('en-GB')}, 
        'Delete', 
        'Edit'
    ]

    // @ts-ignore
    function handleTableEvent(e) {
        try {
            
            const target = e.detail[0].target;
            if (!target) return;

            const type = target.getAttribute('data-column-id');

            if (!type) return;

            switch (type) {
                case 'id':
                    navigate(`/machine/${target.innerText}`);
                    break;

            }

        } catch (error) {
            console.log(error);
        }
    }

</script>

<div class="segment">
    <h2> Welcome to the Machine page!!</h2>
    
    <div class="menu">
        <Link to="/machines/new">New</Link>
    </div>

    <div class="mobile-grid">
        {#each $machines as machine}
            <div class="mobile-card"> 
                <b>Name: {machine.name}</b>
                <p>Make: {machine.make}</p>
                <p>Type: {machine.machine_type}</p>
                <p>Status: {machine.status}</p>
                <i title={machine.created.toLocaleTimeString()}>Created: {machine.created.toLocaleDateString()}</i><br>
                <i title={machine.edited.toLocaleTimeString()}>Edited: {machine.edited.toLocaleDateString()}</i>
            </div>
        {/each}
    </div>

    <Grid on:rowClick={handleTableEvent} columns={columns} data={$machines} sort={true} search={true}, pagination={{limit: 15}}/>
    
</div>


<style>

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

    :global([data-column-id="id"]) {
        color: #2d53bd;
        cursor: pointer;
    }

    :global(td[data-column-id="delete"]) {
        background-image: url('data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAB8ElEQVR4Xu2bYXvDIAiE1/z/37xOtyRNU5U7hK7tc/nMCLw5QK27fCU91/JEur6UJ9Lf5ivFaXTye7AJEMIBZCWfBSEUwCh5VsKRvkalkw6ATfwcbAvErM/jO1IBRAV6hhDlt4IQAHS0ZDc3NA7UDlWJqYB3S/wMyAIxBPDuySOjswvgU5K3INAALEmhNZpl1/twvbhhAK+euLV+EIDOPkIK6NUiW0tZNc36ZeM2FPBd3r+wMbykPd0DahafMgpHDVwrQVSvj2qoJ14mP9R9uB06tuEMMrekEdl74xMAlL6XMOp/1s4bn0sBtfqXwQktEgxrY9U04q8F2QXg9yjpyQAi3ikADQJSANp8mBrbbdelQqtcEH+IzRY/Y3vMWQpIVcDqXApoTAxEsoiNSmAlwMBSDzgQUBPMb4LXsmpcHkAjkkVs1APUA/4IMGpRE1QTvBHQFMifAu2zA6RmERtNAU0BTQGNQa0DtBDSSlBLYWbNoL2A9gLaC+wEtBnSZggkwHbZo731w0jvl2bLxzF0Nr7tb1NKAGQaaiYAhcDdfAf/xU4KQHXYujRpXVtBfc/azcTmVMDtCu1/Q2DvBp9hwwBae25Pzc1+7d4ZoDcWCoAFISq5GT+sIvMBPPHCOZt8BU0DQCTo/YIzt489ydc4fwCoZ1BugYj1SgAAAABJRU5ErkJggg==');
        cursor: pointer;
        background-repeat: no-repeat;
        background-position: center;
        background-size: 32px, 32px;
    }

    :global(td[data-column-id="edit"]) {
        background-image: url('data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAC8ElEQVR4Xu2biXKDIBCGNXn/R44WQrAcC+yliIMznU5bFvm/PbiadXngs5snlmV/XJfVPKnc7Bej88jFx4pSCI8C0BLvUYQQHgMAKz6F8AgAVPEOwmZqwjsvCqPVgJp4H+pQG1cW3dewDyRsM2peX/8ui/Hvoa8EalgA2LAPCx5kMyQArPi04O2bWR8kiocDQBXfyu8IAL9zX1Jaryv93WeuKUrAas1b0Qvef79LYYjNIsGVJLGDIGDEN0ElEOx7vgD4npfIhGydx1IAFPFNCL8G/h0rSbw00hG8yuKDcE5EQN1ioVUB1PIRoUXcBCsifRHFDgTQW3grLaWFMgSWAbiL+FK2aYq3INb9YxYHdu2IyCtxTCM6oIRv2B3XLogAuAIjxqzWhCuCa+ciIDk+6pUCXBFcuyPi7wCAK4Jrd6siyBXBtUvztWsKZCKCtY52tS8Vqm4AuB7k2uEAGA9ccUzGFcG1q01Tl0cAVwTXrjVHXwqAK2I3qzV30pc/0mn7MgB88ek1l4HwWyeXdo4UKJcA0BR/HGcnJ0fc9czpADTF1/YrwwHQnuctgHAHiU2D1WwGd3+RYL9jDVvVtbSnt4N8CQ8+oXffMgKg8Nf2vIehAOBj+nqfHgElANxaoQjAdXV2CkiPvEuppxABfQDknsddkqQg4n7c1TemTp02DboBxSd71vu1UI/26ZVCOUQRDIW2rxNyUBjvhW1ulwKZp0EKtMuOGpT7A0C4VFKAhwYgEX7babBU7DTEqhZBvxSubTQQ0du9iUIK9L8YkVBUAKC/EJIIotpOAG43fDzYWnPaSpDqQWn7GQEzAmYKzBowi+CcBeY0ONcB1D3NXAhxV1DSlZu2PVfHIyKAegETTZcSY20vcvuTaCgeU2N3U9xBa9lJT57Q5/S0AeenvTT7Ruv2OTv6huuaD0z8eCDG/b1L2cyo4H+IwWGkRC/vIzNFB6Mkwipsn8cnGfn9UMTbgSh9aArQBEHi60K5niredvoHFzyrR767xa8AAAAASUVORK5CYII=');
        cursor: pointer;
        background-repeat: no-repeat;
        background-position: center;
        background-size: 32px, 32px;
    }

    @media (min-width: 1701px) {
        :global(article) {
            display:inherit;
        }
        .mobile-grid {
            display:none;
        }
    }

</style>