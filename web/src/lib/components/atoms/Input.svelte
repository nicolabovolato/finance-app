<script lang="ts">
	export let id: string;
	export let type: string;
	export let value: string;
	export let placeholder: string | undefined;
	export let disabled = false;

	const setType = (node: HTMLInputElement) => {
		node.type = type;
	};
</script>

<input
	{id}
	use:setType
	bind:value
	on:input
	on:change
	on:blur
	{placeholder}
	{disabled}
	class="rounded border py-2 px-3 outline-transparent transition-[outline] invalid:outline-red-500 focus:outline-green-500 {$$props.class}"
/>
