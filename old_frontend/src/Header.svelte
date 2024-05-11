<script>
    import { Link } from 'svelte-navigator';
    import { account } from '$lib/stores';
    import { Header, Item, Icon, behavior } from 'svelte-fomantic-ui';
</script>

<Header ui fixed menu>
    <Item
        on:click={() => {
            behavior('sidebar', 'toggle');
        }}
    >
        <Icon ui icon bars></Icon>
    </Item>
    {#if $account.id}
        <Link class="ui right item" to="/account">{$account.first_name}</Link>
    {:else}
        <Link class="ui item" to="/login">Login</Link>
    {/if}
</Header>
