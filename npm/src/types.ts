export type EffectName = 'idle' | 'firework';

export type Direction = Readonly<{
  x: number;
  y: number;
}>;

export type UiHandles = Readonly<{
  particleCount: HTMLElement;
  cubeSize: HTMLElement;
  rotationSpeed: HTMLElement;
  effectLabel: HTMLElement;
  fireworkButton: HTMLButtonElement;
}>;
