<script>
    import Header from '$components/Header.svelte';
    import Sidebar from '$components/Sidebar.svelte';
    import { Router, Route, navigate } from 'svelte-navigator';

    import { SidebarOpen, account } from '$lib/stores';
    import Login from '$routes/Login.svelte';
    import Mainmenu from '$routes/Mainmenu.svelte';
    import NotFound from '$routes/NotFound.svelte';
    import Account from '$routes/Account.svelte';
    import { getLoggedIn } from '$utils';
    import { onMount } from 'svelte';

    $: IsSidebarOpen = $SidebarOpen;

    function closeSidebar() {
        if (IsSidebarOpen) SidebarOpen.update((state) => !state);
    }

    onMount(async function () {
        if (!$account.id) {
            const successState = await getLoggedIn();
            if (!successState) navigate('/login');
        }
    });
</script>

<Router primary={false}>
    <Sidebar />
    <pusher class={IsSidebarOpen ? 'push dim' : ''} on:click={closeSidebar}>
        <Header />
        <main>
            <Route path="/" component={Mainmenu} />
            <Route path="/login/" component={Login} />
            <Route path="/account/" component={Account} />
            <Route path="*" component={NotFound} />
        </main>
    </pusher>
</Router>

<style>
    pusher {
        width: 100%;
        min-height: 100dvh;
        transition:
            margin-left 0.3s,
            opacity 0.3s;
        display: block;
        background-color: var(--main);
    }

    main {
        min-height: 95dvh;
        margin-top: 5dvh;
        padding: 1em;
    }
</style>
