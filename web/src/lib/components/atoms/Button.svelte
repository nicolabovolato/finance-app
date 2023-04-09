<script lang="ts">
	type Variant = 'primary' | 'secondary' | 'danger';
	type Size = 'sm' | 'md';

	export let variant: Variant;
	export let outline = false;
	export let size: Size = 'md';
	export let disabled = false;
</script>

<button
	on:click
	{disabled}
	class="rounded hover:text-white {$$props.class} transition"
	class:px-5={size == 'md'}
	class:py-2={size == 'md'}
	class:px-3={size == 'sm'}
	class:py-1={size == 'sm'}
	class:border={outline == true}
	class:border-gray-500={outline == true}
	class:disabled:border-gray-400={outline == true}
	class:hover:disabled:border-gray-400={outline == true}
	class:text-gray-500={outline == true}
	class:disabled:text-gray-400={outline == true}
	class:hover:disabled:text-gray-400={outline == true}
	class:hover:disabled:bg-inherit={outline == true}
	class:text-white={outline == false}
	class:disabled:bg-gray-400={outline == false}
	class:hover:disabled:bg-gray-400={outline == false}
	class:bg-green-500={variant == 'primary' && outline == false}
	class:hover:bg-green-600={variant == 'primary'}
	class:hover:border-green-600={variant == 'primary' && outline == true}
	class:bg-gray-500={variant == 'secondary' && outline == false}
	class:hover:bg-gray-600={variant == 'secondary'}
	class:hover:border-gray-600={variant == 'secondary' && outline == true}
	class:bg-red-500={variant == 'danger' && outline == false}
	class:hover:bg-red-600={variant == 'danger'}
	class:hover:border-red-600={variant == 'danger' && outline == true}
>
	<slot />
</button>
