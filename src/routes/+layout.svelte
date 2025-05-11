<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { LayoutData } from './$types';
	import { locales, localizeHref } from '@/paraglide/runtime';

	import '../app.css';
	import { page } from '$app/state';

	let { data, children }: { data: LayoutData; children: Snippet } = $props();
</script>

{@render children()}

<div style="display:none">
	{#each locales as locale}
		<a href={localizeHref(page.url.pathname, { locale })}>{locale}</a>
	{/each}
</div>
