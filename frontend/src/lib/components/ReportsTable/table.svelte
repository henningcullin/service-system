<script>
    import { Render, Subscribe, createRender, createTable } from 'svelte-headless-table';
    import { reports } from '$stores';
    import {
        addColumnFilters,
        addHiddenColumns,
        addPagination,
        addSelectedRows,
        addSortBy,
        addTableFilter,
    } from 'svelte-headless-table/plugins';
    import {
        DTPagination,
        DTRowActions,
        DTReportTypeCell,
        DTReportStatusCell,
        DTFieldCell,
        DTToolbar,
        DTIdCell,
        DTDateCell,
        DTMachineCell,
        DTCreatorCell,
    } from './index.js';

    import * as Table from '$lib/components/ui/table/index.js';

    const table = createTable(reports, {
        select: addSelectedRows(),
        sort: addSortBy({
            toggleOrder: ['asc', 'desc'],
        }),
        page: addPagination(),
        filter: addTableFilter({
            fn: ({ filterValue, value }) => {
                return value.toLowerCase().includes(filterValue.toLowerCase());
            },
        }),
        colFilter: addColumnFilters(),
        hide: addHiddenColumns(),
    });

    const columns = table.createColumns([
        table.column({
            accessor: 'id',
            id: 'id',
            header: 'Report',
            cell: ({ value, row }) => {
                if (row.isData()) {
                    return createRender(DTIdCell, {
                        value,
                    });
                }
                return value;
            },
        }),
        table.column({
            accessor: 'title',
            id: 'title',
            header: 'Title',
            cell: ({ value, row }) => {
                if (row.isData()) {
                    return createRender(DTFieldCell, {
                        value,
                    });
                }
                return value;
            },
        }),
        table.column({
            accessor: 'description',
            id: 'description',
            header: 'Description',
            cell: ({ value, row }) => {
                if (row.isData()) {
                    return createRender(DTFieldCell, {
                        value,
                    });
                }
                return value;
            },
        }),
        table.column({
            accessor: 'report_type',
            id: 'report_type',
            header: 'Type',
            cell: ({ value }) => {
                return createRender(DTReportTypeCell, {
                    value,
                });
            },
        }),
        table.column({
            accessor: 'status',
            id: 'status',
            header: 'Status',
            cell: ({ value }) => {
                return createRender(DTReportStatusCell, {
                    value,
                });
            },
        }),
        table.column({
            accessor: 'machine',
            id: 'machine',
            header: 'Machine',
            cell: ({ value }) => {
                return createRender(DTMachineCell, {
                    value,
                });
            },
        }),
        table.column({
            accessor: 'creator',
            id: 'creator',
            header: 'Creator',
            cell: ({ value }) => {
                return createRender(DTCreatorCell, {
                    value,
                });
            },
        }),
        table.column({
            accessor: 'created',
            id: 'created',
            header: 'Created at',
            cell: ({ value, row }) => {
                if (row.isData()) {
                    return createRender(DTDateCell, {
                        value,
                    });
                }
                return value;
            },
        }),
        table.column({
            accessor: 'edited',
            id: 'edited',
            header: 'Edited at',
            cell: ({ value, row }) => {
                if (row.isData()) {
                    return createRender(DTDateCell, {
                        value,
                    });
                }
                return value;
            },
        }),
        table.display({
            id: 'actions',
            header: '',
            cell: ({ row }) => {
                if (row.isData() && row.original) {
                    return createRender(DTRowActions, {
                        row: row.original,
                    });
                }
                return '';
            },
        }),
    ]);

    const tableModel = table.createViewModel(columns);

    const { headerRows, pageRows, tableAttrs, tableBodyAttrs } = tableModel;
</script>

<div class="space-y-2">
    <DTToolbar {tableModel} {reports} />
    <div class="rounded-md border">
        <Table.Root {...$tableAttrs}>
            <Table.Header>
                {#each $headerRows as headerRow}
                    <Subscribe rowAttrs={headerRow.attrs()}>
                        <Table.Row>
                            {#each headerRow.cells as cell (cell.id)}
                                <Subscribe attrs={cell.attrs()} let:attrs props={cell.props()} let:props>
                                    <Table.Head {...attrs}>
                                        <Render of={cell.render()} />
                                    </Table.Head>
                                </Subscribe>
                            {/each}
                        </Table.Row>
                    </Subscribe>
                {/each}
            </Table.Header>
            <Table.Body {...$tableBodyAttrs}>
                {#if $pageRows.length}
                    {#each $pageRows as row (row.id)}
                        <Subscribe rowAttrs={row.attrs()} let:rowAttrs>
                            <Table.Row {...rowAttrs}>
                                {#each row.cells as cell (cell.id)}
                                    <Subscribe attrs={cell.attrs()} let:attrs>
                                        <Table.Cell {...attrs}>
                                            {#if cell.id === 'id'}
                                                <div class="w-[80px]">
                                                    <Render of={cell.render()} />
                                                </div>
                                            {:else}
                                                <Render of={cell.render()} />
                                            {/if}
                                        </Table.Cell>
                                    </Subscribe>
                                {/each}
                            </Table.Row>
                        </Subscribe>
                    {/each}
                {:else}
                    <Table.Row>
                        <Table.Cell colspan={columns.length} class="h-24 text-center">No results.</Table.Cell>
                    </Table.Row>
                {/if}
            </Table.Body>
        </Table.Root>
    </div>
    <DTPagination {tableModel} />
</div>
