<script lang="ts">
	import { cn } from '$lib/utils/cn';
	import { useMotionTemplate, useMotionValue, Motion } from 'svelte-motion';

	export let className: string | undefined = undefined;
	export let type: string = 'text';
	export let configs: string[] = [];

	let visible = false;

	let mouseX = useMotionValue(0);
	let mouseY = useMotionValue(0);

	function handleMouseMove({ currentTarget, clientX, clientY }: any) {
		let { left, top } = currentTarget.getBoundingClientRect();

		mouseX.set(clientX - left);
		mouseY.set(clientY - top);
	}
</script>

<Motion
	let:motion
	style={{
		background: visible
			? useMotionTemplate`
            radial-gradient(
              100px circle at ${mouseX}px ${mouseY}px, 
              var(--blue-500),
              transparent 80%
            )
          `
			: useMotionTemplate`
            radial-gradient(
              '0px' circle at ${mouseX}px ${mouseY}px,
              var(--blue-500),
              transparent 80%
            )
          `
	}}
>
	<div
		use:motion
		on:mousemove={handleMouseMove}
		on:mouseenter={() => (visible = true)}
		on:mouseleave={() => (visible = false)}
		role="none"
		class="group/input rounded-lg p-[2px] transition duration-300"
	>
		<select
			class={cn(
				`placeholder-text-neutral-600 duration-400 flex h-10 w-full rounded-md border-none px-3 py-2 text-sm transition file:border-0 
          file:bg-transparent file:text-sm file:font-medium placeholder:text-neutral-400 
          focus-visible:outline-none focus-visible:ring-[2px]   disabled:cursor-not-allowed
          disabled:opacity-50 group-hover/input:shadow-none
          bg-zinc-800
          text-white shadow-[0px_0px_1px_1px_var(--neutral-700)] focus-visible:ring-neutral-600
          `,
				className
			)}
			{...$$restProps}
		>
			{#each configs as option}
				<option value={option}>{option}</option>
			{/each}
		</select>
	</div>
</Motion>
