<script lang="ts">
	import { Plus } from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';
	import {
		DropdownMenu,
		DropdownMenuLabel,
		DropdownMenuItem,
		DropdownMenuTrigger,
		DropdownMenuContent,
		DropdownMenuGroup
	} from '$lib/components/ui/dropdown-menu';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Checkbox } from '$lib/components/ui/checkbox';

	type Resource = { name: string; available: boolean };

	// Demo data
	let resources = $state<Resource[]>([
		{ name: 'AUV-Alpha', available: true },
		{ name: 'AUV-Bravo', available: true },
		{ name: 'UUV-Charlie', available: false },
		{ name: 'ROV-Delta', available: true },
		{ name: 'Glider-Echo', available: false },
		{ name: 'Sensor-Foxtrot', available: true },
		{ name: 'Beacon-Golf', available: false },
		{ name: 'USV-Hotel', available: true }
	]);

	let query = $state('');
	// Defaults to false as requested
	let showUnavailable = $state(false);

	const normalizedQuery = $derived(query.trim().toLowerCase());

	// If showUnavailable is false, filter out unavailable items. Otherwise include all.
	const filteredResources = $derived(
		resources
			.filter((r) => showUnavailable || r.available)
			.filter((r) => r.name.toLowerCase().includes(normalizedQuery))
	);
</script>

<DropdownMenu>
	<DropdownMenuTrigger>
		<Button
			variant="ghost"
			size="icon"
			class="h-auto w-auto p-1 rounded text-blue-600 hover:text-blue-800 hover:bg-blue-50 focus-visible:ring-1"
			title="Add resource"
		>
			<Plus size={14} aria-hidden="true" />
		</Button>
	</DropdownMenuTrigger>

	<DropdownMenuContent class="w-64">
		<DropdownMenuLabel>Find Resources</DropdownMenuLabel>

		<div class="px-2 py-2 space-y-3">
			<div class="space-y-1">
				<Input
					id="resource-search"
					placeholder="Start typing…"
					autocomplete="off"
					aria-label="Filter resources by name"
					bind:value={query}
				/>
			</div>

			<div class="flex items-center gap-2">
				<Checkbox
					id="show-unavailable"
					bind:checked={showUnavailable}
					aria-label="Show unavailable resources"
				/>
				<Label for="show-unavailable">Show unavailable</Label>
			</div>
		</div>

		<DropdownMenuGroup>
			{#if filteredResources.length === 0}
				<DropdownMenuItem disabled>No matches</DropdownMenuItem>
			{:else}
				{#each filteredResources as r (r)}
					<DropdownMenuItem>
						<span class="flex w-full items-center justify-between">
							<span>{r.name}</span>
							{#if !r.available}
								<span class="text-xs text-muted-foreground">unavailable</span>
							{/if}
						</span>
					</DropdownMenuItem>
				{/each}
			{/if}
		</DropdownMenuGroup>
	</DropdownMenuContent>
</DropdownMenu>
