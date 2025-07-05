import { apiFetch, type Currency } from '.';

const currencyApiUrl = 'https://cdn.jsdelivr.net/npm/@fawazahmed0/currency-api@latest/v1';

export const getExchangeRate = async (from: Currency, to: Currency) => {
	const response = await apiFetch<Record<string, Record<string, unknown>>>(
		`/currencies/${from.toLowerCase()}.json`,
		{
			method: 'GET',
		},
		currencyApiUrl,
	);
	return response[from.toLowerCase()][to.toLowerCase()] as number;
};
