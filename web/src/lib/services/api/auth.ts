import { apiFetch, withJson } from '.';
import { accessTokenStore } from '../stores';

export type LoginResponse = {
	access_token: string;
};

export const getOtp = async (email: string) =>
	await apiFetch(
		`/auth/otp`,
		withJson(
			{
				method: 'POST',
			},
			{ email },
		),
	);

export const login = async (email: string, otp: string) => {
	const response = await apiFetch<LoginResponse>(
		`/auth/login`,
		withJson(
			{
				method: 'POST',
			},
			{ email, otp },
		),
	);
	accessTokenStore.set(response.access_token);
	return response;
};

export const signup = async (email: string, otp: string) =>
	await apiFetch(
		`/auth/signup`,
		withJson(
			{
				method: 'POST',
			},
			{ email, otp },
		),
	);
