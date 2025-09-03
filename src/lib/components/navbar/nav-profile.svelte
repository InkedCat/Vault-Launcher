<script lang="ts">
	import * as Sidebar from '$lib/components/ui/sidebar';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Avatar from '$lib/components/ui/avatar';
	import { toggleMode } from 'mode-watcher';
	import {
		Sun,
		Moon,
		ChevronsUpDown,
		LogOut,
		Settings,
		ArrowLeftRight as Switch
	} from 'lucide-svelte';

	import type { UserProfile } from './types';

	interface NavProfileProps {
		user: UserProfile;
	}

	let { user }: NavProfileProps = $props();
</script>

<Sidebar.Menu>
	<Sidebar.MenuItem>
		<DropdownMenu.Root>
			<DropdownMenu.Trigger>
				{#snippet child({ props })}
					<Sidebar.MenuButton
						size="lg"
						class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
						{...props}
					>
						<Avatar.Root class="size-8 rounded-lg">
							<Avatar.Image src={user.avatar} alt={user.name} />
							<Avatar.Fallback class="rounded-lg">CN</Avatar.Fallback>
						</Avatar.Root>
						<div class="flex-1">
							<span class="truncate text-sm font-semibold">{user.name}</span>
						</div>
						<ChevronsUpDown class="ml-auto size-4" />
					</Sidebar.MenuButton>
				{/snippet}
			</DropdownMenu.Trigger>
			<DropdownMenu.Content
				class="w-(--bits-dropdown-menu-anchor-width) min-w-56 rounded-lg"
				side="right"
				align="end"
				sideOffset={4}
			>
				<DropdownMenu.Group>
					<DropdownMenu.Item onSelect={toggleMode}>
						<Sun class="scale-100 transition-all dark:scale-0" />
						<Moon class="absolute scale-0 transition-all dark:scale-100" />
						Toggle theme
					</DropdownMenu.Item>
				</DropdownMenu.Group>
				<DropdownMenu.Group>
					<DropdownMenu.Item>
						<Settings />
						Settings
					</DropdownMenu.Item>
				</DropdownMenu.Group>
				<DropdownMenu.Separator />
				<DropdownMenu.Group>
					<DropdownMenu.Item>
						<Switch />
						Switch Account
					</DropdownMenu.Item>
					<DropdownMenu.Item>
						<LogOut />
						Logout
					</DropdownMenu.Item>
				</DropdownMenu.Group>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</Sidebar.MenuItem>
</Sidebar.Menu>
