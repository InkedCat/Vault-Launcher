<script lang="ts">
	import * as Sidebar from '@/components/ui/sidebar';
	import type { SidebarGroup } from './types/sidebar';

	interface NavMainProps {
		groups: SidebarGroup[];
	}

	let { groups }: NavMainProps = $props();
</script>

<div>
	{#each groups as group (group.title)}
		<Sidebar.Group>
			<Sidebar.GroupLabel>{group.title}</Sidebar.GroupLabel>
			<Sidebar.GroupContent>
				<Sidebar.Menu>
					{#each group.items as item (item.name)}
						<Sidebar.MenuItem>
							<Sidebar.MenuButton>
								{#snippet child({ props })}
									<a href={item.url} {...props}>
										<item.icon />
										<span>{item.name}</span>
									</a>
								{/snippet}
							</Sidebar.MenuButton>
						</Sidebar.MenuItem>
					{/each}
				</Sidebar.Menu>
			</Sidebar.GroupContent>
		</Sidebar.Group>
	{/each}
</div>
