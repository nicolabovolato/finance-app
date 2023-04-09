<script lang="ts">
	import { fade } from 'svelte/transition';
	import { createQuery } from '@tanstack/svelte-query';
	import {
		Listbox,
		ListboxButton,
		ListboxOption,
		ListboxOptions,
	} from '@rgossiaux/svelte-headlessui';
	import Chart from 'svelte-frappe-charts';
	import {
		currencies,
		getExchangeRate,
		getProfile,
		type Account,
		type Currency,
		type Profile,
	} from '$lib/services/api';
	import { getCurrencyFormatter } from '$lib/services/utils';
	import Error from '$lib/components/Error.svelte';
	import Icon from '$lib/components/atoms/Icon.svelte';

	let preferredCurrency: Currency = 'USD';
	let chartRef;

	$: balanceFormatter = getCurrencyFormatter(preferredCurrency);

	$: profileQuery = createQuery<Profile, Error>({
		queryKey: ['profile'],
		queryFn: async () => await getProfile(),
	});

	type Accounts = (Omit<Account, 'balance'> & { balance: number })[];

	$: accountsInPreferredCurrencyQuery = createQuery<Accounts, Error>({
		enabled: $profileQuery.status == 'success',
		queryKey: ['accounts-in-preferred-currency'],
		queryFn: async () =>
			await Promise.all(
				$profileQuery.data!.accounts.map(async (account) => ({
					...account,
					currency: preferredCurrency,
					balance:
						Number(account.balance) * (await getExchangeRate(account.currency, preferredCurrency)),
				})),
			),
	});
</script>

<div class="flex items-center justify-between">
	<h1 class="text-3xl font-bold">Overview</h1>
	<Listbox
		id="category"
		value={preferredCurrency}
		on:change={(e) => (preferredCurrency = e.detail)}
		on:blur={(e) => (preferredCurrency = e.detail)}
		let:open
	>
		<ListboxButton
			class="relative rounded border py-2 pl-3 pr-10 outline outline-2 outline-transparent transition-[outline] focus-within:outline-green-500"
		>
			<span class="absolute inset-y-0 right-0 flex items-center pr-2 text-gray-400">
				<Icon type="chevron-up-down" class="h-6 w-6" />
			</span>
			<span>
				{preferredCurrency}
			</span>
		</ListboxButton>
		{#if open}
			<div transition:fade={{ duration: 150 }}>
				<ListboxOptions class="absolute z-10 max-h-52 rounded border bg-white py-1" static>
					{#each currencies as currency}
						<ListboxOption
							value={currency}
							class={({ active }) => `relative py-2 transition ${active ? 'bg-gray-100' : ''}`}
							let:selected
						>
							{#if selected}
								<span class="absolute inset-y-0 left-0 flex items-center pl-2 text-gray-400">
									<Icon type="check" class="h-6 w-6" />
								</span>
							{/if}
							<span class="pl-10 pr-3" class:font-bold={selected}>{currency}</span>
						</ListboxOption>
					{/each}
				</ListboxOptions>
			</div>
		{/if}
	</Listbox>
</div>
{#key $profileQuery.status || $accountsInPreferredCurrencyQuery.status}
	<div in:fade={{ duration: 150 }} class="mt-3">
		{#if $profileQuery.status == 'error' || $accountsInPreferredCurrencyQuery.status == 'error'}
			<Error />
		{:else if $accountsInPreferredCurrencyQuery.status == 'loading'}
			<div class="mt-3 h-10 w-72 animate-pulse rounded bg-gray-200" />
			<div class="mx-auto mt-3 h-48 w-48 animate-pulse rounded-full bg-gray-200" />
		{:else if $accountsInPreferredCurrencyQuery.status == 'success'}
			{@const total = $accountsInPreferredCurrencyQuery.data.reduce(
				(acc, curr) => acc + Number(curr.balance),
				0,
			)}
			<h2 class="mt-3 flex flex-col items-baseline  lg:flex-row">
				<span
					class="text-3xl font-bold lg:text-4xl"
					class:text-red-600={total < 0}
					class:text-green-600={total > 0}
					class:text-gray-500={total == 0}
				>
					{balanceFormatter.format(total)}
					{preferredCurrency}
				</span>
				<span class="order-first text-xs text-gray-500 lg:order-none lg:ml-2">
					Approximate assets estimate
				</span>
			</h2>
			<Chart
				data={{
					labels: $accountsInPreferredCurrencyQuery.data
						.filter((account) => account.balance > 0)
						.map((account) => account.name),
					datasets: [
						{
							name: 'accounts',
							values: $accountsInPreferredCurrencyQuery.data
								.filter((account) => account.balance > 0)
								.map((account) => account.balance),
						},
					],
				}}
				type="donut"
				maxSlices={4}
				colors={['#14532d', '#15803d', '#22c55e', '#86efac']}
				bind:this={chartRef}
			/>
		{/if}
	</div>
{/key}

<style>
	:global(svg.frappe-chart) {
		overflow: visible;
	}
</style>
