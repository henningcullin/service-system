<script>
    import { Link } from "svelte-navigator";
    import { account } from "$lib/stores";
</script>

<header class="ui huge fixed menu">
    <div class="item">
        <i class="bars icon"></i>
    </div>
    {#if $account.id}
            <Link to='/' class='header item'>Home</Link>
            <Link to='/machines' class='item'>Machines</Link>
            <Link to='/users' class='item'>Users</Link>
            <Link to='/tasks' class='item'>Tasks</Link>
            <Link to='/account' class='right item'>{$account.first_name}</Link>
        {:else}
            <Link to='/login' class='item'>Login</Link>
        {/if}
</header>