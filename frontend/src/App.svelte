<script>
// @ts-nocheck

  import { Router, Route} from 'svelte-navigator';

  import Header from './Header.svelte';
  import Footer from './Footer.svelte';
  import Sidebar from './Sidebar.svelte';

  import { account } from '$lib/stores';
  import { getLoggedIn } from '$lib/utils';

  import Home from './routes/home.svelte';
  import NotFound from './routes/notFound.svelte';

  import Machine from './routes/machines/machine.svelte';
  import Machines from './routes/machines/machines.svelte';

  import User from './routes/users/user.svelte';
  import Users from './routes/users/users.svelte';
  
  import Task from './routes/tasks/task.svelte';
  import Tasks from './routes/tasks/tasks.svelte';

  import Login from './routes/login.svelte'
  import Account from './routes/account.svelte';

  if (Object.keys($account).length == 0) {
    getLoggedIn();
  }
  
</script>

<Router primary={false}>

  <Header/>
  <Sidebar/>

  <main class="ui segment">
    <Route path='/' component={Home} />

    <Route path='/machines/' component={Machines} />
    <Route path='/machine/' component={Machine}/>
    
    <Route path='/users/' component={Users} />
    <Route path='/user/' component={User} />

    <Route path='/tasks/' component={Tasks} />
    <Route path='/task/' component={Task} />
    

    <Route path='/login/*' component={Login} />
    <Route path='/account/*' component={Account} />
    <Route path='*' component={NotFound} />
  </main>

  <Footer/>
  
</Router>