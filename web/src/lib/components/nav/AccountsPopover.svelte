<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { fade } from 'svelte/transition';
	import { Popover, PopoverButton, PopoverPanel } from '@rgossiaux/svelte-headlessui';
	import { page } from '$app/stores';
	import { truncate } from '$lib/services/utils';
	import type { Account } from '$lib/services/api';
	import Button from '../atoms/Button.svelte';
	import Icon from '../atoms/Icon.svelte';

	const dispatch = createEventDispatcher();
	export let accounts: Account[];
	export let loading: boolean;
</script>

<Popover class="mt-2 sm:hidden" let:open>
	<PopoverButton
		class="rounded py-1 px-2 {$page.url.pathname.startsWith('/accounts') ? 'bg-gray-100' : ''}"
	>
		<Icon type="banknotes" class="inline-block h-8 w-8 sm:mr-2 sm:h-6 sm:w-6" />
		<span class="hidden align-middle sm:inline">Accounts</span>
	</PopoverButton>

	{#if open}
		<div transition:fade={{ duration: 150 }}>
			<PopoverPanel
				class="absolute left-20 top-20 z-20 max-h-[50vh] overflow-y-scroll rounded bg-gray-200 p-2"
				static
			>
				<Button
					class="mb-1"
					variant="secondary"
					size="sm"
					outline
					disabled={loading}
					on:click={() => dispatch('new-account-clicked')}
				>
					New Account
				</Button>
				{#if loading}
					{#each Array(3) as _}
						<div class="mt-1 h-8 animate-pulse rounded bg-gray-300 py-1 px-2" />
					{/each}
				{:else}
					{#each accounts as account}
						<div
							class="rounded py-1 px-2"
							class:bg-gray-100={$page.url.pathname == `/accounts/${account.id}`}
						>
							<a href="/accounts/{account.id}">{truncate(account.name, 32)}</a>
						</div>
					{/each}
				{/if}
			</PopoverPanel>
		</div>
	{/if}
</Popover>
