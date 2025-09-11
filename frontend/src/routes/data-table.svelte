<script lang="ts">
	import { type ColumnDef } from '@tanstack/table-core';
	import { createSvelteTable, getCoreRowModel, flexRender } from '@tanstack/svelte-table';
	import * as Table from '$lib/components/ui/table/index.js';
	import type { ProjectMonthMatrixRow } from './columns';

	type DataTableProps = {
		columns: ColumnDef<ProjectMonthMatrixRow>[];
		data: ProjectMonthMatrixRow[];
	};

	let { data, columns }: DataTableProps = $props();

	const table = createSvelteTable({
		get data() {
			return data;
		},
		columns,
		getCoreRowModel: getCoreRowModel()
	});
</script>

<div class="rounded-md border">
	<Table.Root>
		<Table.Header>
			{#each $table.getHeaderGroups() as headerGroup (headerGroup.id)}
				<Table.Row>
					{#each headerGroup.headers as header (header.id)}
						<Table.Head colspan={header.colSpan}>
							{#if !header.isPlaceholder}
								{@const H = flexRender(header.column.columnDef.header, header.getContext())}
								{#if H}<H />{/if}
							{/if}
						</Table.Head>
					{/each}
				</Table.Row>
			{/each}
		</Table.Header>

		<Table.Body>
			{#each $table.getRowModel().rows as row (row.id)}
				<Table.Row data-state={row.getIsSelected() && 'selected'}>
					{#each row.getVisibleCells() as cell (cell.id)}
						<Table.Cell>
							{@const C = flexRender(cell.column.columnDef.cell, cell.getContext())}
							{#if C}<C />{/if}
						</Table.Cell>
					{/each}
				</Table.Row>
			{:else}
				<Table.Row>
					<Table.Cell colspan={columns.length} class="h-24 text-center">No results.</Table.Cell>
				</Table.Row>
			{/each}
		</Table.Body>
	</Table.Root>
</div>
