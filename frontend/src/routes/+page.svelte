<script lang="ts">
	import type { ColumnDef } from '@tanstack/table-core';
	import { gql, GraphQLClient } from 'graphql-request';
	import { onMount } from 'svelte';
	import DataTable from './DataTable.svelte';
	import { makeColumns, type ProjectMonthMatrixRow, type ProjectMonthCell } from './columns';
	import type { GetProjectMonthMatrixQueryVariables } from '$lib/gql/graphql';

	// --- GraphQL (pivoted) ---
	const GET_PROJECT_MONTH_MATRIX = gql`
		query GetProjectMonthMatrix($months: [Month!]!) {
			projectMonthMatrix(months: $months) {
				months
				rows {
					project {
						id
						name
					}
					cells {
						resources {
							id
							name
						}
					}
				}
			}
			resources {
				id
				name
				assignments {
					month
					project {
						id
					}
				}
			}
		}
	`;

	// GraphQL client: build endpoint at call time using the current origin
	// so it remains absolute (required by some fetch implementations) but
	// still routes through Vite's dev proxy in development.

	// --- Fetch & mapping ---
	type RawProjectMonthMatrix = {
		projectMonthMatrix: {
			months: Array<unknown>;
			rows: Array<{
				project: { id: unknown; name: string };
				cells: Array<{
					resources: Array<{ id: unknown; name: string }>;
				}>;
			}>;
		};
		resources: Array<{
			id: unknown;
			name: string;
			assignments: Array<{
				month: unknown;
				project: { id: unknown };
			}>;
		}>;
	};

	async function fetchProjectMonthMatrix(months: string[]): Promise<{
		months: string[];
		rows: ProjectMonthMatrixRow[];
	}> {
		const endpoint = new URL('/graphql', window.location.origin).toString();
		const graphqlClient = new GraphQLClient(endpoint);
		const variables: GetProjectMonthMatrixQueryVariables = { months };
		const data = await graphqlClient.request<RawProjectMonthMatrix>(
			GET_PROJECT_MONTH_MATRIX,
			variables
		);

		const resources = data.resources.map((resource) => ({
			id: String(resource.id),
			name: resource.name,
			assignments: resource.assignments.map((assignment) => ({
				month: String(assignment.month),
				projectId: String(assignment.project.id)
			}))
		}));

		const outMonths = data.projectMonthMatrix.months.map(String);

		const outRows: ProjectMonthMatrixRow[] = data.projectMonthMatrix.rows.map((row) => {
			const projectId = String(row.project.id);

			return {
				projectId,
				projectName: row.project.name,
				cells: row.cells.map<ProjectMonthCell>((cell, columnIndex) => {
					const month = outMonths[columnIndex];
					const assignedResources = cell.resources.map((resource) => ({
						id: String(resource.id),
						name: resource.name
					}));

					const dropdownItems = resources
						.map((resource) => {
							const assignmentsInMonth = resource.assignments.filter(
								(assignment) => assignment.month === month
							);
							const assignedToCurrentProject = assignmentsInMonth.some(
								(assignment) => assignment.projectId === projectId
							);
							if (assignedToCurrentProject) {
								return null;
							}
							const assignedElsewhere = assignmentsInMonth.some(
								(assignment) => assignment.projectId !== projectId
							);
							return {
								id: resource.id,
								name: resource.name,
								available: !assignedElsewhere
							};
						})
						.filter((item): item is ProjectMonthCell['dropdownItems'][number] => item !== null);

					return {
						resources: assignedResources,
						dropdownItems
					};
				})
			};
		});

		return { months: outMonths, rows: outRows };
	}

	// --- state ---
	let rows = $state<ProjectMonthMatrixRow[]>([]);
	let columns = $state<ColumnDef<ProjectMonthMatrixRow>[]>([]);
	let error = $state<string | null>(null);
	let loading = $state(true);

	// Example: request a few months
	const requestedMonths = ['2025-06', '2025-07', '2025-08', '2025-09', '2025-10', '2025-11'];

	async function refreshData() {
		const res = await fetchProjectMonthMatrix(requestedMonths);
		rows = res.rows;
		columns = makeColumns(res.months, handleChange);
		error = null;
	}

	function handleChange() {
		void refreshData().catch((e) => {
			console.error('Failed to refresh matrix after change', e);
			error = e instanceof Error ? e.message : String(e);
		});
	}

	onMount(async () => {
		loading = true;
		await refreshData().catch((e) => {
			error = e instanceof Error ? e.message : String(e);
		});
		loading = false;
	});
</script>

<h1>Heading</h1>
{#if loading}
	<p>Loading…</p>
{:else if error}
	<p style="color: crimson;">Error: {error}</p>
{:else}
	<DataTable {columns} data={rows} />
{/if}
