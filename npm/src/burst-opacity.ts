const smoothstep = (value: number): number => {
  const clamped = Math.min(1, Math.max(0, value));
  return clamped * clamped * (3 - 2 * clamped);
};

const burstOpacity = (elapsed: number, duration: number): number => {
  if (duration <= 0) {
    return 1;
  }
  if (elapsed >= duration) {
    return 0;
  }

  const fadeStart = duration * 0.42;
  if (elapsed <= fadeStart) {
    return 1;
  }

  return 1 - smoothstep((elapsed - fadeStart) / (duration - fadeStart));
};

export default burstOpacity;
