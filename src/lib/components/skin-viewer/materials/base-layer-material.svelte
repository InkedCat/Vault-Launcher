<script lang="ts">
	import { T } from '@threlte/core';
	import { FrontSide } from 'three';
	import type { Texture } from 'three';

	interface BaseLayerProps {
		texture: Texture;
	}

	const { texture }: BaseLayerProps = $props();
</script>

<T.MeshStandardMaterial 
	map={texture} 
	side={FrontSide} 
	depthWrite={true}
	depthTest={true}
	alphaTest={0.5}
	polygonOffset={true}
	polygonOffsetFactor={1}
	polygonOffsetUnits={1}
/>
