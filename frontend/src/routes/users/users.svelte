<script>
    // @ts-nocheck
    
        import { DataHandler, Datatable, Th, ThFilter} from '@vincjo/datatables';
        import { Link, navigate } from 'svelte-navigator';
        import { account, users } from '../../lib/stores.js';
        import { getUsers } from '../../lib/utils.js';
    
        let lastFetch = false;
    
        if (!$users.length && !lastFetch) {
            lastFetch = Date.now();
            getUsers();
        }
    
        let currentPage = 1;
        const cardsPerPage = 4;
        $: pageCount = Math.ceil( ($users.length+1) / cardsPerPage);
    
        const handler = new DataHandler($users, {rowsPerPage: 10});
        const rows = handler.getRows();
        
        $: handler.setRows($users)
    
        async function editUser() {
    
            if ($account.role === 'Worker') return;
    
            const id = this ? this.id : null;
    
            if (!id) return;
    
            navigate(`/user?id=${id}&edit=true`);
        }
    
    </script>
    
    <div class='segment'>
        <h2> Users </h2>
        
        {#if $account.role !== 'Worker'}
            <div class='menu'>
                <Link to='/user?new=true'>New</Link>
            </div>
        {/if}
    
        <div class='mobile-grid'>
            {#each $users.slice((currentPage - 1) * cardsPerPage, currentPage * cardsPerPage) as u}
                <div class='mobile-card'> 
                    <Link to='/user?id={u.id}' class='itemLink' >{u.first_name}</Link>
                    <p>{u.last_name}</p>
                    <p>{u.email}</p>
                    <p>{u.phone}</p>
                    <p>{u.role}</p>
                    <p class="{u.active}" >{u.active}</p>
                    <i>{u.last_login.toLocaleString('en-GB')}</i><br>
                    {#if $account.role !== 'Worker'}
                        <button on:click={editUser} id='{u.id}' class='cardEditButton'/>
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
                        <Th {handler} orderBy='first_name'>First Name</Th>
                        <Th {handler} orderBy='last_name'>Last Name</Th>
                        <Th {handler} orderBy='email'>Email</Th>
                        <Th {handler} orderBy='phone'>Phone</Th>
                        <Th {handler} orderBy='role'>Role</Th>
                        <Th {handler} orderBy='active'>Active</Th>
                        <Th {handler} orderBy='last_login'>Last Login</Th>
                        {#if $account.role !== 'Worker'}
                            <Th {handler} orderBy='none'>Edit</Th>
                        {/if}
                    </tr>
                    <tr>
                        <ThFilter {handler} filterBy='id'/>
                        <ThFilter {handler} filterBy='first_name'/>
                        <ThFilter {handler} filterBy='last_name'/>
                        <ThFilter {handler} filterBy='email'/>
                        <ThFilter {handler} filterBy='phone'/>
                        <ThFilter {handler} filterBy='role'/>
                        <ThFilter {handler} filterBy='active'/>
                        <ThFilter {handler} filterBy='last_login'/>
                        {#if $account.role !== 'Worker'}
                            <ThFilter {handler} filterBy='none'/>
                        {/if}
                    </tr>
                </thead>
                <tbody>
                    {#each $rows as row}
                        <tr>
                            <td><Link to='/user?id={row.id}' class='itemLink'>{row.id}</Link></td>
                            <td>{row.first_name}</td>
                            <td>{row.last_name}</td>
                            <td>{row.email}</td>
                            <td>{row.phone ? row.phone : ''}</td>
                            <td>{row.role}</td>
                            <td class='{row.active}'>{row.active ? 'Active' : 'Deactivated'}</td>
                            <td>{row.last_login.toLocaleString('en-GB')}</td>
                            {#if $account.role !== 'Worker'}
                                <td class='buttonCell'><button class='tableEditButton' id='{row.id}' on:click={editUser}></button></td>
                            {/if}
                        </tr>
                    {/each}
                </tbody>
            </table>
        </Datatable>
    
    </div>
    
    
    <style>
    
        .true {
            color: #40b129;
        }
    
        .false {
            color: #d84444;
        }
    
        /* mobile */
    
        .mobile-grid {
            margin-top: 30px;
            display:grid;
            width:100%;
            grid-template-columns: 1fr;
            grid-template-rows: 1fr 1fr 1fr;
            gap:6px;
            padding:0.3%;
            min-height: 72dvh;
        }
    
        .mobile-card {
            padding: 0.2%;
            width:80%;
            margin-left: 10%;
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
        
        .tableEditButton {
            height:48px;
            width:48px;
            background-color: transparent;
            background-repeat: no-repeat;
            background-position: center;
            background-size: 32px, 32px;
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