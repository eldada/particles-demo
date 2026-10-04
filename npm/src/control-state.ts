export type ControlState = Readonly<{
  particleCount: number;
  cubeSize: number;
  rotationSpeed: number;
  cameraDistance: number;
  direction: Readonly<{ x: number; y: number }>;
}>;

export const createInitialControlState = (): ControlState => ({
  particleCount: 2800,
  cubeSize: 2.8,
  rotationSpeed: 0.14,
  cameraDistance: 11.4,
  direction: { x: 0, y: 0 },
});

const clamp = (value: number, minimum: number, maximum: number): number =>
  Math.min(maximum, Math.max(minimum, value));

export const reduceControlState = (
  state: ControlState,
  key: string,
): ControlState => {
  const directionSteps: Readonly<Record<string, ControlState['direction']>> = {
    ArrowLeft: { x: -1, y: 0 },
    ArrowRight: { x: 1, y: 0 },
    ArrowUp: { x: 0, y: 1 },
    ArrowDown: { x: 0, y: -1 },
  };
  const directionStep = directionSteps[key];
  if (directionStep !== undefined) {
    return {
      ...state,
      direction: {
        x: clamp(state.direction.x + directionStep.x, -3, 3),
        y: clamp(state.direction.y + directionStep.y, -3, 3),
      },
    };
  }

  if (key === '+' || key === '=') {
    return {
      ...state,
      particleCount: clamp(state.particleCount + 300, 600, 6000),
    };
  }
  if (key === '-') {
    return {
      ...state,
      particleCount: clamp(state.particleCount - 300, 600, 6000),
    };
  }
  if (key === '[' || key === ']') {
    const delta = key === ']' ? 0.08 : -0.08;
    return {
      ...state,
      rotationSpeed: clamp(
        Number((state.rotationSpeed + delta).toFixed(2)),
        -1.2,
        1.2,
      ),
    };
  }
  if (key.toLowerCase() === 'a' || key.toLowerCase() === 'z') {
    const delta = key.toLowerCase() === 'a' ? -0.8 : 0.8;
    return {
      ...state,
      cameraDistance: clamp(
        Number((state.cameraDistance + delta).toFixed(1)),
        5,
        18,
      ),
    };
  }
  if (/^[1-9]$/.test(key)) {
    return {
      ...state,
      cubeSize: Number((1.05 + Number(key) * 0.35).toFixed(2)),
    };
  }

  return state;
};
