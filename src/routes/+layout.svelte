<script lang="ts">
	import { onMount, type Snippet } from 'svelte';
	import type { LayoutData } from './$types';
	import { locales, localizeHref } from '$lib/paraglide/runtime';

	import '../app.css';
	import { page } from '$app/state';

	import { ModeWatcher } from 'mode-watcher';
	import { invoke } from '@tauri-apps/api/core';
	import { goto } from '$app/navigation';
	import { LoaderCircle } from 'lucide-svelte';
	import { fade } from 'svelte/transition';
	import { resolve } from '$app/paths';

	let { children }: { data: LayoutData; children: Snippet } = $props();

	let isLoading = $state(false);

	onMount(async () => {
		if (page.url.pathname === '/sign-in') {
			// hide the white flash screen
			await invoke('init_window');
			return;
		}
		isLoading = true;

		// wait for the loader to be rendered before showing the window
		await invoke('init_window');

		const accountState = await invoke<boolean>('get_account_state');
		if (!accountState) {
			await goto(resolve('/sign-in'), {});
		}

		await new Promise((resolve) => setTimeout(resolve, 1500));
		isLoading = false;
	});
</script>

<ModeWatcher />

{#if isLoading}
	<div
		out:fade={{ duration: 750 }}
		class="absolute flex h-screen w-full items-center justify-center"
	>
		<LoaderCircle size={128} class="animate-spin" />
	</div>
{:else}
	<div in:fade={{ duration: 750 }} class="absolute h-screen w-full">
		{@render children()}
	</div>
{/if}

<div style="display:none">
	{#each locales as locale (locale)}
		<!-- eslint-disable-next-line svelte/no-navigation-without-resolve -->
		<a href={localizeHref(page.url.pathname, { locale })}>{locale}</a>
	{/each}
</div>
