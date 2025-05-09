<script lang="ts">
	import { T } from '@threlte/core';
	import { BaseLayerMaterial, StyleLayerMaterial } from './materials';
	import type { Texture } from 'three';
	import { generateArmUV, setPartUV } from './utils';

	interface RightArmProps {
		texture: Texture;
		slim?: boolean; // default: false
		'position.x'?: number; // default: 0
		'position.y'?: number; // default: 0
		'position.z'?: number; // default: 0
		'rotation.x'?: number; // default: 0
		'rotation.y'?: number; // default: 0
		'rotation.z'?: number; // default: 0
	}

	const {
		slim = false,
		texture,
		'position.x': x = 0,
		'position.y': y = 0,
		'position.z': z = 0,
		'rotation.x': rx = 0,
		'rotation.y': ry = 0,
		'rotation.z': rz = 0
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
