import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import {
  createInitialControlState,
  reduceControlState,
} from './control-state.js';

describe('reduceControlState', () => {
  it('extends the burn farther with repeated arrow keys', () => {
    const initial = createInitialControlState();
    const left = reduceControlState(initial, 'ArrowLeft');

    assert.deepEqual(left.direction, { x: -1, y: 0 });
    assert.deepEqual(reduceControlState(left, 'ArrowLeft').direction, {
      x: -2,
      y: 0,
    });
    assert.deepEqual(
      reduceControlState(
        { ...initial, direction: { x: 0, y: 3 } },
        'ArrowUp',
      ).direction,
      { x: 0, y: 3 },
    );
  });

  it('keeps particle count within its performance bounds', () => {
    const initial = createInitialControlState();
    const maximum = { ...initial, particleCount: 6000 };
    const minimum = { ...initial, particleCount: 600 };

    assert.equal(reduceControlState(maximum, '+').particleCount, 6000);
    assert.equal(reduceControlState(maximum, '=').particleCount, 6000);
    assert.equal(reduceControlState(minimum, '-').particleCount, 600);
  });

  it('maps number keys to a readable cube-size range', () => {
    const initial = createInitialControlState();

    assert.equal(reduceControlState(initial, '1').cubeSize, 1.4);
    assert.equal(reduceControlState(initial, '9').cubeSize, 4.2);
  });

  it('zooms the camera with A and Z within safe bounds', () => {
    const initial = createInitialControlState();
    const nearest = { ...initial, cameraDistance: 5 };
    const farthest = { ...initial, cameraDistance: 18 };

    assert.equal(reduceControlState(initial, 'a').cameraDistance, 10.6);
    assert.equal(reduceControlState(initial, 'Z').cameraDistance, 12.2);
    assert.equal(reduceControlState(nearest, 'A').cameraDistance, 5);
    assert.equal(reduceControlState(farthest, 'z').cameraDistance, 18);
  });

  it('keeps cube speed bounded in both directions', () => {
    const initial = createInitialControlState();
    const fastest = { ...initial, rotationSpeed: 1.2 };
    const reversed = { ...initial, rotationSpeed: -1.2 };

    assert.equal(reduceControlState(fastest, ']').rotationSpeed, 1.2);
    assert.equal(reduceControlState(reversed, '[').rotationSpeed, -1.2);
  });

  it('does not mutate state for an unrelated key', () => {
    const initial = createInitialControlState();

    assert.equal(reduceControlState(initial, 'x'), initial);
  });
});
