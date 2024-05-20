<script>
    import Search from 'lucide-svelte/icons/search';
    import { Input } from '$components/ui/input/';
    import { Separator } from '$components/ui/separator';
    import * as Tabs from '$components/ui/tabs/';
    import { account, reports, tasks } from '$stores';
    import * as Card from '$components/ui/card';
    import { getMyReports, getTasksToExecute } from '$utils';
    import { onMount } from 'svelte';
    import { Button } from '$components/ui/button/';
    import { Link } from 'svelte-navigator';

    onMount(async function () {
        reports.set([]);
        tasks.set([]);
        getMyReports($account?.id);
        getTasksToExecute($account?.id);
    });

    let type = 'task';
    let activeTab = 'active';
</script>

<div class="mainGrid">
    <div>
        <Button on:click={() => (type = 'task')}>Tasks to do</Button>
        <Button on:click={() => (type = 'report')}>My Reports</Button>
    </div>
    <div>
        <Tabs.Root bind:value={activeTab}>
            <div class="flex items-center px-4 py-2">
                <h1 class="text-xl font-bold">Inbox</h1>
                <Tabs.List class="ml-auto">
                    <Tabs.Trigger value="all" class="text-zinc-600 dark:text-zinc-200">All</Tabs.Trigger>
                    <Tabs.Trigger value="active" class="text-zinc-600 dark:text-zinc-200">Active</Tabs.Trigger>
                </Tabs.List>
            </div>
            <Separator />
            <div class="bg-background/95 p-4 backdrop-blur supports-[backdrop-filter]:bg-background/60">
                <form>
                    <div class="relative">
                        <Search class="absolute left-2 top-3 h-4 w-4 text-muted-foreground" />
                        <Input placeholder="Search" class="pl-8" />
                    </div>
                </form>
            </div>
            <Tabs.Content value="all" class="m-0">
                {#if type === 'task'}
                    {#each $tasks as task}
                        <p>{task?.id}</p>
                    {/each}
                {:else}
                    {#each $reports as report}
                        <p>{report?.id}</p>{/each}
                {/if}
            </Tabs.Content>
            <Tabs.Content value="active" class="m-0">
                {#if type === 'task'}
                    {#each $tasks?.filter((t) => !t?.archived) as task}
                        <Card.Root>
                            <Card.Header>
                                <Card.Title
                                    ><b class="ml-auto text-xs text-muted-foreground">Title </b><Link
                                        to="/task/{task?.id}">{task?.title}</Link
                                    ></Card.Title
                                >
                            </Card.Header>
                            <Card.Content>
                                <p>
                                    <b class="ml-auto text-xs text-muted-foreground">Type</b>
                                    <Link to="/task/panel/?type={task?.task_type?.id}">{task?.task_type?.name}</Link>
                                </p>
                                <p>
                                    <b class="ml-auto text-xs text-muted-foreground">Status</b>
                                    <Link to="/task/panel/?status={task?.status?.id}">{task?.status?.name}</Link>
                                </p>
                                {#if task?.machine?.id}
                                    <p>
                                        <b class="ml-auto text-xs text-muted-foreground">Machine</b>

                                        <Link to="/machine/{task?.machine?.id}">{task?.machine?.name}</Link>
                                    </p>
                                {/if}
                                <p>
                                    <b class="ml-auto text-xs text-muted-foreground">Creator</b>
                                    <Link to="/user/{task?.creator?.id}">
                                        {task?.creator?.first_name}
                                        {task?.creator?.last_name}
                                    </Link>
                                </p>
                                <Separator class="my-1" />
                                <h2 class="ml-auto text-xs text-muted-foreground">Description</h2>
                                <span>
                                    {task?.description}
                                </span>
                            </Card.Content>
                        </Card.Root>
                    {/each}
                {:else}
                    {#each $reports?.filter((r) => !r?.archived) as report}
                        <p>{report?.id}</p>{/each}
                {/if}
            </Tabs.Content>
        </Tabs.Root>
    </div>
    <div>Panel 3</div>
</div>

<style>
    .mainGrid {
        padding-top: 2%;
        display: grid;
        grid-template-columns: 1fr 2fr 4fr;
        height: 90dvh;
        width: 100%;
    }

    .mainGrid > div {
        height: 100%;
        border: 1px solid red;
    }
</style>
