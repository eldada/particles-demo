import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import effectEnvelope from './effect-envelope.js';

describe('effectEnvelope', () => {
  it('starts at zero, peaks, and returns to zero', () => {
    assert.equal(effectEnvelope(0, 2.4), 0);
    assert.ok(effectEnvelope(0.45, 2.4) > 0.9);
    assert.equal(effectEnvelope(2.4, 2.4), 0);
    assert.equal(effectEnvelope(4, 2.4), 0);
  });

  it('returns zero for an invalid duration', () => {
    assert.equal(effectEnvelope(0.5, 0), 0);
  });
});
