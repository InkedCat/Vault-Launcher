<script lang="ts">
	import * as Sidebar from '@/components/ui/sidebar';
	import NavProfile from './nav-profile.svelte';
	import ServerSwitcher from './server-switcher.svelte';
	import NavMain from './nav-main.svelte';
	import type { ComponentProps } from 'svelte';
	import type { ServerProfile, SidebarGroup, SidebarItem, UserProfile } from './index';

	import { Earth, Eclipse, House, Newspaper, Settings, Wallpaper } from 'lucide-svelte';

	interface Data {
		user: UserProfile;
		servers: ServerProfile[];
		groups: SidebarGroup[];
	}

	const serverItems: SidebarItem[] = [
		{
			name: 'Home',
			url: '/',
			icon: House
		},
		{
			name: 'News',
			url: '/server/news',
			icon: Newspaper
		},
		{
			name: 'Resource packs',
			url: '/server/ressources',
			icon: Eclipse
		},
		{
			name: 'Worlds',
			url: '/server/worlds',
			icon: Earth
		},
		{
			name: 'Screenshots',
			url: '/server/screenshots',
			icon: Wallpaper
		},
		{
			name: 'Settings',
			url: '/server/settings',
			icon: Settings
		}
	];

	const data: Data = {
		user: {
			name: 'Squensay',
			avatar: 'https://avatars.githubusercontent.com/u/146424693?s=96&v=4'
		},
		servers: [
			{
				name: 'Chronalia',
				icon: 'https://avatars.githubusercontent.com/u/163040649?v=4',
				online: true
			}
		],
		groups: [
			{
				title: 'Server',
				items: serverItems
			}
		]
	};

	let {
		ref = $bindable(null),
		collapsible = 'icon',
		...restProps
	}: ComponentProps<typeof Sidebar.Root> = $props();
</script>

<Sidebar.Root bind:ref {collapsible} {...restProps}>
	<Sidebar.Header>
		<ServerSwitcher servers={data.servers} addServer={() => console.log('Server added')} />
	</Sidebar.Header>
	<Sidebar.Content>
		<NavMain groups={data.groups} />
	</Sidebar.Content>
	<Sidebar.Footer>
		<NavProfile user={data.user} />
	</Sidebar.Footer>
</Sidebar.Root>
