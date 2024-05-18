<script>
    import ReportTable from '$components/ReportsTable/table.svelte';
    import { getReports } from '$utils';
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
