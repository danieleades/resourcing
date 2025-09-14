import { ItemCell } from '$lib/components/item-cell';
import { renderComponent, type AccessorFnColumnDef } from '@tanstack/svelte-table';

// --- Row model for the table (pivoted) ---
export interface RowResource {
	id: string;
	name: string;
}
export interface ProjectMonthCell {
	resources: RowResource[];
}
export interface ProjectMonthMatrixRow {
	projectId: string;
	projectName: string;
	cells: ProjectMonthCell[]; // index j corresponds to months[j]
}

// --- TanStack columns (dynamic per months) ---
// Kept concise: use accessorFn and a local cast in the cell to avoid verbose generics
export function makeColumns(
	months: string[]
): AccessorFnColumnDef<ProjectMonthMatrixRow, unknown>[] {
	// First fixed column for project name
	const fixed: AccessorFnColumnDef<ProjectMonthMatrixRow, unknown>[] = [
		{
			id: 'project',
			header: 'Project',
			accessorFn: (row) => row.projectName
		}
	];

	// One column per requested month (order preserved)
	const monthCols: AccessorFnColumnDef<ProjectMonthMatrixRow, unknown>[] = months.map((m, i) => ({
		id: m,
		header: m,
		accessorFn: (row) => row.cells[i].resources,
		cell: ({ getValue }) => {
			const items = (getValue() as RowResource[]).map((r) => r.name);
			return renderComponent(ItemCell, { items });
		}
	}));

	return [...fixed, ...monthCols];
}
