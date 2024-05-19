<script>
    import ReportTable from '$components/ReportsTable/table.svelte';
    import { reports } from '$stores';
    import { getReports } from '$utils';
    import * as Card from '$components/ui/card';
    import { Link } from 'svelte-navigator';
    getReports();
</script>

<div class="hidden h-full flex-1 flex-col space-y-8 p-8 md:flex">
    <div class="flex items-center justify-between space-y-2">
        <div>
            <h2 class="text-2xl font-bold tracking-tight">Reports</h2>
        </div>
    </div>
    <ReportTable />
</div>

<div class="h-full flex-1 flex-col space-y-8 p-4 md:hidden">
    <div class="flex items-center justify-between space-y-2">
        <div>
            <h2 class="text-2xl font-bold tracking-tight">Reports</h2>
        </div>
    </div>
    {#each $reports as report}
        <Card.Root class="w-[100%]">
            <Card.Header>
                <Card.Title><Link to="/report/{report?.id}">{report?.title}</Link></Card.Title>
                <Card.Description>{report?.description}</Card.Description>
            </Card.Header>
        </Card.Root>
    {/each}
</div>
