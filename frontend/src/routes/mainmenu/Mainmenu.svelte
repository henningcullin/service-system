<script>
    import Search from 'lucide-svelte/icons/search';
    import { Input } from '$components/ui/input/';
    import { Separator } from '$components/ui/select/';
    import * as Tabs from '$components/ui/tabs/';
    import { account, reports, tasks } from '$stores';
    import * as Card from '$components/ui/card';
    import { getMyReports, getTasksToExecute } from '$utils';
    import { onMount } from 'svelte';

    const TASK_VIEWING = 0;
    const REPORT_VIEWING = 1;

    onMount(async function () {
        reports.set([]);
        tasks.set([]);
        getMyReports($account?.id);
        getTasksToExecute($account?.id);
    });

    let sourceStore = [];
    let allResults = [];
    let activeResults = [];

    let type = TASK_VIEWING;
    let activeTab = 'active';
</script>

<div class="mainGrid">
    <div>
        <Tabs.Root bind:value={type}>
            <Tabs.Trigger value={TASK_VIEWING}>Tasks</Tabs.Trigger>
            <Tabs.Trigger value={REPORT_VIEWING}>Reports</Tabs.Trigger>
        </Tabs.Root>
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
            <Tabs.Content value="all" class="m-0">All reports / tasks</Tabs.Content>
            <Tabs.Content value="active" class="m-0">Non archived reports / tasks</Tabs.Content>
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
