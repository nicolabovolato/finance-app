import { writable } from 'svelte/store';

function createAccessTokenStore() {
	const { subscribe, set } = writable(localStorage.getItem('access_token'));

	return {
		subscribe,
		set: (value: string | null) => {
			if (typeof value == 'string') localStorage.setItem('access_token', value);
			else localStorage.removeItem('access_token');
			set(value);
		},
	};
}

export const accessTokenStore = createAccessTokenStore();
