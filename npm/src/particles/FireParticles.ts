import {
  AdditiveBlending,
  BufferAttribute,
  BufferGeometry,
  Points,
  ShaderMaterial,
  Vector2,
} from 'three';
import burstOpacity from '../burst-opacity.js';
import burstTravel from '../burst-travel.js';
import effectEnvelope from '../effect-envelope.js';
import fireworkLimit from '../firework-limit.js';
import fireFragmentShader from '../shaders/fire-fragment.js';
import fireVertexShader from '../shaders/fire-vertex.js';
import type { Direction, EffectName } from '../types.js';
import createParticleSeeds from './create-particle-seeds.js';

const MAXIMUM_PARTICLES = 6000;
const FIREWORK_DURATION = 2.8;

class FireParticles {
  readonly points: Points<BufferGeometry, ShaderMaterial>;

  private readonly direction = new Vector2();
  private readonly targetDirection = new Vector2();
  private activeEffect: EffectName = 'idle';
  private effectElapsed = 0;
  private bloomStrength = 0;
  private recovering = false;
  private recoverElapsed = 0;

  constructor(pixelRatio: number) {
    const geometry = new BufferGeometry();
    geometry.setAttribute(
      'position',
      new BufferAttribute(new Float32Array(MAXIMUM_PARTICLES * 3), 3),
    );
    geometry.setAttribute(
      'aSeed',
      new BufferAttribute(createParticleSeeds(MAXIMUM_PARTICLES), 4),
    );
    geometry.setDrawRange(0, 2800);

    const material = new ShaderMaterial({
      uniforms: {
        uTime: { value: 0 },
        uPixelRatio: { value: pixelRatio },
        uExplosion: { value: 0 },
        uCubeLimit: { value: fireworkLimit(2.8, 0.5) },
        uVisibility: { value: 1 },
        uDirection: { value: this.direction },
      },
      vertexShader: fireVertexShader,
      fragmentShader: fireFragmentShader,
      transparent: true,
      depthWrite: false,
      blending: AdditiveBlending,
    });

    this.points = new Points(geometry, material);
    this.points.scale.setScalar(0.5);
    this.points.frustumCulled = false;
  }

  setCount(count: number): void {
    this.points.geometry.setDrawRange(
      0,
      Math.min(MAXIMUM_PARTICLES, Math.max(0, Math.round(count))),
    );
  }

  setDirection(direction: Direction): void {
    this.targetDirection.set(direction.x, direction.y);
  }

  setCubeSize(cubeSize: number): void {
    this.points.material.uniforms.uCubeLimit = {
      value: fireworkLimit(cubeSize, this.points.scale.x),
    };
  }

  setPixelRatio(pixelRatio: number): void {
    this.points.material.uniforms.uPixelRatio = { value: pixelRatio };
  }

  trigger(effect: Exclude<EffectName, 'idle'>): void {
    this.activeEffect = effect;
    this.effectElapsed = 0;
    this.recovering = false;
    this.recoverElapsed = 0;
  }

  update(delta: number, elapsed: number): EffectName {
    this.direction.lerp(this.targetDirection, 1 - Math.exp(-delta * 3.8));
    this.points.material.uniforms.uTime = { value: elapsed };

    let explosion = 0;
    let visibility = 1;
    this.bloomStrength = 0;
    if (this.activeEffect !== 'idle') {
      this.effectElapsed += delta;
      const travel = burstTravel(this.effectElapsed, FIREWORK_DURATION);
      visibility = burstOpacity(this.effectElapsed, FIREWORK_DURATION);
      this.bloomStrength = effectEnvelope(this.effectElapsed, FIREWORK_DURATION);
      explosion = travel;
      if (this.effectElapsed >= FIREWORK_DURATION) {
        this.activeEffect = 'idle';
        this.recovering = true;
        this.recoverElapsed = 0;
        explosion = 0;
        visibility = 0;
        this.bloomStrength = 0;
      }
    } else if (this.recovering) {
      this.recoverElapsed += delta;
      const progress = Math.min(1, this.recoverElapsed / 0.7);
      visibility = progress * progress * (3 - 2 * progress);
      if (this.recoverElapsed >= 0.7) {
        this.recovering = false;
        visibility = 1;
      }
    }

    this.points.material.uniforms.uExplosion = { value: explosion };
    this.points.material.uniforms.uVisibility = { value: visibility };
    return this.activeEffect;
  }

  getEffectStrength(): number {
    return this.bloomStrength;
  }

  dispose(): void {
    this.points.geometry.dispose();
    this.points.material.dispose();
  }
}

export default FireParticles;
