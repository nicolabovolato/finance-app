<script lang="ts">
	import { fade, fly } from 'svelte/transition';
	import { createQuery } from '@tanstack/svelte-query';
	import type { PageData } from './$types';
	import { getAccount, getMovements, type Account, type Movement } from '$lib/services/api';
	import { getCurrencyFormatter, getDateFormatter } from '$lib/services/utils';
	import NewMovement from '$lib/components/NewMovement.svelte';
	import Button from '$lib/components/atoms/Button.svelte';
	import Error from '$lib/components/Error.svelte';

	export let data: PageData;
	let isMovementDialogOpen = false;

	$: accountQuery = createQuery<Account, Error>({
		queryKey: ['accounts', data.account_id],
		queryFn: async () => await getAccount(data.account_id),
	});

	$: movementsQuery = createQuery<Movement[], Error>({
		queryKey: ['movements', data.account_id],
		queryFn: async () => await getMovements(data.account_id),
		enabled: $accountQuery.status == 'success',
	});

	$: movementAmountFormatter = getCurrencyFormatter(
		$accountQuery.data?.currency || 'USD',
		'exceptZero',
	);
	$: balanceFormatter = getCurrencyFormatter($accountQuery.data?.currency || 'USD');
	$: dateFormatter = getDateFormatter();
</script>

{#key $accountQuery.status}
	<div in:fade={{ duration: 150 }}>
		{#if $accountQuery.status == 'loading'}
			<div class="h-8 w-64 animate-pulse rounded bg-gray-200" />
			<div class="mt-3 h-10 w-72 animate-pulse rounded bg-gray-200" />
		{:else if $accountQuery.status == 'error'}
			<Error />
		{:else}
			<h1 class="text-3xl font-bold">
				{$accountQuery.data.name}
			</h1>
			<h2
				class="mt-3 text-4xl font-bold"
				class:text-red-600={Number($accountQuery.data.balance) < 0}
				class:text-green-600={Number($accountQuery.data.balance) > 0}
				class:text-gray-500={Number($accountQuery.data.balance) == 0}
			>
				{balanceFormatter.format(Number($accountQuery.data.balance))}
				{$accountQuery.data.currency}
			</h2>

			<div class="mt-4 flex items-baseline justify-between">
				<h3 class="text-xl font-bold">Movements</h3>
				<Button size="sm" variant="primary" outline on:click={() => (isMovementDialogOpen = true)}>
					New Movement
				</Button>
			</div>

			<div class="mt-5">
				{#if $movementsQuery.status == 'loading'}
					<div out:fly={{ y: -500, duration: 250 }}>
						{#each Array(3) as _}
							<div class="my-2 animate-pulse rounded p-5 shadow">
								<div class="flex justify-between">
									<div class="h-6 w-48 rounded bg-gray-200" />
									<div class="ml-4 h-6 w-16 rounded bg-gray-200" />
								</div>
								<div class="mt-1 flex justify-between">
									<div class="h-4 w-20 rounded bg-gray-200" />
									<div class="ml-4 h-4 w-12 rounded bg-gray-200" />
								</div>
							</div>
						{/each}
					</div>
				{:else if $movementsQuery.status == 'error'}
					<div in:fade={{ duration: 150 }}>
						<Error />
					</div>
				{:else}
					<div in:fly={{ y: 500, delay: 250, duration: 250 }}>
						{#each $movementsQuery.data as movement}
							<div class="my-2 rounded p-5 shadow">
								<div class="flex justify-between">
									<div>{movement.title}</div>
									<div
										class:text-red-600={Number(movement.amount) < 0}
										class:text-green-600={Number(movement.amount) > 0}
									>
										{movementAmountFormatter.format(Number(movement.amount))}
									</div>
								</div>
								<div class="flex justify-between">
									<div class="text-sm text-gray-500">{movement.category}</div>
									<div class="text-sm text-gray-500">
										{dateFormatter.format(new Date(movement.timestamp))}
									</div>
								</div>
							</div>
						{/each}
					</div>
				{/if}
			</div>
		{/if}
	</div>
{/key}
<NewMovement bind:isOpen={isMovementDialogOpen} accountId={data.account_id} />
