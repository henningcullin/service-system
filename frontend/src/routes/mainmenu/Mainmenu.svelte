<script>
    import Search from 'lucide-svelte/icons/search';
    import { Input } from '$components/ui/input/';
    import { Separator } from '$components/ui/separator';
    import * as Tabs from '$components/ui/tabs/';
    import { account, reports, tasks } from '$stores';
    import { getMyReports, getTasksToExecute } from '$utils';
    import { onMount } from 'svelte';
    import TaskCard from '$components/MainMenu/TaskCard.svelte';
    import ReportCard from '$components/MainMenu/ReportCard.svelte';
    import { Button } from '$components/ui/button/';

    function eventToObj(ev) {
        try {
            const data = JSON.parse(ev.data);
            return data;
        } catch (error) {
            return false;
        }
    }

    onMount(function () {
        reports.set([]);
        tasks.set([]);
        getMyReports($account?.id);
        getTasksToExecute($account?.id);

        const taskChannel = new EventSource('/api/auth/channel/tasks');
        const reportChannel = new EventSource('/api/auth/channel/reports');

        taskChannel.onmessage = (e) => {
            const message = eventToObj(e);
            if (!message) return;
            console.log('task message', message);
        };

        reportChannel.onmessage = (e) => {
            const message = eventToObj(e);
            if (!message) return;
            console.log('report message', message);
        };

        return () => {
            taskChannel.close();
            reportChannel.close();
        };
    });

    let type = 'task';
    let activeTab = 'active';
</script>

<div class="mainGrid">
    <div>
        <Tabs.Root bind:value={activeTab}>
            <div class="flex items-center px-4 py-2">
                <h1 class="text-xl font-bold">Inbox</h1>
                <Button on:click={() => (type = 'task')}>Tasks to do</Button>
                <Button on:click={() => (type = 'report')}>My Reports</Button>
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
            <div class="p-1">
                <Tabs.Content value="all" class="p-2">
                    {#if type === 'task'}
                        {#each $tasks as task}
                            <TaskCard {task} />
                        {/each}
                    {:else}
                        {#each $reports as report}
                            <ReportCard {report} />
                        {/each}
                    {/if}
                </Tabs.Content>
                <Tabs.Content value="active" class="p-2">
                    {#if type === 'task'}
                        {#each $tasks?.filter((t) => !t?.archived) as task}
                            <TaskCard {task} />
                        {/each}
                    {:else}
                        {#each $reports?.filter((r) => !r?.archived) as report}
                            <ReportCard {report} />{/each}
                    {/if}
                </Tabs.Content>
            </div>
        </Tabs.Root>
    </div>
</div>

<style>
    .mainGrid {
        padding-top: 2%;
        display: grid;
        grid-template-columns: 1fr 2fr 4fr;
        height: 90dvh;
        width: 90%;
    }

    .mainGrid > div {
        height: 100%;
        width: 100%;
        border: 1px solid red;
    }
</style>
