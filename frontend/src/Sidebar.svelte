<script>
    import {Link} from 'svelte-navigator';
    import {account} from '$lib/stores';
    import { Sidebar, Header, Accordion, Title, Content, Icon, Menu } from 'svelte-fomantic-ui';

</script>


<Sidebar ui vertical menu left id="sidebar">
    <Header ui massive centered>
        Service System
    </Header>
    {#if $account.id}
        <Link to='/' class='header item'>Home</Link>
        <Link to='/machines' class='item'>Machines</Link>
        <Link to='/users' class='item'>Users</Link>
        <Link to='/tasks' class='item'>Tasks</Link>
    {/if}
    <Accordion ui basic styled>
        <Title>
            <Icon microchip></Icon>
            Machines
        </Title>
        <Content>
            <Link to="/machine?new=true" class="item">Create Machine</Link>
        </Content>
        <Title>
            <Icon clipboard></Icon>
            Work Orders
        </Title>
        <Content>
            
        </Content>

        <Title>
            <Icon user></Icon>
            User
        </Title>
        <Content>

        </Content>
    </Accordion>
</Sidebar>