const smoothstep = (value: number): number => {
  const clamped = Math.min(1, Math.max(0, value));
  return clamped * clamped * (3 - 2 * clamped);
};

const effectEnvelope = (elapsed: number, duration: number): number => {
  if (duration <= 0 || elapsed <= 0 || elapsed >= duration) {
    return 0;
  }

  const attack = smoothstep(elapsed / (duration * 0.14));
  const decayStart = duration * 0.2;
  const decay = 1 - smoothstep((elapsed - decayStart) / (duration - decayStart));
  return attack * decay;
};

export default effectEnvelope;
