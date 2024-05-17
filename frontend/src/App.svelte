<script>
    import './app.css';
    import './custom.css';
    import { onMount } from 'svelte';
    import { Router, Route, navigate } from 'svelte-navigator';

    import { SidebarOpen, account } from '$lib/stores';
    import { getLoggedIn } from '$utils';
    import Header from '$components/Header.svelte';
    import Sidebar from '$components/Sidebar.svelte';
    import { Toaster } from '$lib/components/ui/sonner';

    import Login from '$routes/Login.svelte';
    import Mainmenu from '$routes/Mainmenu.svelte';
    import NotFound from '$routes/NotFound.svelte';

    import Machines from '$routes/machines/machines.svelte';
    import Machine from '$routes/machine/machine.svelte';
    import MachinePanel from '$routes/machine/panel/machinePanel.svelte';

    import TaskPanel from '$routes/task/panel/taskPanel.svelte';

    import { ModeWatcher, localStorageKey, setMode, systemPrefersMode, userPrefersMode } from 'mode-watcher';

    $: IsSidebarOpen = $SidebarOpen;

    function closeSidebar() {
        if (IsSidebarOpen) SidebarOpen.update((state) => !state);
    }

    let preference = localStorage.getItem(localStorageKey);
    preference ?? ($userPrefersMode === 'system' ? $systemPrefersMode : $userPrefersMode);
    setMode(preference);

    onMount(async function () {
        if (!$account.id) {
            const successState = await getLoggedIn();
            if (!successState) navigate('/login');
        }
    });
</script>

<ModeWatcher />
<Toaster />

<Router primary={false}>
    <Sidebar></Sidebar>
    <pushable class={IsSidebarOpen ? 'push dim' : ''} on:click={closeSidebar}>
        <Header></Header>
        <main>
            <Route path="/" component={Mainmenu}></Route>
            <Route path="/login/" component={Login}></Route>

            <Route path="/machines/*" component={Machines}></Route>
            <Route path="/machine/panel/*" component={MachinePanel}></Route>
            <Route path="/machine/*" component={Machine}></Route>

            <Route path="/task/panel/*" component={TaskPanel} />

            <Route path="*" component={NotFound}></Route>
        </main>
    </pushable>
</Router>

<style>
    pushable {
        width: 100%;
        min-height: 95dvh;
        transition:
            margin-left 0.3s,
            opacity 0.3s;
        display: block;
        background-color: var(--main);
    }

    main {
        min-height: 95dvh;
        margin-top: 3dvh;
        padding: 1em;
    }
</style>
