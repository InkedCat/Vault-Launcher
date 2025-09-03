<script lang="ts">
	import { Titlebar } from '$lib/components';
	import type { LayoutData } from './$types';
	import type { Snippet } from 'svelte';

	let { data, children }: { data: LayoutData; children: Snippet } = $props();
</script>

<div class="flex h-screen flex-col gap-4">
	<Titlebar />
	<div class="flex-1">
		{@render children()}
	</div>
</div>
