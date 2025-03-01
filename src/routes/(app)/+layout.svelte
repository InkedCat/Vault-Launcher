<script lang="ts">
	import { ModeWatcher } from 'mode-watcher';
	import { onNavigate } from '$app/navigation';
	import * as Sidebar from '@/components/ui/sidebar';
	import { AppSidebar } from '@/components/navbar';
	import { Titlebar } from '@/components';

	let { children } = $props();

	onNavigate((navigation) => {
		if (!document.startViewTransition) return;

		return new Promise((resolve) => {
			document.startViewTransition(async () => {
				resolve();
				await navigation.complete;
			});
		});
	});

	let windowWidth = $state(0);
</script>

<svelte:window bind:innerWidth={windowWidth} />

<ModeWatcher />
<Sidebar.Provider open={windowWidth >= 1024}>
	<AppSidebar />
	<div class="flex w-full flex-1 flex-col">
		<Titlebar />
		<main class="flex-1 p-6">
			{@render children?.()}
		</main>
	</div>
</Sidebar.Provider>
