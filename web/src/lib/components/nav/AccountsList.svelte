<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { fade } from 'svelte/transition';
	import { page } from '$app/stores';
	import { truncate } from '$lib/services/utils';
	import type { Account } from '$lib/services/api';
	import Icon from '../atoms/Icon.svelte';

	const dispatch = createEventDispatcher();

	export let accounts: Account[];
	export let loading: boolean;
</script>

<div class="sm mt-2 hidden sm:inline">
	<div class="py-1 px-2">
		<Icon type="banknotes" class="inline-block h-8 w-8 sm:mr-2 sm:h-6 sm:w-6" />
		<span class="hidden align-middle sm:inline">Accounts </span>
		<button class="ml-2" on:click={() => dispatch('new-account-clicked')}>
			<Icon type="plus-circle" class="inline-block h-6 w-6" />
		</button>
	</div>
	{#key loading}
		<div transition:fade={{ duration: 150 }}>
			{#if loading}
				{#each Array(3) as _}
					<div class="mt-1 ml-8 h-6 animate-pulse rounded bg-gray-300 py-1 px-2" />
				{/each}
			{:else}
				{#each accounts as account}
					<div
						class="mt-1 rounded py-1 px-2"
						class:bg-gray-100={$page.url.pathname == `/accounts/${account.id}`}
					>
						<a class="ml-8" href="/accounts/{account.id}">{truncate(account.name, 24)}</a>
					</div>
				{/each}
			{/if}
		</div>
	{/key}
</div>
