# Resource Allocation Matrix - Framework-Agnostic Specification

## 1. Scope and goals

A tabular UI for planning people against projects by month. The leftmost column lists projects. Each subsequent column represents a month. Each table cell lists the resources assigned to that project for that month. Rows are expandable to reveal a per-project “Resource Profile” chart that aligns with the month columns and visually indicates continuation beyond the visible window.

Primary goals:

- Fast editing of per-month assignments.
- Low-friction duplication of assignments to adjacent months.
- Accurate, column-centered chart that reads as a continuation outside the visible window without distorting scale.

Out of scope: capacity management, partial allocations (FTE percentages), skill matching, permissions.

---

## 2. Concepts and definitions

- **Project:** a planning entity with a name and a 12+ month allocation grid.
- **Resource:** a person or role assignable to a project in a given month.
- **Visible window:** the contiguous sequence of month-columns currently rendered in the table header.
- **Continuation:** the conceptual extension of the resource trend to months left and right of the visible window, indicated with dashed segments in the chart.

---

## 3. Data model

```ts
type MonthLabel = string; // e.g., "Jan 2025"
type ResourceName = string; // e.g., "Jane Smith"

type Project = {
	id: string | number;
	name: string;
	expanded: boolean;
	allocations: Record<MonthLabel, ResourceName[]>;
};
```

Assumptions:

- Month labels use the format "Mon YYYY" with three-letter English month abbreviations.
- A project may omit some months from `allocations`; missing implies no resources (empty list).

---

## 4. Layout

- **Header row:** one “Project” column + N month columns (N ≥ 1).
- **Body rows:** one row per project in collapsed state. When expanded:
  - The same row remains visible with its cells.
  - Directly underneath, a “Resource Profile” chart row spans the month columns and is vertically aligned with the header. The first cell under the “Project” column is a label cell for the chart.

- The table is horizontally scrollable when total width exceeds viewport.

---

## 5. Interactions

### 5.1 Collapsed row

- Each month cell shows a stack of assigned resources (badges or chips).
- Controls in each cell:
  - **Add resource** button opens a menu of available resources not yet in the cell.
  - **Remove resource** button on each badge removes it from the cell.
  - **Copy to previous month** button copies the cell’s assignments into the immediate previous month.
  - **Copy to next month** button copies into the immediate next month.

**Copy semantics:** overwrite destination with a clone of the source list (no merge, no dedup of destination, full replace).

### 5.2 Expanded row

- Only the chart is shown in the expanded region. Resource editing controls remain in the collapsed row cells.

### 5.3 Keyboard and mouse

- Clicking the chevron toggles the row.
- Hover reveals cell controls; for accessibility the controls must be tab-reachable.
- Keyboard:
  - Arrow keys navigate cells.
  - Enter activates the focused control.
  - Delete on a focused resource badge removes it.
  - Esc closes an open add-menu.

---

## 6. Chart specification

### 6.1 Goals

- Data points for visible months are centered under their respective header columns.
- The plotted series “continues” beyond the visible window to left and right.
- The continuation is styled as dashed and is clipped to half a column width on each side.
- The vertical scale includes 0 and avoids clipping peaks.

### 6.2 Coordinate system

- X-axis is numeric. Map visible month indices to integers: `0..N-1`.
- Domain is `[ -0.5, N - 0.5 ]`. This centers ticks at integers and clips half a column on either edge.

### 6.3 Series construction (per project)

- Let `count[i] = allocations[month[i]].length` for `i in [0..N-1]`.
- Compute adjacent hidden months:
  - `leftMonth = prev(month[0])`
  - `rightMonth = next(month[N-1])`
  - `leftCount = allocations[leftMonth]?.length || 0`
  - `rightCount = allocations[rightMonth]?.length || 0`

**Visible solid series:**

```
points = [ { x: i, y: count[i] } for i in 0..N-1 ]
```

**Left dashed half-segment:**

- From `x = -0.5` to `x = 0`
- Values: start at midpoint between `leftCount` and `count[0]` to preserve scale when clipped.

```
leftSegment = [
  { x: -0.5, y: (leftCount + count[0]) / 2 },
  { x: 0,    y:  count[0] }
]
```

**Right dashed half-segment:**

- From `x = N-1` to `x = N-0.5`
- Values: end at midpoint between `count[N-1]` and `rightCount`.

