import { accessTokenStore } from '../stores';

const apiUrl = import.meta.env.VITE_API_URL.replace(/\/\s*$/, '');

export const apiFetch = async <T = void>(
	url: string,
	options: RequestInit | undefined,
	baseUrl = apiUrl,
): Promise<T> => {
	const response = await fetch(`${baseUrl}${url}`, {
		mode: 'cors',
		...options,
	});

	if (!response.ok) throw response.status;

	if (response.headers.get('content-type')?.includes('application/json')) {
		return (await response.json()) as T;
	}

	return undefined as T;
};

export const apiFetchProtected = async <T = void>(
	url: string,
	options: RequestInit | undefined,
): Promise<T> => {
	try {
		return await apiFetch<T>(url, withBearer(options));
	} catch (err) {
		if (err == 401) accessTokenStore.set(null);
		throw err;
	}
};

export const withJson = (options: RequestInit | undefined, data?: unknown) => ({
	...options,
	headers: {
		...options?.headers,
		'content-type': 'application/json',
	},
	body: JSON.stringify(data),
});

export const withBearer = (options: RequestInit | undefined) => ({
	...options,
	headers: {
		...options?.headers,
		authorization: `Bearer ${localStorage.getItem('access_token')}`,
	},
});

export * from './auth';
export * from './profile';
export * from './currencyExchange';
