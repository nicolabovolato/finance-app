import { apiFetchProtected, withJson } from '.';

export const currencies = ['USD', 'EUR'] as const;
export type Currency = (typeof currencies)[number];

export const categories = ['GENERIC', 'BILLS', 'INCOME', 'SHOPPING', 'INSURANCE'] as const;
export type Category = (typeof categories)[number];

export type Movement = {
	id: string;
	title: string;
	category: Category;
	timestamp: string;
	amount: string;
};

export type Account = {
	id: string;
	name: string;
	balance: string;
	currency: Currency;
};

export type Profile = {
	accounts: Account[];
};

export const getProfile = async () =>
	await apiFetchProtected<Profile>(`/profile`, {
		method: 'GET',
	});

export const getAccount = async (account_id: Account['id']) =>
	await apiFetchProtected<Account>(`/profile/accounts/${account_id}`, {
		method: 'GET',
	});

export const getMovements = async (account_id: Account['id']) =>
	await apiFetchProtected<Movement[]>(`/profile/accounts/${account_id}/movements`, {
		method: 'GET',
	});

export const createAccount = async (account: Pick<Account, 'name' | 'currency'>) =>
	await apiFetchProtected<Account>(
		`/profile/accounts`,
		withJson(
			{
				method: 'POST',
			},
			account,
		),
	);

export const createMovement = async (
	accountId: Account['id'],
	movement: Pick<Movement, 'title' | 'category' | 'amount'>,
) =>
	await apiFetchProtected<Account>(
		`/profile/accounts/${accountId}/movements`,
		withJson(
			{
				method: 'POST',
			},
			movement,
		),
	);
