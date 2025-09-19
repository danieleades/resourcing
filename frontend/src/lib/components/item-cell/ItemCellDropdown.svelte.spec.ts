import { render, screen, within, waitFor } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { describe, it, expect, vi } from 'vitest';
import ItemCellDropdown from './ItemCellDropdown.svelte';

describe('ItemCellDropdown', () => {
  it('invokes onAdd with the selected item id', async () => {
    const onAdd = vi.fn().mockResolvedValue(true);
    const onChange = vi.fn();

    render(ItemCellDropdown, {
      props: {
        items: [
          { id: '1', name: 'Alpha', available: true },
          { id: '2', name: 'Bravo', available: true }
        ],
        onAdd,
        onChange
      }
    });

    // Open the menu (trigger behavior is delegated via the snippet)
    const user = userEvent.setup();
    const trigger = screen.getByTitle('Add item');
    await user.click(trigger);

    // Wait for the popup to mount
    const menu = await screen.findByRole('menu');

    // Click the desired item
    const item = within(menu).getByRole('menuitem', { name: 'Bravo' });
    await user.click(item);

    await waitFor(() => {
      expect(onAdd).toHaveBeenCalledTimes(1);
      expect(onAdd).toHaveBeenCalledWith('2');
    });
    expect(onChange).toHaveBeenCalledTimes(1);
  });
});
