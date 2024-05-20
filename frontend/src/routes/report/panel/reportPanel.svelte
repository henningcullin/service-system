<script lang="ts">
    import * as Tabs from '$components/ui/tabs/index.js';
    import TypeStatusCard from '$components/TypeStatusCard.svelte';
    import { reportTypes, reportType, reportStatuses, reportStatus } from '$stores';
    import { getReportStatuses, getReportTypes } from '$utils';

    const url = new URL(location.href);
    const type = url.searchParams?.get('type');
    const status = url.searchParams?.get('status');

    getReportTypes();
    getReportStatuses();

    document.title = 'Report Panel';
</script>

<div class="w-full min-h-[15vh] flex items-center justify-center">
    <h2 class="text-2xl font-bold tracking-tight pb-2">Report Panel</h2>
</div>

<div class="w-full min-h-[65vh] flex items-center justify-center">
    <Tabs.Root value={status ? 'status' : 'type'} class="w-[400px]">
        <Tabs.List class="grid w-full grid-cols-2">
            <Tabs.Trigger value="type">Type</Tabs.Trigger>
            <Tabs.Trigger value="status">Status</Tabs.Trigger>
        </Tabs.List>
        <Tabs.Content value="type">
            <TypeStatusCard
                cardProps={{
                    title: 'Type',
                    desc: 'Manage your report types',
                }}
                formProps={{
                    selectPlaceholder: 'Select a type',
                    namePlaceholder: 'Name of type',
                }}
                formStore={reportType}
                sourceStore={reportTypes}
                initialSelected={type}
                apiEndpoint="report_type"
            ></TypeStatusCard>
        </Tabs.Content>
        <Tabs.Content value="status">
            <TypeStatusCard
                cardProps={{
                    title: 'Status',
                    desc: 'Manage your report statuses',
                }}
                formProps={{
                    selectPlaceholder: 'Select a status',
                    namePlaceholder: 'Name of status',
                }}
                formStore={reportStatus}
                sourceStore={reportStatuses}
                initialSelected={status}
                apiEndpoint="report_status"
            ></TypeStatusCard>
        </Tabs.Content>
    </Tabs.Root>
</div>
