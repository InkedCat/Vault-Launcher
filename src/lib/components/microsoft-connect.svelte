<script lang="ts">
	import { m } from '$lib/paraglide/messages';
	import { Button } from './ui/button';
	import { LoaderCircle } from 'lucide-svelte';

	let connecting = $state(false);

	interface MicrosoftConnectProps {
		openLogin: () => void;
	}

	let { openLogin }: MicrosoftConnectProps = $props();
</script>

<Button
	disabled={connecting}
	class="h-10 scale-150 gap-3 px-4 transition-all duration-300"
	onclick={() => {
		connecting = true;
		openLogin();
	}}
>
	{#if connecting}
		<LoaderCircle class="animate-spin" />
		<span class="w-full text-center font-semibold">Loading...</span>
	{:else}
		<img src="icons/microsoft.svg" alt="Microsoft" class="size-6" />
		<span class="w-full text-center font-semibold">{m.acidic_every_vole_bloom()}</span>
	{/if}
</Button>
