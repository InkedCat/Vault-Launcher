<script lang="ts">
	import { T } from '@threlte/core';
	import { OrbitControls, useTexture } from '@threlte/extras';
	import Head from './head.svelte';
	import Body from './body.svelte';
	import RightArm from './right-arm.svelte';
	import LeftArm from './left-arm.svelte';
	import RightLeg from './right-leg.svelte';
	import LeftLeg from './left-leg.svelte';
	import { NearestFilter } from 'three';
	import Renderer from './renderer.svelte';
	import { generateArmUV, generateBodyUV, generateHeadUV, generateLegUV } from './utils';

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
	<OrbitControls
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

{#await texture then map}
	<T.Group>
		<Head
			texture={map}
			position.y={28}
			rotation.x={6 * (Math.PI / 180)}
			rotation.y={-5 * (Math.PI / 180)}
		/>
		<Body texture={map} position.y={18} />
		<RightArm
			texture={map}
			{slim}
			position.y={18}
			position.x={6 - (slim ? 0.5 : 0)}
			rotation.x={10 * (Math.PI / 180)}
		/>
		<LeftArm
			texture={map}
			{slim}
			position.y={18}
			position.x={-6 + (slim ? 0.5 : 0)}
			rotation.x={-12 * (Math.PI / 180)}
		/>
		<RightLeg
			texture={map}
			position.y={6}
			position.x={2}
			rotation.x={-11 * (Math.PI / 180)}
			rotation.z={2 * (Math.PI / 180)}
		/>
		<LeftLeg
			texture={map}
			position.y={6}
			position.x={-2}
			rotation.x={10 * (Math.PI / 180)}
			rotation.z={-2 * (Math.PI / 180)}
		/>
	</T.Group>
{/await}
