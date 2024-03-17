<script lang="ts">
	import Option from './Option.svelte';
	import Icon from '@iconify/svelte';
	import Label from './Label.svelte';
	import { onMount } from 'svelte';
	let configs: string[] = [];
	async function handleSubmit(event: { currentTarget: EventTarget & HTMLFormElement }) {
		const formData = new FormData(event.currentTarget);
		formData.set('deployment', 'vector-aws');
		console.log(formData);
		fetch('/set-configmaps', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(Object.fromEntries(formData))
		})
			.then((response) => {
				console.log(response.url);
				if (!response.ok) {
					throw new Error('Error sending config');
				}
				return response.json();
			})
			.then((data) => {
				console.log(data);
				if (data.error) {
					throw new Error(data.error);
				}
			})
			.catch((error) => {
				console.error(error);
			});
	}
	async function getConfigs(): Promise<string[]> {
		let returnData: string[] = [];
		await fetch('/configmaps')
			.then((response) => response.json())
			.then((data) => {
				returnData = data;
			});
		return returnData;
	}
	onMount(async () => {
		configs = await getConfigs();
	});
</script>

<div
	class="mx-auto w-full max-w-md rounded-none border p-4 shadow border-gray-800 bg-black md:rounded-2xl md:p-8"
>
	<h2 class="text-xl font-bold text-neutral-200">Set Vector Config</h2>
	<p class="mt-2 max-w-sm text-sm text-neutral-300">
		Set the deployment configmap for the vector server.
	</p>

	<form class="my-8" on:submit|preventDefault={handleSubmit}>
		<div class="mb-4 flex flex-col space-y-2 md:flex-row md:space-x-2 md:space-y-0">
			<div class={'flex w-full flex-col space-y-2'}>
				<Label htmlFor="config">Configuration Name</Label>
				<Option name="config" id="config" placeholder="Config" type="text" {configs} />
			</div>
		</div>

		<div
			class="my-8 h-[1px] w-full bg-gradient-to-r from-transparent to-transparent via-neutral-700"
		/>

		<div class="flex flex-col space-y-4">
			<button
				class=" group/btn relative flex h-10 w-full items-center justify-start space-x-2 rounded-md px-4 font-medium text-white bg-zinc-900 shadow-[0px_0px_1px_1px_var(--neutral-800)]"
				type="submit"
			>
				<Icon icon="mdi:kubernetes" class="text-blue-500 h-5 w-5" />
				<span class="text-sm text-neutral-200"> Set Deployment Config </span>
				<span
					class="absolute inset-x-0 -bottom-px block h-px w-full bg-gradient-to-r from-transparent via-cyan-500 to-transparent opacity-0 transition duration-500 group-hover/btn:opacity-100"
				/>
				<span
					class="absolute inset-x-10 -bottom-px mx-auto block h-px w-1/2 bg-gradient-to-r from-transparent via-indigo-500 to-transparent opacity-0 blur-sm transition duration-500 group-hover/btn:opacity-100"
				/>
			</button>
		</div>
	</form>
</div>