```
rightSegment = [
  { x: N-1,     y: count[N-1] },
  { x: N-0.5,   y: (count[N-1] + rightCount) / 2 }
]
```

**Rationale:** Using midpoints at the clipped boundaries avoids a visual artifact where a full segment is compressed into a half width.

### 6.4 Styling

- Solid visible series: stroke width 2, circular markers at visible integer x only.
- Dashed segments: same color, `strokeDasharray "4 4"`, no markers.
- Y-axis is visually hidden to avoid gutter; domain includes zero and headroom:
  - `domain = [ 0, max(1, ceil(maxY + 0.5)) ]`

- Grid lines optional. If shown, prefer light horizontal lines only.

### 6.5 Tooltip

- Shows “Allocated: <count> resources”.
- Label uses the visible month for integer x values.
- For half-step dashed points, either no label or a special label “outside window”.

---

## 7. Component structure (abstract)

- **ResourceAllocationTable**
  - Props:
    - `projects: Project[]`
    - `months: MonthLabel[]`
    - `availableResources: ResourceName[]`
    - Callbacks: `onAddResource`, `onRemoveResource`, `onCopyCell`, `onToggleExpanded`

  - State: may be external or internal. For Svelte, you can pass stores or props with events.

- **ResourceCell**
  - Inputs: `projectId`, `month`, `resourcesInCell`, `isFirst`, `isLast`
  - Emits: `add(resource)`, `remove(resource)`, `copyToPrev()`, `copyToNext()`

- **ResourceAddMenu**
  - Inputs: `availableToAdd: ResourceName[]`
  - Emits: `select(resource)`, `close()`

- **ResourceChart**
  - Inputs: `counts: number[]` aligned to `months`, plus `leftCount` and `rightCount`
  - Behavior: builds `points`, `leftSegment`, `rightSegment` as above and renders as aligned with header columns.

---

## 8. Algorithms

### 8.1 Month label helpers

- `parseMonthLabel("Mon YYYY") -> { m: 0..11, y: number }`
- `formatMonthLabel(y, m) -> "Mon YYYY"`
- `prevMonthLabel(label)` wraps at year boundaries.
- `nextMonthLabel(label)` wraps at year boundaries.

### 8.2 Available-to-add filtering

- In an add menu, exclude resources already assigned in that cell:
  - `availableToAdd = allResources - currentCellResources`.

### 8.3 Copy semantics

- On copy to prev or next:
  - `destinationList = clone(sourceList)`
  - Replace destination list entirely.

---

## 9. Accessibility

- Buttons and menus have `aria-label` and `title`.
- The expand/collapse button has `aria-expanded` and `aria-controls` referencing the chart row id.
- Focus order:
  1. Project toggle
  2. Cell resource badges (each removable)
  3. Cell add button
  4. Copy to prev
  5. Copy to next

- Keyboard:
  - Space/Enter activates controls.
  - Esc closes the add menu.
  - Tab order must remain stable when rows expand.

---

## 10. Error handling and edge cases

- Adding a duplicate resource is a no-op.
- Removing a resource not in the cell is a no-op.
- Copy into a month with no existing entry creates it.
- Copy at boundaries:
  - If there is no previous month in `months`, disable or hide “copy to previous”.
  - If there is no next month in `months`, disable or hide “copy to next”.

- Chart empty states:
  - If all counts in-window are zero and left/right are zero, render a flat line at zero. Tooltip still works.

---

## 11. Performance considerations

- Do not re-render all rows when editing a single cell. In Svelte, isolate reactive stores per row or per project.
- Virtualize rows if the number of projects is large.
- Defer expensive chart calculations to the expanded state only.
- Avoid layout thrash; keep column widths stable to preserve chart alignment.

---

## 12. Theming and styling

- Use a neutral palette with blue accent for interactive elements.
- Resource badges: small rounded chips with remove affordance.
- Hover-only controls should also be reachable by keyboard; ensure visible focus styles.
- Dashed segments use the same hue as the solid series to emphasize continuity, with reduced visual weight through dashing rather than color change.

---

## 13. Internationalization

- Month labels are currently English short names. If localization is needed:
  - Use a locale-aware formatter to produce headers.
  - Update parse/format helpers accordingly.

- Right-to-left is not targeted in this version.

---

## 14. Persistence and API contracts (example)

Endpoints (illustrative):

