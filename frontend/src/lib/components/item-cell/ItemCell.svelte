<script lang="ts">
	import { Copy, X } from '@lucide/svelte';
	import { gql, GraphQLClient } from 'graphql-request';
	import { Button } from '$lib/components/shadcn/button';
	import ItemCellDropdown, { type Item as DropdownItem } from './ItemCellDropdown.svelte';
	import type { AssignInput, UnassignInput } from '$lib/gql/graphql';

	const ASSIGN_RESOURCE = gql`
		mutation AssignResource($input: AssignInput!) {
			assign(input: $input) {
				id
			}
		}
	`;

	const UNASSIGN_RESOURCE = gql`
		mutation UnassignResource($input: UnassignInput!) {
			unassign(input: $input)
		}
	`;

	interface AssignedItem {
		id: string;
		name: string;
	}

	interface Props {
		projectId: string;
		month: string;
		assignedItems: AssignedItem[];
		dropdownItems: DropdownItem[];
		onChange?: () => void;
	}

	const props = $props<Props>();

	let assigned = $state<AssignedItem[]>([]);
	let dropdownItems = $state<DropdownItem[]>([]);

	const hasItems = $derived(() => assigned.length > 0);

	function getClient() {
		return new GraphQLClient(new URL('/graphql', window.location.origin).toString());
	}

	$effect(() => {
		assigned = props.assignedItems.map((item) => ({ ...item }));
		dropdownItems = props.dropdownItems.map((item) => ({ ...item }));
	});

	async function handleAdd(id: string): Promise<boolean> {
		const candidate = dropdownItems.find((item) => item.id === id);
		if (!candidate) return false;

		assigned = [...assigned, { id: candidate.id, name: candidate.name }];
		dropdownItems = dropdownItems.filter((item) => item.id !== id);

		const input: AssignInput = {
			projectId: props.projectId,
			resourceId: candidate.id,
			month: props.month
		};

		try {
			await getClient().request<{ assign: { id: string } }, { input: AssignInput }>(
				ASSIGN_RESOURCE,
				{ input }
			);
			return true;
		} catch (error) {
			console.error('Failed to assign resource', error);
			assigned = assigned.filter((item) => item.id !== candidate.id);
			dropdownItems = [...dropdownItems, { ...candidate }];
			return false;
		}
	}

	async function handleRemove(id: string) {
		const candidate = assigned.find((item) => item.id === id);
		if (!candidate) return;

		assigned = assigned.filter((item) => item.id !== id);
		dropdownItems = [...dropdownItems, { id: candidate.id, name: candidate.name, available: true }];

		const input: UnassignInput = {
			projectId: props.projectId,
			resourceId: candidate.id,
			month: props.month
		};

		try {
			await getClient().request<{ unassign: boolean }, { input: UnassignInput }>(
				UNASSIGN_RESOURCE,
				{ input }
			);
			props.onChange?.();
		} catch (error) {
			console.error('Failed to unassign resource', error);
			assigned = [...assigned, candidate];
			dropdownItems = dropdownItems.filter((item) => item.id !== candidate.id);
		}
	}

	function handleAddWrapper(id: string) {
		return handleAdd(id);
	}
</script>

<div class="relative group align-top">
	<div class="min-h-12 flex flex-col gap-1">
		{#if hasItems()}
			{#each assigned as assignedItem (assignedItem.id)}
				<div class="flex items-center justify-between bg-blue-50 px-2 py-1 rounded text-xs">
					<span class="text-blue-800">{assignedItem.name}</span>

					<!-- Remove (disabled, visual only) -->
					<Button
						variant="ghost"
						size="icon"
						class="ml-1 h-auto w-auto p-0 rounded text-blue-600 hover:text-red-600 hover:bg-transparent focus-visible:ring-1"
						title="Remove item"
						onclick={() => void handleRemove(assignedItem.id)}
					>
						<X size={12} aria-hidden="true" />
					</Button>
				</div>
			{/each}
		{:else}
			<span class="text-gray-400 text-xs select-none">-</span>
		{/if}

		<!-- Hover strip -->
		<div
			class="flex items-center gap-1 mt-1 opacity-0 group-hover:opacity-100 group-focus-within:opacity-100 transition-opacity"
		>
			<!-- Add -->
			<ItemCellDropdown items={dropdownItems} onAdd={handleAddWrapper} onChange={props.onChange} />

			<Button
				variant="ghost"
				size="icon"
				class="h-auto w-auto p-1 rounded text-gray-500 hover:text-gray-700 hover:bg-gray-100 focus-visible:ring-1"
				title="Copy to previous month"
			>
				<Copy size={14} aria-hidden="true" />
			</Button>
			<Button
				variant="ghost"
				size="icon"
				class="h-auto w-auto p-1 rounded text-gray-500 hover:text-gray-700 hover:bg-gray-100 focus-visible:ring-1"
				title="Copy to next month"
			>
				<Copy size={14} class="transform scale-x-[-1]" aria-hidden="true" />
			</Button>
		</div>
	</div>
</div>
