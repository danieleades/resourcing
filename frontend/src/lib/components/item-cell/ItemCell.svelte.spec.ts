import { render, fireEvent, waitFor } from '@testing-library/svelte';
import { describe, it, expect, vi, afterEach } from 'vitest';
import { GraphQLClient } from 'graphql-request';
import ItemCell from './ItemCell.svelte';

afterEach(() => {
  vi.restoreAllMocks();
  vi.unstubAllGlobals();
});

describe('ItemCell (local state)', () => {
  it('removes an item locally and calls onChange after unassign', async () => {
    const requestMock = vi
      .spyOn(GraphQLClient.prototype, 'request')
      .mockResolvedValue({ unassign: true });

    const onChange = vi.fn();
    const { getAllByTitle, queryByText } = render(ItemCell, {
      props: {
        projectId: 'project-1',
        month: '2025-06',
        assignedItems: [
          { id: 'alice', name: 'Alice' },
          { id: 'bob', name: 'Bob' }
        ],
        dropdownItems: [],
        onChange
      }
    });

    const removeButtons = getAllByTitle('Remove item');
    await fireEvent.click(removeButtons[0]);

    expect(queryByText('Alice')).not.toBeInTheDocument();
    expect(requestMock).toHaveBeenCalledTimes(1);
    await waitFor(() => expect(onChange).toHaveBeenCalledTimes(1));
  });
});
