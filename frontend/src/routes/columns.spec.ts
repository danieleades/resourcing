import { describe, it, expect } from 'vitest';
import { makeColumns, type ProjectMonthMatrixRow } from './columns';

describe('makeColumns', () => {
	it('creates a project column and one column per month', () => {
		const months = ['2025-06', '2025-07'];
		const columns = makeColumns(months);
		expect(columns).toHaveLength(3);
		expect(columns[0].id).toBe('project');
		expect(columns[0].header).toBe('Project');
		expect(columns[1].id).toBe('2025-06');
		expect(columns[2].id).toBe('2025-07');

		const row: ProjectMonthMatrixRow = {
			projectId: 'p1',
			projectName: 'Project One',
			cells: [
				{ resources: [{ id: 'r1', name: 'Res1' }] },
				{ resources: [{ id: 'r2', name: 'Res2' }] }
			]
		};
		const accessor = columns[1].accessorFn;
		expect(accessor(row, 0)).toEqual(row.cells[0].resources);
	});
});
