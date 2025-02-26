import type { Icon as LucideIcon } from 'lucide-svelte';

export interface ServerProfile {
	name: string;
	icon: string;
	online: boolean;
}

export interface UserProfile {
	name: string;
	avatar: string;
}

export interface SidebarGroup {
	title: string;
	items: SidebarItem[];
}

export interface SidebarItem {
	name: string;
	url: string;
	icon: typeof LucideIcon;
}
