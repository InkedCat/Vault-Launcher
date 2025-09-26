<script lang="ts">
	import { T } from '@threlte/core';
	import { BaseLayerMaterial, StyleLayerMaterial } from './materials';
	import type { Texture } from 'three';
	import { setPartUV, generateHeadUV } from './utils';

	interface HeadProps {
		texture: Texture;
		x?: number;
		y?: number;
		z?: number;
		rx?: number;
		ry?: number;
		rz?: number;
	}

	const { texture, x = 0, y = 0, z = 0, rx = 0, ry = 0, rz = 0 }: HeadProps = $props();
</script>

<T.Group position={[x, y, z]}>
	<T.Group position={[0, -4, 0]} rotation={[rx, ry, rz]}>
		<T.Group position={[0, 4, 0]}>
			<T.Mesh>
				<T.BoxGeometry args={[8, 8, 8]} oncreate={(box) => setPartUV(box, generateHeadUV())} />
				<BaseLayerMaterial {texture} />
			</T.Mesh>
			<T.Mesh>
				<T.BoxGeometry args={[9, 9, 9]} oncreate={(box) => setPartUV(box, generateHeadUV(true))} />
				<StyleLayerMaterial {texture} />
			</T.Mesh>
		</T.Group>
	</T.Group>
</T.Group>
