<script lang="ts">
	import { fade } from 'svelte/transition';
	import {
		Dialog,
		DialogOverlay,
		DialogTitle,
		Listbox,
		ListboxButton,
		ListboxOption,
		ListboxOptions,
	} from '@rgossiaux/svelte-headlessui';
	import { createMutation, useQueryClient } from '@tanstack/svelte-query';
	import { createForm } from 'svelte-forms-lib';
	import * as yup from 'yup';
	import { createAccount, currencies, type Currency } from '$lib/services/api';
	import Button from './atoms/Button.svelte';
	import Input from './atoms/Input.svelte';
	import Icon from './atoms/Icon.svelte';

	export let isOpen = false;

	const queryClient = useQueryClient();

	let errorMessage: string | undefined = undefined;

	const accountMutation = createMutation({
		mutationFn: async ({ name, currency }: { name: string; currency: Currency }) =>
			await createAccount({ name, currency }),
		onError: (err: number) => {
			switch (err) {
				default:
					errorMessage = 'An error has occurred, please try again later';
			}
			setTimeout(() => (errorMessage = undefined), 3000);
		},
		onSuccess: () => {
			queryClient.invalidateQueries(['profile']);
			isOpen = false;
		},
	});

	const { form, errors, state, handleChange, updateValidateField, handleSubmit } = createForm<{
		name: string;
		currency: Currency;
	}>({
		initialValues: {
			name: '',
			currency: 'USD',
		},
		validationSchema: yup.object().shape({
			name: yup
				.string()
				.min(3, 'Name must be longer than 3 characters')
				.max(64, 'Name must not be longer than 64 characters')
				.required('Name required'),
			currency: yup.string(),
		}),
		onSubmit: async (values) => {
			await $accountMutation.mutateAsync(values);
		},
	});
</script>

<Dialog open={isOpen} on:close={() => (isOpen = false)}>
	<div transition:fade={{ duration: 150 }}>
		<DialogOverlay class="fixed top-0 z-50 h-screen w-screen bg-gray-400 opacity-50" />
		<div
			class="fixed top-1/2 left-1/2 z-50 translate-x-[-50%] translate-y-[-50%] rounded bg-white px-5 py-4"
		>
			<DialogTitle class="text-2xl font-bold">New Account</DialogTitle>
			<div class="mt-3 flex">
				<Input
					id="name"
					type="text"
					bind:value={$form.name}
					on:input={handleChange}
					on:blur={handleChange}
					placeholder="Account name"
					class="mr-2"
				/>
				<Listbox
					id="currency"
					value={$form.currency}
					on:change={(e) => updateValidateField('currency', e.detail)}
					on:blur={(e) => updateValidateField('currency', e.detail)}
					let:open
				>
					<ListboxButton
						class="relative rounded border py-2 pl-3 pr-10 outline outline-2 outline-transparent transition-[outline] focus-within:outline-green-500"
					>
						<span class="absolute inset-y-0 right-0 flex items-center pr-2 text-gray-400">
							<Icon type="chevron-up-down" class="h-6 w-6" />
						</span>
						<span>
							{$form.currency}
						</span>
					</ListboxButton>
					{#if open}
						<div transition:fade={{ duration: 150 }}>
							<ListboxOptions class="absolute max-h-52 rounded border bg-white py-1" static>
								{#each currencies as currency}
									<ListboxOption
										value={currency}
										class={({ active }) =>
											`relative py-2 transition ${active ? 'bg-gray-100' : ''}`}
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
			{#if $errors.name}
				<div transition:fade={{ duration: 150 }} class="text-sm text-red-600">{$errors.name}</div>
			{/if}
			<div class="mt-3">
				<Button
					variant="primary"
					disabled={!$state.isValid || $state.isSubmitting}
					on:click={handleSubmit}
				>
					Create
				</Button>
				<Button
					variant="secondary"
					outline
					disabled={$state.isSubmitting}
					on:click={() => (isOpen = false)}
				>
					Cancel
				</Button>
				{#if errorMessage}
					<div class="mt-3 text-center text-red-600" transition:fade={{ duration: 150 }}>
						{errorMessage}
					</div>
				{/if}
			</div>
		</div>
	</div>
</Dialog>
