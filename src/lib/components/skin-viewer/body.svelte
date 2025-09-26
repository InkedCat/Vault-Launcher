<script lang="ts">
	import { T } from '@threlte/core';
	import { BaseLayerMaterial, StyleLayerMaterial } from './materials';
	import type { Texture } from 'three';
	import { generateBodyUV, setPartUV } from './utils';

	interface BodyProps {
		texture: Texture;
		x?: number;
		y?: number;
		z?: number;
		rx?: number;
		ry?: number;
		rz?: number;
	}

	const { texture, x = 0, y = 0, z = 0, rx = 0, ry = 0, rz = 0 }: BodyProps = $props();
</script>

<T.Group position={[x, y, z]}>
	<T.Group position={[0, 6, 0]} rotation={[rx, ry, rz]}>
		<T.Group position={[0, -6, 0]}>
			<T.Mesh>
				<T.BoxGeometry args={[8, 12, 4]} oncreate={(box) => setPartUV(box, generateBodyUV())} />
				<BaseLayerMaterial {texture} />
			</T.Mesh>
			<T.Mesh>
				<T.BoxGeometry
					args={[8.5, 12.5, 4.5]}
					oncreate={(box) => setPartUV(box, generateBodyUV(true))}
				/>
				<StyleLayerMaterial {texture} />
			</T.Mesh>
		</T.Group>
	</T.Group>
</T.Group>
