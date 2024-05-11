<script>
    import { Link } from 'svelte-navigator';
    import { Sidebar, Header, Accordion, Title, Content, Icon, Menu, Divider } from 'svelte-fomantic-ui';
</script>

<Sidebar ui vertical menu left id="sidebar">
    <Header ui massive centered>Service System</Header>
    <Link to="" class="item">Home</Link>
    <Accordion ui styled>
        <!-- TASKS-->
        <Title>
            <Icon clipboard></Icon>
            Tasks
        </Title>
        <Content>
            <Link to="" class="item">
                <Icon plus></Icon>
                Create Task
            </Link>
            <Link to="" class="item">
                <Icon list></Icon>
                View Tasks
            </Link>
        </Content>
        <Divider></Divider>
        <!-- REPORTS-->
        <Title>
            <Icon exclamation></Icon>
            Reports
        </Title>
        <Content>
            <Link to="" class="item">
                <Icon plus></Icon>
                Create Report
            </Link>
            <Link to="" class="item">
                <Icon list></Icon>
                View Reports
            </Link>
        </Content>
        <!-- MACHINES -->
        <Title>
            <Icon microchip></Icon>
            Machines
        </Title>
        <Content>
            <Link to="" class="item">
                <Icon plus></Icon>
                Create Machine
            </Link>
            <Link to="" class="item">
                <Icon list></Icon>
                View Machines
            </Link>
        </Content>
        <!-- USERS -->
        <Title>
            <Icon user></Icon>
            User
        </Title>
        <Content>
            <Link to="" class="item">
                <Icon plus></Icon>
                Create User
            </Link>
        </Content>
    </Accordion>
</Sidebar>

<style>
    :global(.ui.styled.accordion > .content) {
        padding: 0 !important;
    }
</style>
