<script>
  import { Router, Route, Link} from 'svelte-navigator';

  import Machines from './routes/machines.svelte'
  import Tasks from './routes/tasks.svelte'
  import Users from './routes/users.svelte'
</script>

<Router>
  <header>
    <nav>
      <Link to="/">Home</Link>
      <Link to="/machines">Machines</Link>
      <Link to="/tasks">Tasks</Link>
      <Link to="/users">Users</Link>
    </nav>
  </header>

  <main>
    <Route path="/machines" component={Machines}/>
    <Route path="/tasks" component={Tasks}/>
    <Route path="/users" component={Users}/>
  </main>

  <footer>

  </footer>

</Router>