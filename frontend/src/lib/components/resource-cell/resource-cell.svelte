<script lang="ts">
	import { Copy, X } from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';
	import ResourceCellDropdown from './resource-cell-dropdown.svelte';

	export interface Props {
		resources: string[];
	}

	const { resources = [] }: Props = $props();

	const hasResources = $derived(() => resources.length > 0);
</script>

<div class="relative group align-top">
	<div class="min-h-12 flex flex-col gap-1">
		{#if hasResources()}
			{#each resources as resource (resource)}
				<div class="flex items-center justify-between bg-blue-50 px-2 py-1 rounded text-xs">
					<span class="text-blue-800">{resource}</span>

					<!-- Remove (disabled, visual only) -->
					<Button
						variant="ghost"
						size="icon"
						disabled
						aria-disabled="true"
						class="ml-1 h-auto w-auto p-0 rounded text-blue-600 hover:text-red-600 hover:bg-transparent focus-visible:ring-1"
						title="Remove resource"
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
			<ResourceCellDropdown />

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
