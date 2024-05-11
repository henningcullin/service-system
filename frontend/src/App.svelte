<script>
    import Header from '$components/Header.svelte';
    import Sidebar from '$components/Sidebar.svelte';
    import { Router, Route } from 'svelte-navigator';
</script>

<pusher>
    <Header />
    <main>
        <Router>
            <Route></Route>
        </Router>
    </main>
</pusher>

<style>
</style>
