import { describe, expect, it } from 'vitest';
import Page from './+page.svelte';
import {render, screen} from '@testing-library/svelte'

describe('/+page.svelte', () => {
	it('should render h1', async () => {
		render(Page);

		const heading = screen.getByRole('heading', { level: 1 });
		await expect.element(heading).toBeInTheDocument();
	});
});