- `GET /projects?months=Jan%202025..Jun%202025` → `Project[]`
- `POST /projects/:id/allocations` body `{ month: MonthLabel, resources: ResourceName[] }` → replace list
- `PATCH /projects/:id/allocations/add` body `{ month, resource }`
- `PATCH /projects/:id/allocations/remove` body `{ month, resource }`

Consistency rules:

- Server is source of truth.
- On optimistic updates, reconcile on failure with a toast and rollback.

---

## 15. Telemetry

- Track add/remove/copy events with project id, month, resource id (hashed or anonymized).
- Track expand/collapse, time-in-expanded state, and add-menu open rate.

---

## 16. Acceptance criteria and test cases

### 16.1 Cell editing

- **Add resource:** Given a cell without “Alex Brown”, when I add “Alex Brown” then the cell lists “Alex Brown” and the add menu no longer offers it.
- **Duplicate add:** Given a cell with “Jane Smith”, adding “Jane Smith” again does not duplicate it.
- **Remove resource:** Given a cell with “Mike Johnson”, when I remove “Mike Johnson” then the cell no longer lists it.

### 16.2 Copy behavior

- **Copy to previous:** Given month i has \[A, B], when I copy to previous, month i-1 becomes \[A, B] exactly, replacing any prior content.
- **Copy to next:** Symmetric to previous.
- **Boundary copy:** When i=0, “copy to previous” is disabled. When i=N-1, “copy to next” is disabled.

### 16.3 Chart alignment and continuation

- **Centering:** For N visible months, data markers for the solid series render at x = 0..N-1, visually centered under each column header.
- **Left dashed edge:** If leftCount=0 and count\[0]=1, the dashed left boundary at x=-0.5 is 0.5, and the line connects to 1 at x=0. The marker is not rendered on dashed points.
- **Right dashed edge:** If count\[N-1]=2 and rightCount=0, the dashed right boundary at x=N-0.5 is 1.0, and the line connects from 2 at x=N-1. No dashed markers.
- **Vertical scale:** Y-axis domain includes 0 and the maximum visible value plus headroom; no point is clipped.

### 16.4 Accessibility

- **ARIA state:** Toggling a row updates `aria-expanded`. `aria-controls` points to the chart row id.
- **Keyboard navigation:** Tab and Shift+Tab reach all actionable controls in a predictable order. Esc closes add menu.

### 16.5 Resilience

- **Network failure:** On failed add/remove/copy, UI reverts to previous state and shows an error toast.

---

## 17. Migration notes for Svelte + shadcn-svelte

- Replace React state with Svelte stores or component-local reactivity.
- Menus: use `DropdownMenu` from shadcn-svelte and ensure focus management.
- Icons: lucide-svelte is a drop-in for icons like `ChevronDown` and `Plus`.
- Charting: choose a Svelte chart library that supports:
  - Numeric x-axis with custom domain, custom ticks.
  - Multiple series with independent data arrays.
  - Dashed strokes and no markers on specific series.
  - Hidden Y-axis with explicit domain control.
  - Tooltip customization.
    If such a library is not available, render a lightweight SVG with scales from d3-shape and d3-scale.

---

example code (react):

