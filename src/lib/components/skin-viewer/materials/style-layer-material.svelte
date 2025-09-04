<script lang="ts">
	import { T } from '@threlte/core';
	import { DoubleSide, Texture } from 'three';

	interface StyleLayerProps {
		texture: Texture;
	}

	const { texture }: StyleLayerProps = $props();
</script>

<T.MeshStandardMaterial
	map={texture}
	side={DoubleSide}
	transparent
	alphaTest={0.5}
	depthWrite={true}
	depthTest={true}
	polygonOffset={true}
	polygonOffsetFactor={1}
	polygonOffsetUnits={1}
/>
