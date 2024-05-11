<script>
    import Header from '$components/Header.svelte';
    import Sidebar from '$components/Sidebar.svelte';
    import { Router, Route } from 'svelte-navigator';

    import { SidebarOpen } from '$lib/stores';
    import Login from '$routes/Login.svelte';

    $: IsSidebarOpen = $SidebarOpen;

    function closeSidebar() {
        if (IsSidebarOpen) SidebarOpen.update((state) => !state);
    }
</script>

<Sidebar />
<pusher class={IsSidebarOpen ? 'push dim' : ''} on:click={closeSidebar}>
    <Header />
    <main>
        <Router primary={false}>
            <Route path="/login/" component={Login} />
        </Router>
    </main>
</pusher>

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
        min-height: 100dvh;
        margin-top: 5dvh;
        padding: 1em;
    }
</style>
