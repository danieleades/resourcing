import { ItemCell } from '$lib/components/item-cell';
import type { Item as DropdownItem } from '$lib/components/item-cell/ItemCellDropdown.svelte';
import { renderComponent, type AccessorFnColumnDef } from '@tanstack/svelte-table';

// --- Row model for the table (pivoted) ---
export interface RowResource {
	id: string;
	name: string;
}
export interface ProjectMonthCell {
	resources: RowResource[];
	dropdownItems: DropdownItem[];
}
export interface ProjectMonthMatrixRow {
	projectId: string;
	projectName: string;
	cells: ProjectMonthCell[]; // index j corresponds to months[j]
}

// --- TanStack columns (dynamic per months) ---
// Kept concise: use accessorFn and a local cast in the cell to avoid verbose generics
export function makeColumns(
	months: string[],
	onChange: () => void
): AccessorFnColumnDef<ProjectMonthMatrixRow>[] {
	// First fixed column for project name
	const fixed: AccessorFnColumnDef<ProjectMonthMatrixRow>[] = [
		{
			id: 'project',
			header: 'Project',
			accessorFn: (row) => row.projectName
		}
	];

	// One column per requested month (order preserved)
	const monthCols: AccessorFnColumnDef<ProjectMonthMatrixRow>[] = months.map((m, i) => ({
		id: m,
		header: m,
		accessorFn: (row) => row.cells[i],
		cell: ({ getValue, row }) => {
			const cellData = getValue() as ProjectMonthCell;
			return renderComponent(ItemCell, {
				projectId: row.original.projectId,
				month: m,
				assignedItems: cellData.resources,
				dropdownItems: cellData.dropdownItems,
				onChange
			});
		}
	}));

	return [...fixed, ...monthCols];
}
