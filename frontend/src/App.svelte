<script>
  import { Router, Route } from "svelte-navigator";

  import Mainmenu from "./routes/Mainmenu.svelte";
</script>

<Router primary={false}>
  <main>
    <Route path="/" component={Mainmenu} />
  </main>
</Router>

<style>
  main {
  }
</style>
