<script lang="ts">
	import Auth from './Auth.svelte';
	import { accessTokenStore } from '$lib/services/stores';

	let loggedIn: boolean;

	accessTokenStore.subscribe((value) => (loggedIn = !!value));
</script>

{#if !loggedIn}
	<Auth />
{:else}
	<slot />
{/if}
