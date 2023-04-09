<script lang="ts">
	import { createMutation, createQuery } from '@tanstack/svelte-query';
	import { getProfile, type Profile } from '$lib/services/api';
	import { accessTokenStore } from '$lib/services/stores';
	import { page } from '$app/stores';
	import NewAccount from '../NewAccount.svelte';
	import Icon from '../atoms/Icon.svelte';
	import AccountsList from './AccountsList.svelte';
	import AccountsPopover from './AccountsPopover.svelte';

	$: profileQuery = createQuery<Profile, Error>({
		queryKey: ['profile'],
		queryFn: async () => await getProfile(),
	});

	$: logoutMutation = createMutation({
		mutationFn: async () => {
			accessTokenStore.set(null);
		},
	});

	let isAccountDialogOpen = false;
</script>

<div class="grid-template grid h-full">
	<div class="py-1 px-2 text-lg font-bold text-green-600">
		<Icon type="chart-bar-fill" class="inline-block h-8 w-8 sm:mr-2 sm:h-6 sm:w-6" />
		<span class="hidden align-middle sm:inline">Personal finance app</span>
	</div>
	<div class="mt-2 rounded py-1 px-2" class:bg-gray-100={$page.url.pathname == '/'}>
		<a href="/">
			<Icon type="home" class="inline-block h-8 w-8 sm:mr-2 sm:h-6 sm:w-6" />
			<span class="hidden align-middle sm:inline">Overview</span>
		</a>
	</div>

	<AccountsList
		loading={$profileQuery.isLoading}
		accounts={$profileQuery.data?.accounts ?? []}
		on:new-account-clicked={() => (isAccountDialogOpen = true)}
	/>
	<AccountsPopover
		loading={$profileQuery.isLoading}
		accounts={$profileQuery.data?.accounts ?? []}
		on:new-account-clicked={() => (isAccountDialogOpen = true)}
	/>

	<div class="mt-2 inline-block">
		<button class="py-1 px-2" on:click={() => $logoutMutation.mutate()}>
			<Icon type="arrow-left-on-rectangle" class="inline-block h-8 w-8 sm:mr-2 sm:h-6 sm:w-6" />
			<span class="hidden align-middle sm:inline">Logout</span>
		</button>
	</div>
</div>
<NewAccount bind:isOpen={isAccountDialogOpen} />

<style>
	.grid-template {
		grid-template-rows: auto auto 1fr auto;
	}
</style>
