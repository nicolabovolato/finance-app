export const ssr = false;

import type { PageLoad } from './$types';

export const load = (({ params }) => {
	return {
		account_id: params.account_id,
		title: 'Account Detail',
	};
}) satisfies PageLoad;
