<script lang="ts">
	import { podStore, updatePod } from '$lib/pods';
	import { onMount } from 'svelte';
	import EvervaultCard from '$lib/EverVault/EvervaultCard.svelte';
	import ChangeConfigForm from '$lib/ChangeConfig/Form.svelte';
	import Icon from '@iconify/svelte';
	import type { Pod } from '$lib/types';
	let configs: string[] = [];
	function displayVolume(pod: Pod): string {
		let display = '';
		if (!pod.spec.volumes) {
			return '';
		}
		for (let i = 0; i < pod.spec.volumes.length; i = i + 1) {
			if (pod.spec.volumes[i].name === 'config') {
				return pod.spec.volumes[i].projected!.sources![0].configMap!.name!;
			}
		}
		return display;
	}

	function getBorderColor(color: string): string {
		switch (color) {
			case 'green':
				return 'border-[#00ab2569]';
			case 'blue':
				return 'border-[#208ac788]';

			case 'amber':
				return 'border-[#ff910088]';

			case 'delete':
				return 'border-[#00a2ffa8]';

			default:
				return '';
		}
	}

	onMount(async () => {
		const sseSource = new EventSource('/sse');
		sseSource.onmessage = updatePod;
		console.log(configs);
	});
</script>

<div class="relative w-full flex h-[30rem] flex-row items-center justify-center p-4 text-white">
	<ChangeConfigForm />
</div>
<div class="relative w-full flex h-[30rem] flex-row justify-center p-10 flex-wrap">
	{#each $podStore as pod (`${pod.metadata.name} ${pod.rowClass}`)}
		<div class="m-4 h-fit">
			<EvervaultCard
				className="p-4 relative flex h-fit w-min flex-wrap flex-row rounded-md font-medium border {getBorderColor(
					pod.rowClass ? pod.rowClass : ''
				)}"
			>
				<div class="w-fit flex flex-col text-nowrap whitespace-nowrap">
					<span class="flex space-x-2">
						<Icon icon="carbon:ibm-cloud-kubernetes-service" class="text-[#eff4ffea] h-5 w-5" />
						<p class="text-[#eff4ffea] text-nowrap whitespace-nowrap">{pod.metadata.name}</p>
					</span>
				</div>
				<div class="w-fit flex flex-col text-nowrap whitespace-nowrap">
					<span class="flex space-x-2">
						<Icon icon="mynaui:config" class="text-[#eff4ffea] h-5 w-5" />
						<p class="text-[#eff4ffea] whitespace-nowrap">{displayVolume(pod)}</p>
					</span>
				</div>
			</EvervaultCard>
		</div>
	{/each}
</div>

<style lang="postcss">
	:global(html) {
		@apply bg-black;
		color: #d7f0ffdb;
	}
</style>
