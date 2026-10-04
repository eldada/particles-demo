import {
  createInitialControlState,
  reduceControlState,
  type ControlState,
} from './control-state.js';
import type { EffectName, UiHandles } from './types.js';

type ControlCallbacks = Readonly<{
  onStateChange: (state: ControlState) => void;
  onEffect: (effect: Exclude<EffectName, 'idle'>) => void;
}>;

const HANDLED_KEYS = new Set([
  'ArrowLeft',
  'ArrowRight',
  'ArrowUp',
  'ArrowDown',
  '+',
  '=',
  '-',
  '[',
  ']',
  '1',
  '2',
  '3',
  '4',
  '5',
  '6',
  '7',
  '8',
  '9',
  'f',
  'F',
  'a',
  'A',
  'z',
  'Z',
]);

const installControls = (
  ui: UiHandles,
  callbacks: ControlCallbacks,
): (() => void) => {
  let state = createInitialControlState();

  const triggerFirework = (): void => callbacks.onEffect('firework');

  const handleKeyDown = (event: KeyboardEvent): void => {
    if (!HANDLED_KEYS.has(event.key)) {
      return;
    }
    event.preventDefault();

    if (event.key === 'f' || event.key === 'F') {
      if (!event.repeat) {
        triggerFirework();
      }
      return;
    }

    const nextState = reduceControlState(state, event.key);
    if (nextState !== state) {
      state = nextState;
      callbacks.onStateChange(state);
    }
  };

  window.addEventListener('keydown', handleKeyDown);
  ui.fireworkButton.addEventListener('click', triggerFirework);
  callbacks.onStateChange(state);

  return (): void => {
    window.removeEventListener('keydown', handleKeyDown);
    ui.fireworkButton.removeEventListener('click', triggerFirework);
  };
};

export default installControls;
