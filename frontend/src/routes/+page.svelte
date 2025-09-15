<script lang="ts">
	import type { ColumnDef } from '@tanstack/table-core';
	import { gql, GraphQLClient } from 'graphql-request';
	import { onMount } from 'svelte';
	import DataTable from './DataTable.svelte';
	import { makeColumns, type ProjectMonthMatrixRow } from './columns';

	// After you run codegen with the new operation below, these types will exist:
	import type {
		GetProjectMonthMatrixQuery,
		GetProjectMonthMatrixQueryVariables
	} from '$lib/gql/graphql';

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
		}
	`;

	// GraphQL client: build endpoint at call time using the current origin
	// so it remains absolute (required by some fetch implementations) but
	// still routes through Vite's dev proxy in development.

	// --- Fetch & mapping ---
	async function fetchProjectMonthMatrix(months: string[]): Promise<{
		months: string[];
		rows: ProjectMonthMatrixRow[];
	}> {
		const endpoint = new URL('/graphql', window.location.origin).toString();
		const graphqlClient = new GraphQLClient(endpoint);
		const variables: GetProjectMonthMatrixQueryVariables = { months };
		const data = await graphqlClient.request<
			GetProjectMonthMatrixQuery,
			GetProjectMonthMatrixQueryVariables
		>(GET_PROJECT_MONTH_MATRIX, variables);

		const outMonths = data.projectMonthMatrix.months.map(String);

		const outRows: ProjectMonthMatrixRow[] = data.projectMonthMatrix.rows.map((r) => ({
			projectId: String(r.project.id),
			projectName: r.project.name,
			cells: r.cells.map((c) => ({
				resources: c.resources.map((rr) => ({ id: String(rr.id), name: rr.name }))
			}))
		}));

		return { months: outMonths, rows: outRows };
	}

	// --- state ---
	let rows = $state<ProjectMonthMatrixRow[]>([]);
	let columns = $state<ColumnDef<ProjectMonthMatrixRow>[]>([]);
	let error = $state<string | null>(null);
	let loading = $state(true);

	// Example: request a few months
	const requestedMonths = ['2025-06', '2025-07', '2025-08', '2025-09', '2025-10', '2025-11'];

	onMount(async () => {
		try {
			const res = await fetchProjectMonthMatrix(requestedMonths);
			rows = res.rows;
			columns = makeColumns(res.months);
		} catch (e) {
			error = e instanceof Error ? e.message : String(e);
		} finally {
			loading = false;
		}
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
