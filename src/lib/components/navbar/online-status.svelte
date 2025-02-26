<script lang="ts">
	interface OnlineStatusProps {
		isOnline: boolean;
	}

	let { isOnline }: OnlineStatusProps = $props();
</script>

<div class="flex items-center gap-2">
	<div class="relative flex size-2">
		<div class="absolute size-2 rounded-full {isOnline ? 'bg-green-400' : 'bg-red-400'}"></div>
		<div
			class="relative size-2 animate-ping rounded-full {isOnline ? 'bg-green-300' : 'bg-red-300'}"
		></div>
	</div>

	<span>{isOnline ? 'online' : 'offline'}</span>
</div>
