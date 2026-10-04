import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import createSoccerBallEdges from './create-soccer-ball-edges.js';

const length = (
  positions: Float32Array,
  index: number,
): number => {
  const offset = index * 6;
  const dx = (positions[offset + 3] ?? 0) - (positions[offset] ?? 0);
  const dy = (positions[offset + 4] ?? 0) - (positions[offset + 1] ?? 0);
  const dz = (positions[offset + 5] ?? 0) - (positions[offset + 2] ?? 0);
  return Math.hypot(dx, dy, dz);
};

describe('createSoccerBallEdges', () => {
  it('builds a closed soccer-ball wireframe inside a unit sphere', () => {
    const edges = createSoccerBallEdges();
    const edgeCount = edges.length / 6;
    const firstLength = length(edges, 0);
    let maxRadius = 0;

    assert.equal(edgeCount, 90);
    for (let index = 0; index < edgeCount; index += 1) {
      assert.ok(Math.abs(length(edges, index) - firstLength) < 1e-6);
      for (const point of [0, 3]) {
        const radius = Math.hypot(
          edges[index * 6 + point] ?? 0,
          edges[index * 6 + point + 1] ?? 0,
          edges[index * 6 + point + 2] ?? 0,
        );
        maxRadius = Math.max(maxRadius, radius);
      }
    }
    assert.ok(Math.abs(maxRadius - 0.5) < 1e-6);
  });
});
