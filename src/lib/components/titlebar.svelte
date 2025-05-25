<script lang="ts">
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { Minus, Maximize, X as Cross } from 'lucide-svelte';

	const appWindow = getCurrentWindow();

	async function minimizeWindow() {
		console.log('minimize window');
		await appWindow.minimize();
	}

	async function maximizeWindow() {
		console.log('maximize window');
		await appWindow.toggleMaximize();
	}

	async function closeWindow() {
		console.log('close window');
		await appWindow.close();
	}
</script>

<div
	data-tauri-drag-region
	class="flex h-[var(--titlebar-height)] w-full items-center justify-end border-b-[1px] border-titlebar-border
			bg-titlebar-background"
>
	<button
		onmousedown={minimizeWindow}
		class="flex h-full w-11 items-center justify-center hover:bg-titlebar-accent"
	>
		<Minus size="16" />
	</button>
	<button
		onmousedown={maximizeWindow}
		class="flex h-full w-11 items-center justify-center hover:bg-titlebar-accent"
	>
		<Maximize size="14" />
	</button>
	<button
		onmousedown={closeWindow}
		class="flex h-full w-11 items-center justify-center hover:bg-red-500
				hover:text-primary-foreground dark:hover:text-primary"
	>
		<Cross size="18" />
	</button>
</div>
