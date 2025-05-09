<script lang="ts">
	import { T } from '@threlte/core';
	import { BaseLayerMaterial, StyleLayerMaterial } from './materials';
	import type { Texture } from 'three';
	import { generateLegUV, setPartUV } from './utils';

	interface RightLegProps {
		texture: Texture;
		'position.x'?: number;
		'position.y'?: number;
		'position.z'?: number;
		'rotation.x'?: number;
		'rotation.y'?: number;
		'rotation.z'?: number;
	}

	const {
		texture,
		'position.x': x = 0,
		'position.y': y = 0,
		'position.z': z = 0,
		'rotation.x': rx = 0,
		'rotation.y': ry = 0,
		'rotation.z': rz = 0
	}: RightLegProps = $props();
</script>

<T.Group position={[x, y, z]}>
	<T.Group position={[0, 6, 0]} rotation={[rx, ry, rz]}>
		<T.Group position={[0, -6, 0]}>
			<T.Mesh>
				<T.BoxGeometry
					args={[4, 12, 4]}
					oncreate={(box) => setPartUV(box, generateLegUV('right'))}
				/>
				<BaseLayerMaterial {texture} />
			</T.Mesh>
			<T.Mesh>
				<T.BoxGeometry
					args={[4.5, 12.5, 4.5]}
					oncreate={(box) => setPartUV(box, generateLegUV('right', true))}
				/>
				<StyleLayerMaterial {texture} />
			</T.Mesh>
		</T.Group>
	</T.Group>
</T.Group>
