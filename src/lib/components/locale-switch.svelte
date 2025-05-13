<script lang="ts">
	import * as Select from '@/components/ui/select';
	import { platformLanguages, type Locales } from '@/constants';
	import { locales, setLocale, getLocale } from '@/paraglide/runtime';
	import { Earth } from 'lucide-svelte';

	interface LocaleItem {
		label: string;
		value: Locales;
	}

	const localeItems: LocaleItem[] = locales.map((locale) => ({
		label: platformLanguages[locale],
		value: locale
	}));

	let selectedLocale = $state<Locales>(getLocale());

	const changeLocale = (value: Locales) => {
		if (locales.includes(value)) {
			setLocale(value);
		}
	};

	const triggerContent = $derived(localeItems.find((f) => f.value === selectedLocale)?.label);
</script>

<Select.Root
	type="single"
	bind:value={selectedLocale}
	onValueChange={(value) => changeLocale(value as Locales)}
>
	<Select.Trigger class="w-[180px]">
		<Earth class="size-4" />
		{triggerContent}
	</Select.Trigger>
	<Select.Content>
		{#each localeItems as locale}
			<Select.Item value={locale.value} label={locale.label} />
		{/each}
	</Select.Content>
</Select.Root>
