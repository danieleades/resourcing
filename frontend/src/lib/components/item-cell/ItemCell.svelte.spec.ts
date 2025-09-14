import { render, screen } from '@testing-library/svelte';
import { describe, it, expect } from 'vitest';
import ItemCell from './ItemCell.svelte';

describe('ItemCell', () => {
	it('renders provided items', () => {
		render(ItemCell, { props: { items: ['Alpha', 'Bravo'] } });
		expect(screen.getByText('Alpha')).toBeDefined();
		expect(screen.getByText('Bravo')).toBeDefined();
		expect(screen.queryByText('-')).toBeNull();
	});

	it('shows placeholder when no items', () => {
		render(ItemCell, { props: { items: [] } });
		expect(screen.getByText('-')).toBeDefined();
	});
});
