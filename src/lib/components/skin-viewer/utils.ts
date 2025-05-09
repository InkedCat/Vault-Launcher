import { BoxGeometry, BufferAttribute, Vector2 } from 'three';

type UVFace = 'front' | 'back' | 'top' | 'bottom' | 'left' | 'right';

const uvFaces: UVFace[] = ['left', 'right', 'top', 'bottom', 'back', 'front'];

type FaceVertices = [Vector2, Vector2, Vector2, Vector2];
export type PartUV = { [key in UVFace]: FaceVertices };

export type BodyPart = 'head' | 'body' | 'rightArm' | 'leftArm' | 'rightLeg' | 'leftLeg';
export type UVConfig = {
	x: number;
	y: number;
	width: number;
	height: number;
	styleOffset?: { x?: number; y?: number };
	slimOffset?: { x?: number; width?: number };
};

const textureSize = 64;

function generateFaceVertices(x1: number, y1: number, x2: number, y2: number): FaceVertices {
	const u1 = x1 / textureSize;
	const v1 = y1 / textureSize;
	const u2 = x2 / textureSize;
	const v2 = y2 / textureSize;

	return [new Vector2(u1, v1), new Vector2(u2, v1), new Vector2(u1, v2), new Vector2(u2, v2)];
}

// Mapping configuration for each body part and face
const uvConfigs: Record<BodyPart, Record<UVFace, UVConfig>> = {
	head: {
		right: { x: 16, y: 8, width: 8, height: 8, styleOffset: { x: 32 } },
		left: { x: 0, y: 8, width: 8, height: 8, styleOffset: { x: 32 } },
		top: { x: 16, y: 8, width: -8, height: -8, styleOffset: { x: 32 } },
		bottom: { x: 16, y: 0, width: 8, height: 8, styleOffset: { x: 32 } },
		front: { x: 8, y: 8, width: 8, height: 8, styleOffset: { x: 32 } },
		back: { x: 24, y: 8, width: 8, height: 8, styleOffset: { x: 32 } }
	},
	body: {
		right: { x: 28, y: 20, width: 4, height: 12, styleOffset: { y: 16 } },
		left: { x: 16, y: 20, width: 4, height: 12, styleOffset: { y: 16 } },
		top: { x: 28, y: 20, width: -8, height: -4, styleOffset: { y: 16 } },
		bottom: { x: 36, y: 20, width: -8, height: -4, styleOffset: { y: 16 } },
		front: { x: 20, y: 20, width: 8, height: 12, styleOffset: { y: 16 } },
		back: { x: 32, y: 20, width: 8, height: 12, styleOffset: { y: 16 } }
	},
	leftArm: {
		right: { x: 40, y: 52, width: 4, height: 12, styleOffset: { x: 16 }, slimOffset: { x: -1 } },
		left: { x: 32, y: 52, width: 4, height: 12, styleOffset: { x: 16 } },
		top: {
			x: 40,
			y: 52,
			width: -4,
			height: -4,
			styleOffset: { x: 16 },
			slimOffset: { x: -1, width: -3 }
		},
		bottom: {
			x: 44,
			y: 52,
			width: -4,
			height: -4,
			styleOffset: { x: 16 },
			slimOffset: { x: -2, width: -3 }
		},
		front: { x: 36, y: 52, width: 4, height: 12, styleOffset: { x: 16 }, slimOffset: { width: 3 } },
		back: {
			x: 44,
			y: 52,
			width: 4,
			height: 12,
			styleOffset: { x: 16 },
			slimOffset: { x: -1, width: 3 }
		}
	},
	rightArm: {
		right: { x: 48, y: 20, width: 4, height: 12, styleOffset: { y: 16 }, slimOffset: { x: -1 } },
		left: { x: 40, y: 20, width: 4, height: 12, styleOffset: { y: 16 } },
		top: {
			x: 48,
			y: 20,
			width: -4,
			height: -4,
			styleOffset: { y: 16 },
			slimOffset: { x: -1, width: -3 }
		},
		bottom: {
			x: 52,
			y: 20,
			width: -4,
			height: -4,
			styleOffset: { y: 16 },
			slimOffset: { x: -2, width: -3 }
		},
		front: { x: 44, y: 20, width: 4, height: 12, styleOffset: { y: 16 }, slimOffset: { width: 3 } },
		back: {
			x: 52,
			y: 20,
			width: 4,
			height: 12,
			styleOffset: { y: 16 },
			slimOffset: { x: -1, width: 3 }
		}
	},
	leftLeg: {
		right: { x: 24, y: 52, width: 4, height: 12, styleOffset: { x: -16 } },
		left: { x: 16, y: 52, width: 4, height: 12, styleOffset: { x: -16 } },
		top: { x: 24, y: 52, width: -4, height: -4, styleOffset: { x: -16 } },
		bottom: { x: 28, y: 52, width: -4, height: -4, styleOffset: { x: -16 } },
		front: { x: 20, y: 52, width: 4, height: 12, styleOffset: { x: -16 } },
		back: { x: 28, y: 52, width: 4, height: 12, styleOffset: { x: -16 } }
	},
	rightLeg: {
		right: { x: 8, y: 20, width: 4, height: 12, styleOffset: { y: 16 } },
		left: { x: 0, y: 20, width: 4, height: 12, styleOffset: { y: 16 } },
		top: { x: 8, y: 20, width: -4, height: -4, styleOffset: { y: 16 } },
		bottom: { x: 12, y: 20, width: -4, height: -4, styleOffset: { y: 16 } },
		front: { x: 4, y: 20, width: 4, height: 12, styleOffset: { y: 16 } },
		back: { x: 12, y: 20, width: 4, height: 12, styleOffset: { y: 16 } }
	}
};

