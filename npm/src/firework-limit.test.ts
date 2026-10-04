import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import fireworkLimit from './firework-limit.js';

describe('fireworkLimit', () => {
  it('keeps the burst inside the cube in particle space', () => {
    assert.equal(fireworkLimit(2.8, 0.5), 2.8);
    assert.equal(fireworkLimit(4.2, 0.5), 4.2);
  });

  it('returns zero when the particle scale is not positive', () => {
    assert.equal(fireworkLimit(2.8, 0), 0);
  });
});
