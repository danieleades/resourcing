<script lang="ts">
	import { Plus } from '@lucide/svelte';
	import { Button } from '$lib/components/shadcn/button';
	import {
		DropdownMenu,
		DropdownMenuLabel,
		DropdownMenuItem,
		DropdownMenuTrigger,
		DropdownMenuContent,
		DropdownMenuGroup
	} from '$lib/components/shadcn/dropdown-menu';
	import { Input } from '$lib/components/shadcn/input';
	import { Label } from '$lib/components/shadcn/label';
	import { Checkbox } from '$lib/components/shadcn/checkbox';

	export type Item = { id: string; name: string; available: boolean };

	interface Props {
		items: Item[];
		onAdd: (id: string) => Promise<unknown> | unknown;
		onChange?: () => void;
	}

	const { items, onAdd, onChange }: Props = $props();

	// Demo data
	// let items = $state<Item[]>([
	// 	{ name: 'AUV-Alpha', available: true },
	// 	{ name: 'AUV-Bravo', available: true },
	// 	{ name: 'UUV-Charlie', available: false },
	// 	{ name: 'ROV-Delta', available: true },
	// 	{ name: 'Glider-Echo', available: false },
	// 	{ name: 'Sensor-Foxtrot', available: true },
	// 	{ name: 'Beacon-Golf', available: false },
	// 	{ name: 'USV-Hotel', available: true }
	// ]);

	let query = $state('');
	// Defaults to false as requested
	let showUnavailable = $state(false);

	const normalizedQuery = $derived(query.trim().toLowerCase());

	// If showUnavailable is false, filter out unavailable items. Otherwise include all.
	const filteredItems = $derived(
		items
			.filter((i) => showUnavailable || i.available)
			.filter((i) => i.name.toLowerCase().includes(normalizedQuery))
	);

	async function handleSelect(id: string) {
		const result = await onAdd(id);
		if (result === false) return;
		onChange?.();
	}
</script>

<DropdownMenu>
	<DropdownMenuTrigger>
		<Button
			variant="ghost"
			size="icon"
			class="h-auto w-auto p-1 rounded text-blue-600 hover:text-blue-800 hover:bg-blue-50 focus-visible:ring-1"
			title="Add item"
		>
			<Plus size={14} aria-hidden="true" />
		</Button>
	</DropdownMenuTrigger>

	<DropdownMenuContent class="w-64">
		<DropdownMenuLabel>Find Items</DropdownMenuLabel>

		<div class="px-2 py-2 space-y-3">
			<div class="space-y-1">
				<Input
					id="item-search"
					placeholder="Start typing…"
					autocomplete="off"
					aria-label="Filter items by name"
					bind:value={query}
				/>
			</div>

			<div class="flex items-center gap-2">
				<Checkbox
					id="show-unavailable"
					bind:checked={showUnavailable}
					aria-label="Show unavailable items"
				/>
				<Label for="show-unavailable">Show unavailable</Label>
			</div>
		</div>

		<DropdownMenuGroup>
			{#if filteredItems.length === 0}
				<DropdownMenuItem disabled>No matches</DropdownMenuItem>
			{:else}
				{#each filteredItems as i (i)}
					<DropdownMenuItem onclick={() => void handleSelect(i.id)}>
						<span class="flex w-full items-center justify-between">
							<span>{i.name}</span>
							{#if !i.available}
								<span class="text-xs text-muted-foreground">unavailable</span>
							{/if}
						</span>
					</DropdownMenuItem>
				{/each}
			{/if}
		</DropdownMenuGroup>
	</DropdownMenuContent>
</DropdownMenu>
