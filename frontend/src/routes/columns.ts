import { ResourceCell } from '$lib/components/resource-cell';
import { renderComponent, type ColumnDef } from '@tanstack/svelte-table';

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
export function makeColumns(months: string[]): ColumnDef<ProjectMonthMatrixRow>[] {
	// First fixed column for project name
	const fixed: ColumnDef<ProjectMonthMatrixRow>[] = [
		{
			id: 'project',
			header: 'Project',
			accessorFn: (row) => row.projectName
		}
	];

	// One column per requested month (order preserved)
	const monthCols: ColumnDef<ProjectMonthMatrixRow>[] = months.map((m, i) => ({
		id: m,
		header: m,
		accessorFn: (row) => row.cells[i].resources,
		cell: ({ getValue }) => {
			const resources = (getValue() as RowResource[]).map((r) => r.name);
			return renderComponent(ResourceCell, { resources });
		}
	}));

	return [...fixed, ...monthCols];
}
