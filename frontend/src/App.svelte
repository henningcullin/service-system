<script>
// @ts-nocheck

  import { Router, Route, Link} from 'svelte-navigator';

  import { account } from './lib/stores';
  import { getUser } from './lib/helpers';

  import Home from './routes/home.svelte';
  import NotFound from './routes/notFound.svelte';

  import Machine from './routes/machines/machine.svelte';
  import Machines from './routes/machines/machines.svelte';

  import User from './routes/users/user.svelte';
  import Users from './routes/users/users.svelte';
  

  import Tasks from './routes/tasks.svelte';

  import Login from './routes/login.svelte'
  import Account from './routes/account.svelte';

  if (Object.keys($account).length == 0) {
    getUser();
  }
  
</script>

<Router primary={false}>
  <header>
    <nav>
      {#if $account.id}
        <Link to='/' class='nav-link'>Home</Link>
        <Link to='/machines' class='nav-link'>Machines</Link>
        <Link to='/tasks' class='nav-link'>Tasks</Link>
        <Link to='/users' class='nav-link'>Users</Link>
        <Link to='/account' class='nav-link'>{$account.first_name}</Link>
      {:else}
        <Link to='/login' class='nav-link'>Login</Link>
      {/if}
    </nav>
  </header>

  <main>
    <Route path='/' component={Home} />

    <Route path='/machines/' component={Machines} />
    <Route path='/machine/' component={Machine}/>
    
    <Route path='/users/' component={Users} />
    <Route path='/user/' component={User} />

    <Route path='/tasks/*' component={Tasks} />
    

    <Route path='/login/*' component={Login} />
    <Route path='/account/*' component={Account} />
    <Route path='*' component={NotFound} />
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


  @media (min-width: 1200px) {
    header nav {
      font-size: 1.4em;
    }
  }
</style>