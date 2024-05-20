<script>
    import ReportTable from '$components/ReportsTable/table.svelte';
    import { reports } from '$stores';
    import { getReports, evToObj, getOne } from '$utils';
    import * as Card from '$components/ui/card';
    import { Link } from 'svelte-navigator';
    import { onMount } from 'svelte';
    getReports();

    document.title = 'Reports';

    onMount(() => {
        const reportChannel = new EventSource('/api/auth/channel/reports');

        reportChannel.onmessage = async (e) => {
            const message = evToObj(e);
            if (!message) return;
            const { id, kind } = message;
            switch (kind) {
                case 'INSERT':
                    const insertData = await getOne(`/api/auth/report?report_id=${id}`);
                    if (!insertData) return;
                    reports.update((prev) => {
                        prev.unshift(insertData);
                        return prev;
                    });
                    break;
                case 'UPDATE':
                    const updateData = await getOne(`/api/auth/report?report_id=${id}`);
                    if (!updateData) return;
                    const index = $reports.findIndex((r) => r.id === id);
                    reports.update((prev) => {
                        prev[index] = updateData;
                        return prev;
                    });
                    break;
                case 'DELETE':
                    reports.update((prev) => prev.filter((r) => r.id !== id));
                    break;
            }
        };

        window.onbeforeunload = () => {
            reportChannel.close();
        };

        return () => {
            reportChannel.close();
        };
    });
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
