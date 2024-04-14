<script>
    import { Link } from "svelte-navigator";
    import { account } from "$lib/stores";
    import {ƒ} from '$lib/utils';

    function toggleSidebar() {
        const sideBar = ƒ('.ui.sidebar');
        const mainPage = ƒ('#mainPage');

        const isOpen = sideBar.getAttribute('class').includes('visible');

        if (isOpen) {
            mainPage.classList.add('closing')
            sideBar.classList.replace('visible', 'animating');
            setTimeout(() => {
                mainPage.classList.remove('closing', 'dimmed')
                sideBar.classList.remove('animating');
            }, 300);
        } else {
            sideBar.classList.add('pusher', 'visible');
            mainPage.classList.add('dimmed');
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