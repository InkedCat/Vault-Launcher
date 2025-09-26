<script lang="ts">
	import { extend, T } from '@threlte/core';
	import { OrbitControls, useTexture } from '@threlte/extras';
	import Head from './head.svelte';
	import Body from './body.svelte';
	import RightArm from './right-arm.svelte';
	import LeftArm from './left-arm.svelte';
	import RightLeg from './right-leg.svelte';
	import LeftLeg from './left-leg.svelte';
	import { NearestFilter } from 'three';
	import Renderer from './renderer.svelte';

	extend({
		OrbitControls
	});

	interface LeftArmProps {
		texturePath: string;
		slim?: boolean; // default: false
	}

	const { slim = false, texturePath }: LeftArmProps = $props();

	const texture = useTexture(texturePath, {
		transform: (texture) => {
			texture.magFilter = NearestFilter;
			texture.minFilter = NearestFilter;
			texture.flipY = false;
		}
	});
</script>

<T.PerspectiveCamera makeDefault position={[-10, 16, -42]}>
	<T.OrbitControls
		minPolarAngle={Math.PI / 2.4}
		maxPolarAngle={0}
		enableDamping
		enableZoom={false}
		enablePan={false}
		target={[0, 16, 0]}
		dampingFactor={0.02}
	/>
</T.PerspectiveCamera>

<T.AmbientLight intensity={2} />

<Renderer />

{#await texture.promise then map}
	{#if map}
		<T.Group>
			<Head texture={map} y={28} rx={6 * (Math.PI / 180)} ry={-5 * (Math.PI / 180)} />
			<Body texture={map} y={18} />
			<RightArm texture={map} {slim} y={18} x={6 - (slim ? 0.5 : 0)} rx={10 * (Math.PI / 180)} />
			<LeftArm texture={map} {slim} y={18} x={-6 + (slim ? 0.5 : 0)} rx={-12 * (Math.PI / 180)} />
			<RightLeg texture={map} y={6} x={2} rx={-11 * (Math.PI / 180)} rz={2 * (Math.PI / 180)} />
			<LeftLeg texture={map} y={6} x={-2} rx={10 * (Math.PI / 180)} rz={-2 * (Math.PI / 180)} />
		</T.Group>
	{:else}
		<T.Group>
			<T.Text text="Loading texture..." />
		</T.Group>
	{/if}
{/await}
