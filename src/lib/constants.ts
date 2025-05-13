import { locales } from '@/paraglide/runtime';

export type Locales = (typeof locales)[number];

type Languages = {
	[key in Locales]: string;
};

export const platformLanguages: Languages = {
	en: 'English',
	fr: 'Français'
} as const;
