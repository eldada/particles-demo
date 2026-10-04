import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import burstOpacity from './burst-opacity.js';
import burstTravel from './burst-travel.js';

describe('burst motion', () => {
  it('holds the burst open instead of returning it to the center', () => {
    assert.equal(burstTravel(0, 2.8), 0);
    assert.ok(burstTravel(1.2, 2.8) > 0.99);
    assert.ok(burstTravel(2.6, 2.8) > 0.99);
  });

  it('fades the burst out by the end', () => {
    assert.equal(burstOpacity(0.2, 2.8), 1);
    assert.ok(burstOpacity(2.7, 2.8) < 0.05);
    assert.equal(burstOpacity(2.8, 2.8), 0);
  });

  it('returns a resting burst for an invalid duration', () => {
    assert.equal(burstTravel(0.4, 0), 0);
    assert.equal(burstOpacity(0.4, 0), 1);
  });
});
