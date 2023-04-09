import { apiFetch, type Currency } from '.';

const currencyApiUrl = 'https://cdn.jsdelivr.net/gh/fawazahmed0/currency-api@1/latest';

export const getExchangeRate = async (from: Currency, to: Currency) => {
	const response = await apiFetch<Record<string, unknown>>(
		`/currencies/${from.toLowerCase()}/${to.toLowerCase()}.json`,
		{
			method: 'GET',
		},
		currencyApiUrl,
	);
	return response[to.toLowerCase()] as number;
};
