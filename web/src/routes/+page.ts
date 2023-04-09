export const ssr = false;

import type { PageLoad } from './$types';

export const load = (() => {
	return {
		title: 'Overview',
	};
}) satisfies PageLoad;
