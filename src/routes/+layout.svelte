<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { LayoutData } from './$types';
	import { locales, localizeHref } from '@/paraglide/runtime';

	import '../app.css';
	import { page } from '$app/state';

	import { ModeWatcher } from 'mode-watcher';
	let { data, children }: { data: LayoutData; children: Snippet } = $props();

	let isConnected = $state(false);
	$effect(() => {
		if (!isConnected && window.location.pathname !== '/sign-in') {
			window.location.href = '/sign-in';
		}
	});
</script>

<ModeWatcher />

{@render children()}

<div style="display:none">
	{#each locales as locale}
		<a href={localizeHref(page.url.pathname, { locale })}>{locale}</a>
	{/each}
</div>
