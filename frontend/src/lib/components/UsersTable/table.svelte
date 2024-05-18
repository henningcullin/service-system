<script>
    import { Render, Subscribe, createRender, createTable } from 'svelte-headless-table';
    import { users } from '$stores';
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
        DTFieldCell,
        DTToolbar,
        DTIdCell,
        DTDateCell,
        DTFacilityCell,
        DTRoleCell,
        DTEmailCell,
    } from './index.js';

    import * as Table from '$lib/components/ui/table/index.js';

    const table = createTable(users, {
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
            accessor: 'first_name',
            id: 'first_name',
            header: 'First Name',
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
            accessor: 'last_name',
            id: 'last_name',
            header: 'Last Name',
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
            accessor: 'email',
            id: 'email',
            header: 'Email',
            cell: ({ value, row }) => {
                if (row.isData()) {
                    return createRender(DTEmailCell, {
                        value,
                    });
                }
                return value;
            },
        }),
        table.column({
            accessor: 'phone',
            id: 'phone',
            header: 'Phone',
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
            accessor: 'role',
            id: 'role',
            header: 'Role',
            cell: ({ value }) => {
                return createRender(DTRoleCell, {
                    value,
                });
            },
        }),
        table.column({
            accessor: 'facility',
            id: 'facility',
            header: 'Facility',
            cell: ({ value }) => {
                return createRender(DTFacilityCell, {
                    value,
                });
            },
        }),
        table.column({
            accessor: 'occupation',
            id: 'occupation',
            header: 'Occupation',
            cell: ({ value }) => {
                return createRender(DTFieldCell, {
                    value,
                });
            },
        }),
        table.column({
            accessor: 'last_login',
            id: 'last_login',
            header: 'Last login',
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
    <DTToolbar {tableModel} {users} />
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
