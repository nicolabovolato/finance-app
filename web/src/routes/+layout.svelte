<script lang="ts">
	import { page } from '$app/stores';
	import { QueryClientProvider, QueryClient } from '@tanstack/svelte-query';

	import 'src/app.css';
	import AuthProvider from '$lib/components/auth/AuthProvider.svelte';
	import Nav from '$lib/components/nav/Nav.svelte';

	const queryClient = new QueryClient();
</script>

<svelte:head>
	<title>{$page.data.title} | Personal Finance App</title>
</svelte:head>

<QueryClientProvider client={queryClient}>
	<div class="background-pattern grid h-screen w-screen items-center justify-center">
		<AuthProvider>
			<div
				class="w-screen rounded bg-white sm:w-[600px] md:w-[728px] lg:w-[984px] xl:w-[1240px] 2xl:w-[1496px]"
			>
				<div class="grid-template grid h-screen sm:h-[90vh]">
					<div
						class="max-h-screen overflow-auto rounded bg-gray-200 p-2 pt-5 sm:max-h-[90vh] md:p-5"
					>
						<Nav />
					</div>
					<div
						class="h-screen max-h-screen  min-h-screen overflow-y-auto p-5 sm:h-[90vh]  sm:max-h-[90vh] sm:min-h-[90vh]"
					>
						<slot />
					</div>
				</div>
			</div>
		</AuthProvider>
	</div>
</QueryClientProvider>

<style>
	.grid-template {
		grid-template-columns: auto 1fr;
	}
</style>
