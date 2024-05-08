<script>
  import { Router, Route } from "svelte-navigator";

  import Header from "./lib/Header.svelte";

  import Mainmenu from "./routes/Mainmenu.svelte";
</script>

<Router primary={false}>
  <Header></Header>
  <main>
    <Route path="/" component={Mainmenu} />
  </main>
</Router>

<style>
  main {
    position: relative;
    background: #fff;
    box-shadow: 0 1px 2px 0 rgba(34, 36, 38, 0.15);
    padding: 1em 1em;
    border-radius: 0.28571429rem;
    border: 1px solid rgba(34, 36, 38, 0.15);
    min-height: 95dvh;
  }

  main:first-child {
    margin-top: 0;
  }

  main:last-child {
    margin-bottom: 0;
  }
</style>