```tsx
import React, { useState } from 'react';
import { ChevronDown, ChevronRight, Plus, Copy, X } from 'lucide-react';
import {
	LineChart,
	Line,
	XAxis,
	YAxis,
	CartesianGrid,
	Tooltip,
	ResponsiveContainer
} from 'recharts';

// -------------------------------------------------
// Month label helpers (top-level so we can test them)
// -------------------------------------------------
const MONTHS_ABBR = [
	'Jan',
	'Feb',
	'Mar',
	'Apr',
	'May',
	'Jun',
	'Jul',
	'Aug',
	'Sep',
	'Oct',
	'Nov',
	'Dec'
];
function parseMonthLabel(label) {
	const [mon, yearStr] = label.split(' ');
	return { m: MONTHS_ABBR.indexOf(mon), y: parseInt(yearStr, 10) };
}
function formatMonthLabel(y, m) {
	return `${MONTHS_ABBR[m]} ${y}`;
}
function prevMonthLabel(label) {
	const { m, y } = parseMonthLabel(label);
	const pm = (m + 11) % 12;
	const py = m === 0 ? y - 1 : y;
	return formatMonthLabel(py, pm);
}
function nextMonthLabel(label) {
	const { m, y } = parseMonthLabel(label);
	const nm = (m + 1) % 12;
	const ny = m === 11 ? y + 1 : y;
	return formatMonthLabel(ny, nm);
}

// ---------------------------------------------
// Resource Allocation Table (single definition)
// ---------------------------------------------
const ResourceAllocationTable = () => {
	const [projects, setProjects] = useState([
		{
			id: 1,
			name: 'Project Alpha',
			expanded: false,
			allocations: {
				'Jan 2025': ['John Doe', 'Jane Smith'],
				'Feb 2025': ['John Doe', 'Mike Johnson'],
				'Mar 2025': ['Jane Smith', 'Sarah Wilson'],
				'Apr 2025': ['John Doe', 'Jane Smith', 'Mike Johnson'],
				'May 2025': ['Sarah Wilson'],
				'Jun 2025': ['John Doe', 'Sarah Wilson']
			}
		},
		{
			id: 2,
			name: 'Project Beta',
			expanded: false,
			allocations: {
				'Jan 2025': ['Mike Johnson'],
				'Feb 2025': ['Mike Johnson', 'Sarah Wilson'],
				'Mar 2025': ['John Doe', 'Mike Johnson'],
				'Apr 2025': ['Sarah Wilson'],
				'May 2025': ['John Doe', 'Jane Smith'],
				'Jun 2025': ['Jane Smith', 'Mike Johnson', 'Sarah Wilson']
			}
		},
		{
			id: 3,
			name: 'Project Gamma',
			expanded: false,
			allocations: {
				'Jan 2025': ['Sarah Wilson'],
				'Feb 2025': ['Jane Smith'],
				'Mar 2025': ['John Doe'],
				'Apr 2025': ['Mike Johnson', 'Sarah Wilson'],
				'May 2025': ['John Doe', 'Jane Smith', 'Sarah Wilson'],
				'Jun 2025': ['John Doe']
			}
		}
	]);

	const months = ['Jan 2025', 'Feb 2025', 'Mar 2025', 'Apr 2025', 'May 2025', 'Jun 2025'];
	const availableResources = [
		'John Doe',
		'Jane Smith',
		'Mike Johnson',
		'Sarah Wilson',
		'Alex Brown',
		'Lisa Chen'
	];

	const toggleExpanded = (projectId) => {
		setProjects((prev) =>
			prev.map((p) => (p.id === projectId ? { ...p, expanded: !p.expanded } : p))
		);
	};

	const addResource = (projectId, month, resource) => {
		setProjects((prev) =>
			prev.map((p) => {
				if (p.id !== projectId) return p;
				const currentResources = p.allocations[month] || [];
				if (currentResources.includes(resource)) return p;
				return {
					...p,
					allocations: {
						...p.allocations,
						[month]: [...currentResources, resource]
					}
				};
			})
		);
	};

	const removeResource = (projectId, month, resource) => {
		setProjects((prev) =>
			prev.map((p) => {
				if (p.id !== projectId) return p;
				const currentResources = p.allocations[month] || [];
				return {
					...p,
					allocations: {
						...p.allocations,
						[month]: currentResources.filter((r) => r !== resource)
					}
				};
			})
		);
	};

	const copyCell = (projectId, fromMonth, toMonth) => {
		setProjects((prev) =>
			prev.map((p) => {
				if (p.id !== projectId) return p;
				const sourceResources = p.allocations[fromMonth] || [];
				return {
					...p,
					allocations: {
						...p.allocations,
						[toMonth]: [...sourceResources]
					}
				};
			})
		);
	};

	// Build a profile that includes out-of-window dashed segments (half-width clipped)
	const getResourceProfile = (project) => {
		const N = months.length;
		const core = months.map((month, idx) => ({
			x: idx,
			monthLabel: month.split(' ')[0],
			count: (project.allocations[month] || []).length
		}));

		const leftLabel = prevMonthLabel(months[0]);
		const rightLabel = nextMonthLabel(months[N - 1]);
		const leftCount = (project.allocations[leftLabel] || []).length || 0;
		const rightCount = (project.allocations[rightLabel] || []).length || 0;
		const first = core[0]?.count ?? 0;
		const last = core[N - 1]?.count ?? 0;

		// Half-width, clipped edges: only the visible half of each adjacent segment.
		// Left shows from x=-0.5 to x=0; Right shows from x=N-1 to x=N-0.5.
		const dashedLeft = [
			{ x: -0.5, count: (leftCount + first) / 2 },
			{ x: 0, count: first }
		];
		const dashedRight = [
			{ x: N - 1, count: last },
			{ x: N - 0.5, count: (last + rightCount) / 2 }
		];

		return { core, dashedLeft, dashedRight };
	};

	const ResourceDropdown = ({ projectId, month, currentResources }) => {
		const [isOpen, setIsOpen] = useState(false);
		const availableToAdd = availableResources.filter((r) => !currentResources.includes(r));

		return (
			<div className="relative">
				<button
					onClick={() => setIsOpen(!isOpen)}
					className="p-1 text-blue-600 hover:text-blue-800 hover:bg-blue-50 rounded"
					title="Add resource"
					aria-label="Add resource"
				>
					<Plus size={14} />
				</button>
				{isOpen && (
					<div className="absolute z-10 mt-1 bg-white border border-gray-200 rounded-md shadow-lg min-w-32">
						{availableToAdd.map((resource) => (
							<button
								key={resource}
								onClick={() => {
									addResource(projectId, month, resource);
									setIsOpen(false);
								}}
								className="block w-full px-3 py-2 text-left text-sm hover:bg-gray-100"
							>
								{resource}
							</button>
						))}
						{availableToAdd.length === 0 && (
							<div className="px-3 py-2 text-sm text-gray-500">No resources available</div>
						)}
					</div>
				)}
			</div>
		);
	};

	const ResourceCell = ({ project, month, monthIndex }) => {
		const resources = project.allocations[month] || [];

		return (
			<td className="px-3 py-2 border-r border-gray-200 relative group align-top">
				<div className="min-h-12 flex flex-col gap-1">
					{resources.map((resource, idx) => (
						<div
							key={idx}
							className="flex items-center justify-between bg-blue-50 px-2 py-1 rounded text-xs"
						>
							<span className="text-blue-800">{resource}</span>
							<button
								onClick={() => removeResource(project.id, month, resource)}
								className="text-blue-600 hover:text-red-600 ml-1"
								title="Remove resource"
								aria-label={`Remove ${resource}`}
							>
								<X size={12} />
							</button>
						</div>
					))}

					<div className="flex items-center gap-1 mt-1 opacity-0 group-hover:opacity-100 transition-opacity">
						<ResourceDropdown projectId={project.id} month={month} currentResources={resources} />

						{monthIndex > 0 && (
							<button
								onClick={() => copyCell(project.id, month, months[monthIndex - 1])}
								className="p-1 text-gray-500 hover:text-gray-700 hover:bg-gray-100 rounded"
								title="Copy to previous month"
								aria-label="Copy to previous month"
							>
								<Copy size={14} />
							</button>
						)}

						{monthIndex < months.length - 1 && (
							<button
								onClick={() => copyCell(project.id, month, months[monthIndex + 1])}
								className="p-1 text-gray-500 hover:text-gray-700 hover:bg-gray-100 rounded"
								title="Copy to next month"
								aria-label="Copy to next month"
							>
								<Copy size={14} className="transform scale-x-[-1]" />
							</button>
						)}
					</div>
				</div>
			</td>
		);
	};

	return (
		<div className="p-6 bg-gray-50 min-h-screen">
			<div className="max-w-7xl mx-auto">
				<h1 className="text-2xl font-bold text-gray-900 mb-6">Resource Allocation</h1>

				<div className="bg-white rounded-lg shadow-sm border border-gray-200 overflow-hidden">
					<div className="overflow-x-auto">
						<table className="w-full">
							<thead>
								<tr className="bg-gray-50 border-b border-gray-200">
									<th className="px-6 py-3 text-left text-sm font-semibold text-gray-900 min-w-48">
										Project
									</th>
									{months.map((month) => (
										<th
											key={month}
											className="px-3 py-3 text-center text-sm font-semibold text-gray-900 min-w-32 border-r border-gray-200"
										>
											{month}
										</th>
									))}
								</tr>
							</thead>
							<tbody>
								{projects.map((project) => (
									<React.Fragment key={project.id}>
										<tr className="border-b border-gray-200 hover:bg-gray-50 align-top">
											<td className="px-6 py-4 border-r border-gray-200 align-top">
												<button
													onClick={() => toggleExpanded(project.id)}
													className="flex items-center gap-2 text-left font-medium text-gray-900 hover:text-blue-600"
													aria-expanded={project.expanded}
													aria-controls={`proj-${project.id}-chart`}
												>
													{project.expanded ? (
														<ChevronDown size={16} />
													) : (
														<ChevronRight size={16} />
													)}
													{project.name}
												</button>
											</td>

											{months.map((month, monthIndex) => (
												<ResourceCell
													key={month}
													project={project}
													month={month}
													monthIndex={monthIndex}
												/>
											))}
										</tr>

										{project.expanded && (
											<tr
												id={`proj-${project.id}-chart`}
												className="bg-gray-50 border-b border-gray-200"
											>
												{/* spacer under project column */}
												<td className="px-6 py-4 border-r border-gray-200 align-top">
													<h3 className="font-semibold text-gray-900">Resource Profile</h3>
												</td>
												<td colSpan={months.length} className="px-4 py-4 align-top">
													<div className="h-64 w-full">
														<ResponsiveContainer width="100%" height="100%">
															<LineChart
																data={getResourceProfile(project).core}
																margin={{ left: 0, right: 0, top: 8, bottom: 0 }}
															>
																<CartesianGrid strokeDasharray="3 3" />
																<XAxis
																	type="number"
																	dataKey="x"
																	domain={[-0.5, months.length - 0.5]}
																	ticks={months.map((_, i) => i)}
																	tickFormatter={(v) =>
																		Number.isInteger(v) ? months[v].split(' ')[0] : ''
																	}
																/>
																<YAxis
																	hide
																	width={0}
																	domain={[0, (dataMax) => Math.max(1, dataMax + 0.5)]}
																	allowDecimals={false}
																/>
																<Tooltip
																	formatter={(value) => [`${value} resources`, 'Allocated']}
																	labelFormatter={(label) =>
																		Number.isInteger(label)
																			? `Month: ${months[label].split(' ')[0]}`
																			: ''
																	}
																/>
																{/* core (visible window) */}
																<Line
																	type="linear"
																	dataKey="count"
																	stroke="#2563eb"
																	strokeWidth={2}
																	dot={{ r: 4 }}
																	activeDot={{ r: 5 }}
																/>
																{/* dashed out-of-window segments (half width, clipped) */}
																<Line
																	type="linear"
																	data={getResourceProfile(project).dashedLeft}
																	dataKey="count"
																	stroke="#2563eb"
																	strokeWidth={2}
																	strokeDasharray="4 4"
																	dot={false}
																	isAnimationActive={false}
																/>
																<Line
																	type="linear"
																	data={getResourceProfile(project).dashedRight}
																	dataKey="count"
																	stroke="#2563eb"
																	strokeWidth={2}
																	strokeDasharray="4 4"
																	dot={false}
																	isAnimationActive={false}
																/>
															</LineChart>
														</ResponsiveContainer>
													</div>
												</td>
											</tr>
										)}
									</React.Fragment>
								))}
							</tbody>
						</table>
					</div>
				</div>
			</div>
		</div>
	);
};

export default ResourceAllocationTable;

// -------------------------------------------------
// Lightweight self-tests (run if window.__RAM_RUN_TESTS__)
// -------------------------------------------------
// These tests exercise core data operations and month helpers.
// To run in a browser console:
//   window.__RAM_RUN_TESTS__ = true; location.reload();
// Results will be logged to the console.

/** @typedef {{id:number,name:string,expanded:boolean,allocations:Record<string,string[]>}} Project */

function cloneProjects(projects /*: Project[] */) {
	return projects.map((p) => ({
		...p,
		allocations: Object.fromEntries(Object.entries(p.allocations).map(([k, v]) => [k, [...v]]))
	}));
}

function addResourceToProjects(projects, projectId, month, resource) {
	const next = cloneProjects(projects);
	const p = next.find((x) => x.id === projectId);
	if (!p) return next;
	const list = p.allocations[month] || (p.allocations[month] = []);
	if (!list.includes(resource)) list.push(resource);
	return next;
}

function removeResourceFromProjects(projects, projectId, month, resource) {
	const next = cloneProjects(projects);
	const p = next.find((x) => x.id === projectId);
	if (!p) return next;
	const list = p.allocations[month] || [];
	p.allocations[month] = list.filter((r) => r !== resource);
	return next;
}

function copyCellInProjects(projects, projectId, fromMonth, toMonth) {
	const next = cloneProjects(projects);
	const p = next.find((x) => x.id === projectId);
	if (!p) return next;
	const src = p.allocations[fromMonth] || [];
	p.allocations[toMonth] = [...src];
	return next;
}

function toggleExpandedInProjects(projects, projectId) {
	const next = cloneProjects(projects);
	const p = next.find((x) => x.id === projectId);
	if (p) p.expanded = !p.expanded;
	return next;
}

function deepEqual(a, b) {
	return JSON.stringify(a) === JSON.stringify(b);
}

function runSelfTests() {
	const initial = [
		{ id: 1, name: 'P1', expanded: false, allocations: { 'Jan 2025': ['A'], 'Feb 2025': [] } },
		{ id: 2, name: 'P2', expanded: true, allocations: { 'Jan 2025': [], 'Feb 2025': ['B'] } }
	];

	const tests = [];

	// --- Month helpers ---
	tests.push(() => prevMonthLabel('Jan 2025') === 'Dec 2024');
	tests.push(() => nextMonthLabel('Jun 2025') === 'Jul 2025');
	tests.push(() => prevMonthLabel('Mar 2025') === 'Feb 2025');
	tests.push(() => nextMonthLabel('Dec 2025') === 'Jan 2026');

	// Add resource (new) only affects target project
	tests.push(() => {
		const next = addResourceToProjects(initial, 1, 'Jan 2025', 'X');
		const unaffected = next[1].allocations['Jan 2025'].length === 0;
		return unaffected && next[0].allocations['Jan 2025'].includes('X');
	});

	// Add resource (duplicate no-op)
	tests.push(() => {
		const next = addResourceToProjects(initial, 1, 'Jan 2025', 'A');
		return next[0].allocations['Jan 2025'].filter((r) => r === 'A').length === 1;
	});

	// Remove resource
	tests.push(() => {
		const next = removeResourceFromProjects(initial, 1, 'Jan 2025', 'A');
		return !next[0].allocations['Jan 2025'].includes('A');
	});

	// Copy cell (clone not reference)
	tests.push(() => {
		const next = copyCellInProjects(initial, 2, 'Feb 2025', 'Jan 2025');
		const sameRef = next[1].allocations['Jan 2025'] === initial[1].allocations['Feb 2025'];
		const sameValues = deepEqual(
			next[1].allocations['Jan 2025'],
			initial[1].allocations['Feb 2025']
		);
		return !sameRef && sameValues;
	});

	// Copy overwrites destination (not merge)
	tests.push(() => {
		const start = [
			{
				id: 3,
				name: 'P3',
				expanded: false,
				allocations: { 'Jan 2025': ['A'], 'Feb 2025': ['B', 'C'] }
			}
		];
		const next = copyCellInProjects(start, 3, 'Jan 2025', 'Feb 2025');
		return deepEqual(next[0].allocations['Feb 2025'], ['A']);
	});

	// Copy from empty month results in empty destination
	tests.push(() => {
		const start = [
			{ id: 5, name: 'P5', expanded: false, allocations: { 'Jan 2025': [], 'Feb 2025': ['B'] } }
		];
		const next = copyCellInProjects(start, 5, 'Jan 2025', 'Feb 2025');
		return deepEqual(next[0].allocations['Feb 2025'], []);
	});

	// Remove from empty month is safe no-op
	tests.push(() => {
		const start = [
			{ id: 4, name: 'P4', expanded: false, allocations: { 'Jan 2025': [], 'Feb 2025': [] } }
		];
		const next = removeResourceFromProjects(start, 4, 'Jan 2025', 'Z');
		return deepEqual(next[0].allocations['Jan 2025'], []);
	});

	// Toggle expanded
	tests.push(() => {
		const next = toggleExpandedInProjects(initial, 1);
		return next[0].expanded === true && next[1].expanded === true;
	});

	const results = tests.map((fn, i) => ({ idx: i + 1, pass: !!fn() }));
	const passed = results.filter((r) => r.pass).length;
	// eslint-disable-next-line no-console
	console.log(`[ResourceAllocationTable] ${passed}/${results.length} tests passed`, results);
}

if (typeof window !== 'undefined' && window.__RAM_RUN_TESTS__) {
	try {
		runSelfTests();
	} catch (e) {
		console.error('Self-tests failed with error', e);
	}
}
```
