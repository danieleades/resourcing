import { render, screen, fireEvent } from '@testing-library/svelte';
import { describe, it, expect, vi } from 'vitest';
import ItemCellDropdown from './ItemCellDropdown.svelte';

describe('ItemCellDropdown', () => {
	it('calls onAdd when an item is selected', async () => {
		const onAdd = vi.fn();
		render(ItemCellDropdown, { props: { onAdd } });

		const trigger = document.querySelector('[data-dropdown-menu-trigger]') as HTMLElement;
		await fireEvent.pointerDown(trigger);
		await fireEvent.pointerUp(trigger);

		const option = await screen.findByRole('menuitem', { name: 'AUV-Alpha' });
		await fireEvent.click(option);

		expect(onAdd).toHaveBeenCalledWith('AUV-Alpha');
	});
});
