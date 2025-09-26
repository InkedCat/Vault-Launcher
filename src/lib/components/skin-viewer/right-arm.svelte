<script lang="ts">
	import { T } from '@threlte/core';
	import { BaseLayerMaterial, StyleLayerMaterial } from './materials';
	import type { Texture } from 'three';
	import { generateArmUV, setPartUV } from './utils';

	interface RightArmProps {
		texture: Texture;
		slim?: boolean; // default: false
		x?: number; // default: 0
		y?: number; // default: 0
		z?: number; // default: 0
		rx?: number; // default: 0
		ry?: number; // default: 0
		rz?: number; // default: 0
	}

	const {
		slim = false,
		texture,
		x = 0,
		y = 0,
		z = 0,
		rx = 0,
		ry = 0,
		rz = 0
	}: RightArmProps = $props();
</script>

<T.Group position={[x, y, z]}>
	<T.Group position={[-1.5, 4, 0]} rotation={[rx, ry, rz]}>
		<T.Group position={[1.5, -4, 0]}>
			<T.Mesh scale={slim ? [3, 12, 4] : [4, 12, 4]}>
				<T.BoxGeometry oncreate={(box) => setPartUV(box, generateArmUV('right', slim))} />
				<BaseLayerMaterial {texture} />
			</T.Mesh>
			<T.Mesh scale={slim ? [3.5, 12.5, 4.5] : [4.5, 12.5, 4.5]}>
				<T.BoxGeometry oncreate={(box) => setPartUV(box, generateArmUV('right', slim, true))} />
				<StyleLayerMaterial {texture} />
			</T.Mesh>
		</T.Group>
	</T.Group>
</T.Group>
