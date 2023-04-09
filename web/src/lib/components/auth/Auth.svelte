<script lang="ts">
	import { fade } from 'svelte/transition';
	import { createMutation } from '@tanstack/svelte-query';
	import { Tab, TabGroup, TabList } from '@rgossiaux/svelte-headlessui';
	import { createForm } from 'svelte-forms-lib';
	import * as yup from 'yup';
	import { login, signup, getOtp } from '$lib/services/api';

	import Button from '../atoms/Button.svelte';
	import Icon from '../atoms/Icon.svelte';
	import Input from '../atoms/Input.svelte';

	let formType: 'login' | 'signup' = 'login';
	let infoMessage: string | undefined = undefined;
	let errorMessage: string | undefined = undefined;

	const loginMutation = createMutation({
		mutationFn: async ({ email, otp }: { email: string; otp: string }) => await login(email, otp),
		onError: (err: number) => {
			switch (err) {
				case 401:
					errorMessage = 'Wrong OTP';
					break;
				case 404:
					errorMessage = 'User not found';
					break;
				default:
					errorMessage = 'An error has occurred, please try again later';
			}
			setTimeout(() => (errorMessage = undefined), 3000);
		},
	});

	const signupMutation = createMutation({
		mutationFn: async ({ email, otp }: { email: string; otp: string }) => await signup(email, otp),
		onError: (err: number) => {
			switch (err) {
				case 401:
					errorMessage = 'Wrong OTP';
					break;
				default:
					errorMessage = 'An error has occurred, please try again later';
			}
			setTimeout(() => (errorMessage = undefined), 3000);
		},
		onSuccess: () => {
			infoMessage = 'Successfully signed up!';
			setTimeout(() => (infoMessage = undefined), 3000);
		},
	});

	const otpMutation = createMutation({
		mutationFn: async (email: string) => await getOtp(email),
		onError: (err: number) => {
			switch (err) {
				default:
					errorMessage = 'An error has occurred, please try again later';
			}
			setTimeout(() => (errorMessage = undefined), 3000);
		},
		onSuccess: () => {
			infoMessage = 'OTP Sent!';
			setTimeout(() => (infoMessage = undefined), 3000);
		},
	});

	const { form, errors, state, handleChange, handleSubmit } = createForm({
		initialValues: {
			email: '',
			otp: '',
		},
		validationSchema: yup.object().shape({
			email: yup.string().email('Invalid email').required('Email required'),
			otp: yup
				.string()
				.required('OTP required')
				.matches(/^\d{6}$/, 'OTP must be 6 numbers'),
		}),
		onSubmit: async (values) => {
			if (formType == 'login') {
				await $loginMutation.mutateAsync(values);
			} else {
				await $signupMutation.mutateAsync(values);
			}
		},
	});
</script>

<div class="rounded bg-white p-6 shadow-lg">
	<h1 class="mb-5 text-center text-3xl font-bold text-green-600">
		<Icon type="chart-bar-fill" class="mr-2 inline-block h-10 w-10 align-bottom" />
		<span> Personal Finance App </span>
	</h1>
	<TabGroup on:change={(e) => (formType = e.detail == 0 ? 'login' : 'signup')}>
		<TabList class="flex rounded bg-gray-100 p-1 transition">
			<Tab
				class={({ selected }) =>
					`${selected ? 'bg-white' : 'bg-transparent'} flex-grow rounded p-2`}
				><div>Login</div>
			</Tab>
			<Tab
				class={({ selected }) =>
					`${selected ? 'bg-white' : 'bg-transparent'} flex-grow rounded p-2`}
				><div>Signup</div>
			</Tab>
		</TabList>
	</TabGroup>
	<div class="pt-5">
		<div class="relative">
			<span class="absolute inset-y-0 left-0 flex items-center pl-2 text-gray-400">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					stroke-width="1.5"
					stroke="currentColor"
					class="h-6 w-6"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						d="M21.75 6.75v10.5a2.25 2.25 0 01-2.25 2.25h-15a2.25 2.25 0 01-2.25-2.25V6.75m19.5 0A2.25 2.25 0 0019.5 4.5h-15a2.25 2.25 0 00-2.25 2.25m19.5 0v.243a2.25 2.25 0 01-1.07 1.916l-7.5 4.615a2.25 2.25 0 01-2.36 0L3.32 8.91a2.25 2.25 0 01-1.07-1.916V6.75"
					/>
				</svg>
			</span>
			<Input
				id="email"
				type="email"
				bind:value={$form.email}
				on:input={handleChange}
				on:blur={handleChange}
				placeholder="Email"
				class="py-2 pl-10 pr-3"
			/>
		</div>
		{#if $errors.email}
			<div transition:fade={{ duration: 150 }} class="text-sm text-red-600">{$errors.email}</div>
		{/if}

		<div class="relative mt-2">
			<span class="absolute inset-y-0 left-0 flex items-center pl-2 text-gray-400">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					stroke-width="1.5"
					stroke="currentColor"
					class="h-6 w-6"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						d="M15.75 5.25a3 3 0 013 3m3 0a6 6 0 01-7.029 5.912c-.563-.097-1.159.026-1.563.43L10.5 17.25H8.25v2.25H6v2.25H2.25v-2.818c0-.597.237-1.17.659-1.591l6.499-6.499c.404-.404.527-1 .43-1.563A6 6 0 1121.75 8.25z"
					/>
				</svg>
			</span>
			<Input
				id="otp"
				type="text"
				bind:value={$form.otp}
				on:input={handleChange}
				on:blur={handleChange}
				placeholder="OTP"
				class="py-2 pl-10 pr-3"
			/>
		</div>
		{#if $errors.otp}
			<div transition:fade={{ duration: 150 }} class="text-sm text-red-600">{$errors.otp}</div>
		{/if}
		<div class="mt-5 flex justify-between align-middle">
			<Button
				variant="primary"
				outline
				disabled={!$state.touched.email || $errors.email.length > 0 || $state.isSubmitting}
				on:click={() => $otpMutation.mutate($form.email)}
			>
				Send OTP
			</Button>
			<Button
				variant="primary"
				disabled={!$state.isValid || $state.isSubmitting}
				on:click={handleSubmit}
			>
				{formType == 'login' ? 'Login' : 'Signup'}
			</Button>
		</div>
	</div>
	{#if infoMessage}
		<div transition:fade={{ duration: 150 }} class="mt-3 text-center text-green-600">
			{infoMessage}
		</div>
	{/if}
	{#if errorMessage}
		<div transition:fade={{ duration: 150 }} class="mt-3 text-center text-red-600">
			{errorMessage}
		</div>
	{/if}
</div>
