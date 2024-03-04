<script>
  import { Router, Route, Link } from "svelte-navigator";

  import Home from "./routes/home.svelte";
  import NotFound from "./routes/notFound.svelte";
  import Machines from "./routes/machines.svelte";
  import Tasks from "./routes/tasks.svelte";
  import Users from "./routes/users.svelte";
</script>

<Router>
  <header>
    <nav>
      <Link to="/" class="nav-link">Home</Link>
      <Link to="/machines" class="nav-link">Machines</Link>
      <Link to="/tasks" class="nav-link">Tasks</Link>
      <Link to="/users" class="nav-link">Users</Link>
    </nav>
  </header>

  <main>
    <Route path="/" component={Home} />
    <Route path="/machines/*" component={Machines} />
    <Route path="/tasks/*" component={Tasks} />
    <Route path="/users/*" component={Users} />
    <Route path="*" component={NotFound} />
  </main>

  <footer></footer>
</Router>

<style>

  header {
    width: 100%;
    height: 7.5vh;
    top: 0px;
    text-align: center;
  }

  nav {
    width:100%;
    height:100%;
    display:flex;
    align-items: center;
    gap:1px;
  }

  :global(.nav-link) {
    display:block;
    position: relative;
    background-color: green;
    color: whitesmoke;
    width:100%;
    height:100%;
    padding:3.6%;
  }

</style>