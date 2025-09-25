<script lang="ts">
	import { Titlebar } from '$lib/components';
	import type { Snippet } from 'svelte';

	let { children }: { children: Snippet } = $props();
</script>

<div class="flex h-screen flex-col gap-4">
	<Titlebar />
	<div class="flex-1">
		{@render children()}
	</div>
</div>
