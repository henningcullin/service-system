<script>
    import { Link } from "svelte-navigator";
    import { account } from "$lib/stores";

    function toggleSidebar() {
        const sideBar = document.querySelector('.ui.sidebar');
        
        const isOpen = sideBar.getAttribute('class').includes('visible');

        if (isOpen) {
            sideBar.classList.replace('visible', 'animating');
            setTimeout(() => {
                sideBar.classList.remove('animating');
            }, 300);
        } else {
            sideBar.classList.add('pusher', 'visible');
        }
    }

</script>

<header class="ui huge fixed menu">
    <a class="item" on:click={toggleSidebar}>
        <i class="bars icon"></i>
    </a>
    {#if $account.id}
        <Link to='/account' class='right item'>{$account.first_name}</Link>
    {:else}
        <Link to='/login' class='item'>Login</Link>
    {/if}
</header>