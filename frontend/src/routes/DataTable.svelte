<script lang="ts">
	import { createSvelteTable, getCoreRowModel, flexRender } from '@tanstack/svelte-table';
	import { makeColumns, type ProjectMonthMatrixRow } from './columns';
	import {
		Table,
		TableHeader,
		TableRow,
		TableHead,
		TableBody,
		TableCell
	} from '$lib/components/shadcn/table';

	type DataTableProps = {
		months: string[];
		data: ProjectMonthMatrixRow[];
	};

	let { data, months }: DataTableProps = $props();

	function handleAdd(name: string) {
		void name;
		// no-op for now
	}

	const columns = $derived(() => makeColumns(months, handleAdd));

	const table = createSvelteTable({
		get data() {
			return data;
		},
		get columns() {
			return columns();
		},
		getCoreRowModel: getCoreRowModel()
	});
</script>

<div class="rounded-md border">
	<Table>
		<TableHeader>
			{#each $table.getHeaderGroups() as headerGroup (headerGroup.id)}
				<TableRow>
					{#each headerGroup.headers as header (header.id)}
						<TableHead colspan={header.colSpan}>
							{#if !header.isPlaceholder}
								{@const H = flexRender(header.column.columnDef.header, header.getContext())}
								{#if H}<H />{/if}
							{/if}
						</TableHead>
					{/each}
				</TableRow>
			{/each}
		</TableHeader>

		<TableBody>
			{#each $table.getRowModel().rows as row (row.id)}
				<TableRow data-state={row.getIsSelected() && 'selected'}>
					{#each row.getVisibleCells() as cell (cell.id)}
						<TableCell>
							{@const C = flexRender(cell.column.columnDef.cell, cell.getContext())}
							{#if C}<C />{/if}
						</TableCell>
					{/each}
				</TableRow>
			{:else}
				<TableRow>
					<TableCell colspan={columns().length} class="h-24 text-center">No results.</TableCell>
				</TableRow>
			{/each}
		</TableBody>
	</Table>
</div>
