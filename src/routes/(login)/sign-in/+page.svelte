<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';

	import type { PageData } from './$types';
	import { LocaleSwitch, LightSwitch, MicrosoftConnect } from '@/components';

	function openLogin() {
		invoke('open_microsoft_login');
	}

	let { data }: { data: PageData } = $props();
</script>

<div class="relative flex size-full flex-col items-center justify-center">
	<div class="absolute left-0 top-0 flex w-full justify-between px-4">
		<LocaleSwitch />
		<LightSwitch />
	</div>
	<MicrosoftConnect {openLogin} />
</div>
