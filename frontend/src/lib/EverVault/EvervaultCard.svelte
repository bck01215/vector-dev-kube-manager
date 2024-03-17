<script lang="ts">
	import { cn } from '$lib/utils/cn';

	export let className: string | undefined = undefined;
</script>

<div class={cn('relative z-10', className)}><slot /></div>
