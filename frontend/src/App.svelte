<script>
  import { showSidebar } from "$lib/stores";

  import { Router, Route } from "svelte-navigator";

  import Header from "$lib/components/Header.svelte";
  import Sidebar from "$lib/components/Sidebar.svelte";

  import Mainmenu from "$routes/Mainmenu.svelte";
  import Login from "$routes/Login.svelte";

  $: isSidebarOpen = $showSidebar;
</script>

<Router primary={false}>
  <Header></Header>
  <Sidebar></Sidebar>
  <main class={isSidebarOpen ? "push" : ""}>
    <Route path="/" component={Mainmenu} />
    <Route path="/login" component={Login} />
  </main>
</Router>

<style>
  main {
    position: relative;
    background: #fff;
    padding: 1em 1em;
    min-height: 95dvh;
  }

  main:first-child {
    margin-top: 0;
  }

  main:last-child {
    margin-bottom: 0;
  }
</style>
