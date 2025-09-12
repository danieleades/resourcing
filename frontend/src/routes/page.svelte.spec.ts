import { describe, expect, it } from 'vitest';
import Page from './+page.svelte';
import { render, screen } from '@testing-library/svelte';

describe('/+page.svelte', () => {
	it('should render h1 (async version)', async () => {
		render(Page);

		// Wait up to 1000ms (default) for the element to appear
		const heading = await screen.findByRole('heading', { level: 1 });
		expect(heading).toBeDefined();
	});
});
