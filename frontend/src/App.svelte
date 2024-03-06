<script>
  import { Router, Route, Link } from "svelte-navigator";

  import Home from "./routes/home.svelte";
  import NotFound from "./routes/notFound.svelte";

  import Machine from "./routes/machines/machine.svelte";
  import Machines from "./routes/machines/machines.svelte";
  import CreateMachine from "./routes/machines/createMachine.svelte";

  import Tasks from "./routes/tasks.svelte";
  import Users from "./routes/users.svelte";
  import Login from "./routes/login.svelte"
</script>

<Router>
  <header>
    <nav>
      <Link to="/" class="nav-link">Home</Link>
      <Link to="/machines" class="nav-link">Machines</Link>
      <Link to="/tasks" class="nav-link">Tasks</Link>
      <Link to="/users" class="nav-link">Users</Link>
      <Link to="/login" class="nav-link">Login</Link>
    </nav>
  </header>

  <main>
    <Route path="/" component={Home} />

    <Route path="/machines/new" component={CreateMachine}/>
    <Route path="/machines/" component={Machines} />
    <Route path="/machine/:id" component={Machine}/>
    
    <Route path="/tasks/*" component={Tasks} />
    <Route path="/users/*" component={Users} />
    <Route path="/login/*" component={Login} />
    <Route path="*" component={NotFound} />
  </main>

  <footer></footer>
</Router>

<style>

  /* #region HEADER */

  header {
    width: 100%;
    height: 7.5dvh;
    top: 0px;
    text-align: center;
    background-color: #202020;
  }

  nav {
    width:100%;
    height:100%;
    display:flex;
    align-items: center;
    justify-content: center;
    text-align: center;
    gap:1px;
  }

  :global(.nav-link) {
    flex:1;
    color: whitesmoke;
    text-align: center;
    height:100%;
    display:grid;
    place-items: center;
  }

  :global(.nav-link):hover {
    background-color: #282828;
  }

  /* #endregion */

  /* #region MAIN */

  main {
    min-height:87.5dvh;
    display:grid;
    place-items: center;
  }

  /* #endregion */

  /* #region FOOTER */

  footer {
    height: 5dvh;
    background-color: #202020;
    bottom:0px;
  }

  /* #endregion */

</style>