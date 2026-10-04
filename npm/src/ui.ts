import type { ControlState } from './control-state.js';
import type { EffectName, UiHandles } from './types.js';

const requireElement = (root: ParentNode, selector: string): HTMLElement => {
  const element = root.querySelector(selector);
  if (!(element instanceof HTMLElement)) {
    throw new Error(`Required interface element '${selector}' was not created.`);
  }
  return element;
};

const requireButton = (
  root: ParentNode,
  selector: string,
): HTMLButtonElement => {
  const element = root.querySelector(selector);
  if (!(element instanceof HTMLButtonElement)) {
    throw new Error(`Required interface button '${selector}' was not created.`);
  }
  return element;
};

const createUi = (container: HTMLElement): UiHandles => {
  const interfaceElement = document.createElement('div');
  interfaceElement.className = 'interface';
  interfaceElement.innerHTML = `
    <section class="key-panel" aria-labelledby="show-title">
      <p class="eyebrow">Interactive study 01</p>
      <h1 id="show-title">Ember Field</h1>
      <dl class="key-list">
        <div class="key-row"><dt><kbd>↑</kbd><kbd>↓</kbd></dt><dd>Extend vertically</dd></div>
        <div class="key-row"><dt><kbd>←</kbd><kbd>→</kbd></dt><dd>Extend horizontally</dd></div>
        <div class="key-row"><dt><kbd>+</kbd><kbd>−</kbd></dt><dd>Particle density</dd></div>
        <div class="key-row"><dt><kbd>F</kbd></dt><dd>Firework burst</dd></div>
        <div class="key-row"><dt><kbd>1–9</kbd></dt><dd>Cube dimension</dd></div>
        <div class="key-row"><dt><kbd>[</kbd><kbd>]</kbd></dt><dd>Rotation speed</dd></div>
        <div class="key-row"><dt><kbd>A</kbd><kbd>Z</kbd></dt><dd>Zoom in / out</dd></div>
      </dl>
    </section>
    <div class="status" aria-live="polite">
      <span data-status="particles">2,800 particles</span>
      <span data-status="cube">cube 2.8</span>
      <span data-status="speed">spin +0.14</span>
    </div>
    <p class="effect-label" data-status="effect">system stable</p>
    <div class="touch-actions" aria-label="Show effects">
      <button type="button" data-action="firework">Firework</button>
    </div>
  `;
  container.append(interfaceElement);

  return {
    particleCount: requireElement(interfaceElement, '[data-status="particles"]'),
    cubeSize: requireElement(interfaceElement, '[data-status="cube"]'),
    rotationSpeed: requireElement(interfaceElement, '[data-status="speed"]'),
    effectLabel: requireElement(interfaceElement, '[data-status="effect"]'),
    fireworkButton: requireButton(interfaceElement, '[data-action="firework"]'),
  };
};

export const updateUiState = (ui: UiHandles, state: ControlState): void => {
  ui.particleCount.textContent = `${state.particleCount.toLocaleString()} particles`;
  ui.cubeSize.textContent = `cube ${state.cubeSize.toFixed(1)}`;
  const sign = state.rotationSpeed >= 0 ? '+' : '';
  ui.rotationSpeed.textContent = `spin ${sign}${state.rotationSpeed.toFixed(2)}`;
};

export const updateUiEffect = (ui: UiHandles, effect: EffectName): void => {
  const labels: Readonly<Record<EffectName, string>> = {
    idle: 'system stable',
    firework: 'firework sequence',
  };
  ui.effectLabel.textContent = labels[effect];
  ui.effectLabel.classList.toggle('active', effect !== 'idle');
};

export default createUi;
