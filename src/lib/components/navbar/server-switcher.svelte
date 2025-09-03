<script lang="ts">
	import * as DropdownMenu from '@/components/ui/dropdown-menu';
	import * as Sidebar from '@/components/ui/sidebar';
	import * as Avatar from '@/components/ui/avatar';
	import ChevronsUpDown from 'lucide-svelte/icons/chevrons-up-down';
	import Plus from 'lucide-svelte/icons/plus';
	import type { ServerProfile } from './types';
	import OnlineStatus from './online-status.svelte';

	interface ServerSwitcherProps {
		servers: ServerProfile[];
		addServer: () => void;
	}

	let { servers, addServer }: ServerSwitcherProps = $props();

	let activeServer = $state(servers[0]);
</script>

<Sidebar.Menu>
	<Sidebar.MenuItem>
		<DropdownMenu.Root>
			<DropdownMenu.Trigger>
				{#snippet child({ props })}
					<Sidebar.MenuButton
						{...props}
						size="lg"
						class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
					>
						<Avatar.Root class="size-8 rounded-lg">
							<Avatar.Image src={activeServer.icon} alt={activeServer.name} />
							<Avatar.Fallback class="rounded-lg bg-transparent text-white">
								{activeServer.name.slice(0, 1)}
							</Avatar.Fallback>
						</Avatar.Root>
						<div class="grid flex-1 text-left text-sm leading-tight">
							<span class="truncate font-semibold">
								{activeServer.name}
							</span>
							<OnlineStatus isOnline={activeServer.online} />
						</div>
						<ChevronsUpDown class="ml-auto" />
					</Sidebar.MenuButton>
				{/snippet}
			</DropdownMenu.Trigger>
			<DropdownMenu.Content
				class="w-(--bits-dropdown-menu-anchor-width) min-w-56 rounded-lg"
				align="start"
				side="right"
				sideOffset={4}
			>
				<DropdownMenu.Label class="text-xs text-muted-foreground">Servers</DropdownMenu.Label>
				{#each servers as server, index (server.name)}
					<DropdownMenu.Item onSelect={() => (activeServer = server)} class="gap-2 p-2">
						<div class="flex size-6 items-center justify-center rounded-sm border">
							<Avatar.Root class="size-4 shrink-0">
								<Avatar.Image src={activeServer.icon} alt={activeServer.name} />
								<Avatar.Fallback class="rounded-sm">
									{activeServer.name.slice(0, 1)}
								</Avatar.Fallback>
							</Avatar.Root>
						</div>
						{server.name}
						<DropdownMenu.Shortcut>⌘{index + 1}</DropdownMenu.Shortcut>
					</DropdownMenu.Item>
				{/each}
				<DropdownMenu.Separator />
				<DropdownMenu.Item onSelect={addServer} class="gap-2 p-2">
					<div class="flex size-6 items-center justify-center rounded-md border bg-background">
						<Plus class="size-4" />
					</div>
					<div class="font-medium text-muted-foreground">Add server</div>
				</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</Sidebar.MenuItem>
</Sidebar.Menu>
