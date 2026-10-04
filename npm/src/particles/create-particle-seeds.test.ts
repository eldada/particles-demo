import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import createParticleSeeds from './create-particle-seeds.js';

describe('createParticleSeeds', () => {
  it('creates four deterministic attributes for every particle', () => {
    const first = createParticleSeeds(3, () => 0.5);

    assert.equal(first.length, 12);
    assert.deepEqual([...first], [
      0.5, 0.5, 0.5, 0.5,
      0.5, 0.5, 0.5, 0.5,
      0.5, 0.5, 0.5, 0.5,
    ]);
  });

  it('returns an empty attribute for a non-positive count', () => {
    assert.equal(createParticleSeeds(0).length, 0);
  });
});
