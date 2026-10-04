import installControls from './controls.js';
import ParticleShow from './ParticleShow.js';
import createUi, { updateUiEffect, updateUiState } from './ui.js';

const app = document.querySelector('#app');

if (!(app instanceof HTMLElement)) {
  throw new Error("Application root '#app' was not found.");
}

try {
  const ui = createUi(app);
  const show = new ParticleShow(app);
  let displayedEffect = show.getActiveEffect();

  const removeControls = installControls(ui, {
    onStateChange: (state) => {
      show.applyControlState(state);
      updateUiState(ui, state);
    },
    onEffect: (effect) => {
      show.triggerEffect(effect);
      updateUiEffect(ui, effect);
      displayedEffect = effect;
    },
  });

  const syncEffectLabel = (): void => {
    const activeEffect = show.getActiveEffect();
    if (activeEffect !== displayedEffect) {
      displayedEffect = activeEffect;
      updateUiEffect(ui, activeEffect);
    }
    window.requestAnimationFrame(syncEffectLabel);
  };

  show.start();
  window.requestAnimationFrame(syncEffectLabel);

  window.addEventListener(
    'beforeunload',
    () => {
      removeControls();
      show.dispose();
    },
    { once: true },
  );
} catch (error: unknown) {
  const failure = document.createElement('p');
  failure.className = 'render-error';
  failure.textContent = 'Unable to start the particle show.';
  app.replaceChildren(failure);
  console.error('Particle show initialization failed.', error);
}