/**
 * Generates UV mapping for any body part
 * @param part The body part to generate UVs for
 * @param options Configuration options
 * @returns PartUV mapping for the specified body part
 */
export function generatePartUV(
	part: BodyPart,
	options: { style?: boolean; slim?: boolean } = {}
): PartUV {
	const { style = false, slim = false } = options;
	const config = uvConfigs[part];

	const result: PartUV = {} as PartUV;

	for (const face of uvFaces) {
		const faceConfig = config[face];

		let x = faceConfig.x;
		let y = faceConfig.y;

		// Apply style offsets if needed
		if (style && faceConfig.styleOffset) {
			if (faceConfig.styleOffset.x !== undefined) x += faceConfig.styleOffset.x;
			if (faceConfig.styleOffset.y !== undefined) y += faceConfig.styleOffset.y;
		}

		// Apply slim model adjustments for arms if needed
		let finalX = x;
		let finalWidth = faceConfig.width;

		if (slim && faceConfig.slimOffset) {
			const slimOffset = faceConfig.slimOffset;
			if (slimOffset.width !== undefined) finalWidth = slimOffset.width;
			if (slimOffset.x !== undefined) finalX += slimOffset.x;
		}

		result[face] = generateFaceVertices(finalX, y, finalX + finalWidth, y + faceConfig.height);
	}

	return result;
}

// Convenience functions that use the generalized method
export function generateHeadUV(style: boolean = false): PartUV {
	return generatePartUV('head', { style });
}

export function generateBodyUV(style: boolean = false): PartUV {
	return generatePartUV('body', { style });
}

export function generateArmUV(
	side: 'right' | 'left',
	slim: boolean = false,
	style: boolean = false
): PartUV {
	return generatePartUV(side === 'right' ? 'rightArm' : 'leftArm', { slim, style });
}

export function generateLegUV(side: 'right' | 'left', style: boolean = false): PartUV {
	return generatePartUV(side === 'right' ? 'rightLeg' : 'leftLeg', { style });
}

export function setPartUV(geometry: BoxGeometry, partUv: PartUV): void {
	const uvAttribute = geometry.getAttribute('uv') as BufferAttribute;

	let faceIndex = 0;
	for (let i = 0; i < uvAttribute.count; i += 4) {
		const face = partUv[uvFaces[faceIndex++]];
		uvAttribute.setXY(i, face[0].x, face[0].y);
		uvAttribute.setXY(i + 1, face[1].x, face[1].y);
		uvAttribute.setXY(i + 2, face[2].x, face[2].y);
		uvAttribute.setXY(i + 3, face[3].x, face[3].y);
	}

	uvAttribute.needsUpdate = true;
}
