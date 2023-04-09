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
	import { createMovement, categories, type Category } from '$lib/services/api';
	import Button from './atoms/Button.svelte';
	import Input from './atoms/Input.svelte';
	import Icon from './atoms/Icon.svelte';

	export let isOpen = false;
	export let accountId: string;
	export let errorMessage: string | undefined = undefined;

	const queryClient = useQueryClient();

	$: movementMutation = createMutation({
		mutationFn: async ({
			title,
			category,
			amount,
		}: {
			title: string;
			category: Category;
			amount: string;
		}) => await createMovement(accountId, { title, category, amount }),
		onSuccess: () => {
			queryClient.invalidateQueries(['movements', accountId]);
			queryClient.invalidateQueries(['accounts', accountId]);
			isOpen = false;
		},
		onError: (err: number) => {
			switch (err) {
				default:
					errorMessage = 'An error has occurred, please try again later';
			}
			setTimeout(() => (errorMessage = undefined), 3000);
		},
	});

	const { form, errors, state, handleChange, updateValidateField, handleSubmit } = createForm<{
		title: string;
		category: Category;
		amount: string;
	}>({
		initialValues: {
			title: '',
			category: 'GENERIC',
			amount: '',
		},
		validationSchema: yup.object().shape({
			title: yup
				.string()
				.min(3, 'Name must be longer than 3 characters')
				.max(64, 'Name must not be longer than 64 characters')
				.required('Name required'),
			amount: yup
				.string()
				.required('Amount is required')
				.matches(/^[+-]?\d+(?:[.,]\d{1,2})?$/, 'Amount must be a valid number'),
			category: yup.string(),
		}),
		onSubmit: async (values) => {
			await $movementMutation.mutateAsync(values);
		},
	});
</script>

<Dialog open={isOpen} on:close={() => (isOpen = false)}>
	<div transition:fade={{ duration: 150 }}>
		<DialogOverlay class="fixed top-0 z-50 h-screen w-screen bg-gray-400 opacity-50" />
		<div
			class="fixed top-1/2 left-1/2 z-50 translate-x-[-50%] translate-y-[-50%] rounded bg-white px-5 py-4"
		>
			<DialogTitle class="text-2xl font-bold">New Movement</DialogTitle>
			<div class="mt-3">
				<Input
					id="title"
					type="text"
					bind:value={$form.title}
					on:input={handleChange}
					on:blur={handleChange}
					placeholder="Title"
					class="mb-2"
				/>
				{#if $errors.title}
					<div transition:fade={{ duration: 150 }} class="text-sm text-red-600">
						{$errors.title}
					</div>
				{/if}
			</div>
			<div class="mt-3 flex">
				<Input
					id="amount"
					type="text"
					bind:value={$form.amount}
					on:input={handleChange}
					on:blur={handleChange}
					placeholder="Amount"
					class="mr-2"
				/>
				<Listbox
					id="category"
					value={$form.category}
					on:change={(e) => updateValidateField('category', e.detail)}
					on:blur={(e) => updateValidateField('category', e.detail)}
					let:open
				>
					<ListboxButton
						class="relative rounded border py-2 pl-3 pr-10 outline outline-2 outline-transparent transition-[outline] focus-within:outline-green-500"
					>
						<span class="absolute inset-y-0 right-0 flex items-center pr-2 text-gray-400">
							<Icon type="chevron-up-down" class="h-6 w-6" />
						</span>
						<span>
							{$form.category}
						</span>
					</ListboxButton>
					{#if open}
						<div transition:fade={{ duration: 150 }}>
							<ListboxOptions class="absolute max-h-52 rounded border bg-white py-1" static>
								{#each categories as category}
									<ListboxOption
										value={category}
										class={({ active }) =>
											`relative py-2 transition ${active ? 'bg-gray-100' : ''}`}
										let:selected
									>
										{#if selected}
											<span class="absolute inset-y-0 left-0 flex items-center pl-2 text-gray-400">
												<Icon type="check" class="h-6 w-6" />
											</span>
										{/if}
										<span class="pl-10 pr-3" class:font-bold={selected}>{category}</span>
									</ListboxOption>
								{/each}
							</ListboxOptions>
						</div>
					{/if}
				</Listbox>
			</div>
			{#if $errors.amount}
				<div transition:fade={{ duration: 150 }} class="text-sm text-red-600">{$errors.amount}</div>
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
					disabled={$state.isSubmitting}
					variant="secondary"
					outline
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
