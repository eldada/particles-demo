const smoothstep = (value: number): number => {
  const clamped = Math.min(1, Math.max(0, value));
  return clamped * clamped * (3 - 2 * clamped);
};

const burstTravel = (elapsed: number, duration: number): number => {
  if (duration <= 0 || elapsed <= 0) {
    return 0;
  }

  return smoothstep(elapsed / (duration * 0.28));
};

export default burstTravel;
