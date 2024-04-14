<script>

    import {Link} from 'svelte-navigator';
    import {account} from '$lib/stores';
    import {ƒ} from '$lib/utils';

    function goToPage() {
        const sideBar = ƒ('.ui.sidebar');
        const mainPage = ƒ('#mainPage');
        mainPage.classList.add('closing')
        sideBar.classList.replace('visible', 'animating');
        setTimeout(() => {
            mainPage.classList.remove('closing', 'dimmed')
            sideBar.classList.remove('animating');
        }, 500);
    }

</script>

<div class="ui sidebar vertical menu left" on:click={goToPage}>
    <h2 class="ui massive centered header">Service System</h2>
    {#if $account.id}
        <Link to='/' class='header item'>Home</Link>
        <Link to='/machines' class='item'>Machines</Link>
        <Link to='/users' class='item'>Users</Link>
        <Link to='/tasks' class='item'>Tasks</Link>
    {/if}
</div>

<style>

    .ui.sidebar {
        max-height: 100dvh;
        overflow: hidden;
    }
    
</style>