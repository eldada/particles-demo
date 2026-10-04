import {
  ACESFilmicToneMapping,
  BoxGeometry,
  BufferAttribute,
  BufferGeometry,
  Clock,
  Color,
  EdgesGeometry,
  FogExp2,
  LineBasicMaterial,
  LineSegments,
  MathUtils,
  PerspectiveCamera,
  Scene,
  SRGBColorSpace,
  WebGLRenderer,
} from 'three';
import type { ControlState } from './control-state.js';
import createSoccerBallEdges from './create-soccer-ball-edges.js';
import FireParticles from './particles/FireParticles.js';
import type { EffectName } from './types.js';

const MAXIMUM_PIXEL_RATIO = 2;
const BALL_TO_CUBE = 0.68;

class ParticleShow {
  private readonly renderer: WebGLRenderer;
  private readonly scene = new Scene();
  private readonly camera = new PerspectiveCamera(42, 1, 0.1, 100);
  private readonly fire: FireParticles;
  private readonly cube: LineSegments;
  private readonly cubeGlow: LineSegments;
  private readonly ball: LineSegments;
  private readonly clock = new Clock();
  private targetCubeSize = 2.8;
  private targetRotationSpeed = 0.14;
  private rotationSpeed = 0.14;
  private targetCameraDistance = 11.4;
  private cameraDistance = 11.4;
  private elapsed = 0;
  private animationFrame = 0;
  private suspended = false;
  private effect: EffectName = 'idle';

  constructor(container: HTMLElement) {
    this.renderer = new WebGLRenderer({
      antialias: true,
      alpha: true,
      powerPreference: 'high-performance',
    });
    this.renderer.setPixelRatio(this.getPixelRatio());
    this.renderer.outputColorSpace = SRGBColorSpace;
    this.renderer.toneMapping = ACESFilmicToneMapping;
    this.renderer.toneMappingExposure = 1.08;
    container.prepend(this.renderer.domElement);

    this.scene.background = new Color(0x030407);
    this.scene.fog = new FogExp2(0x05070b, 0.028);
    this.camera.position.set(0, 0.35, 11.4);

    this.fire = new FireParticles(this.getPixelRatio());
    this.scene.add(this.fire.points);

    const edges = new EdgesGeometry(new BoxGeometry(1, 1, 1));
    this.cube = new LineSegments(
      edges,
      new LineBasicMaterial({
        color: 0xd49a72,
        transparent: true,
        opacity: 0.38,
      }),
    );
    this.cube.scale.setScalar(this.targetCubeSize);
    this.scene.add(this.cube);

    this.cubeGlow = new LineSegments(
      edges.clone(),
      new LineBasicMaterial({
        color: 0xff5f22,
        transparent: true,
        opacity: 0.08,
      }),
    );
    this.cubeGlow.scale.setScalar(this.targetCubeSize * 1.012);
    this.scene.add(this.cubeGlow);

    const ballEdges = new BufferGeometry();
    ballEdges.setAttribute(
      'position',
      new BufferAttribute(createSoccerBallEdges(), 3),
    );
    this.ball = new LineSegments(
      ballEdges,
      new LineBasicMaterial({
        color: 0xd7e4f2,
        transparent: true,
        opacity: 0.55,
      }),
    );
    this.ball.scale.setScalar(this.targetCubeSize * BALL_TO_CUBE);
    this.scene.add(this.ball);

    window.addEventListener('resize', this.handleResize);
    document.addEventListener('visibilitychange', this.handleVisibility);
    this.handleResize();
  }

  start(): void {
    this.clock.start();
    this.animationFrame = window.requestAnimationFrame(this.animate);
  }

  applyControlState(state: ControlState): void {
    this.fire.setCount(state.particleCount);
    this.fire.setDirection(state.direction);
    this.targetCubeSize = state.cubeSize;
    this.fire.setCubeSize(state.cubeSize);
    this.targetRotationSpeed = state.rotationSpeed;
    this.targetCameraDistance = state.cameraDistance;
  }

  triggerEffect(effect: Exclude<EffectName, 'idle'>): void {
    this.effect = effect;
    this.fire.trigger(effect);
  }

  getActiveEffect(): EffectName {
    return this.effect;
  }

  dispose(): void {
    window.cancelAnimationFrame(this.animationFrame);
    window.removeEventListener('resize', this.handleResize);
    document.removeEventListener('visibilitychange', this.handleVisibility);
    this.fire.dispose();
    this.cube.geometry.dispose();
    this.cubeGlow.geometry.dispose();
    this.ball.geometry.dispose();
    this.renderer.dispose();
  }

  private readonly animate = (): void => {
    this.animationFrame = window.requestAnimationFrame(this.animate);
    if (this.suspended) {
      return;
    }

    const delta = Math.min(this.clock.getDelta(), 0.05);
    this.elapsed += delta;
    this.effect = this.fire.update(delta, this.elapsed);
    const effectStrength = this.fire.getEffectStrength();

    const scaleAlpha = 1 - Math.exp(-delta * 4.2);
    const cubeSize = MathUtils.lerp(
      this.cube.scale.x,
      this.targetCubeSize,
      scaleAlpha,
    );
    this.cube.scale.setScalar(cubeSize);
    this.fire.setCubeSize(cubeSize);
    this.cubeGlow.scale.setScalar(cubeSize * (1.012 + effectStrength * 0.018));
    this.ball.scale.setScalar(cubeSize * BALL_TO_CUBE);

    this.rotationSpeed = MathUtils.lerp(
      this.rotationSpeed,
      this.targetRotationSpeed,
      1 - Math.exp(-delta * 3),
    );
    const turn = this.rotationSpeed * delta;
    this.cube.rotation.x += turn * 0.62;
    this.cube.rotation.y += turn;
    this.cube.rotation.z += turn * 0.18;
    this.cubeGlow.rotation.copy(this.cube.rotation);
    this.ball.rotation.x += delta * 0.16;
    this.ball.rotation.y += delta * 0.46;

    this.cameraDistance = MathUtils.lerp(
      this.cameraDistance,
      this.targetCameraDistance,
      1 - Math.exp(-delta * 4),
    );
    this.camera.position.y = 0.35;
    this.camera.position.z = this.cameraDistance;
    this.camera.lookAt(0, 0.25, 0);
    this.renderer.render(this.scene, this.camera);
  };

  private readonly handleResize = (): void => {
    const width = window.innerWidth;
    const height = window.innerHeight;
    const pixelRatio = this.getPixelRatio();
    this.camera.aspect = width / Math.max(1, height);
    this.camera.updateProjectionMatrix();
    this.renderer.setPixelRatio(pixelRatio);
    this.renderer.setSize(width, height, false);
    this.fire.setPixelRatio(pixelRatio);
  };

  private readonly handleVisibility = (): void => {
    this.suspended = document.hidden;
    if (!this.suspended) {
      this.clock.getDelta();
    }
  };

  private getPixelRatio(): number {
    return Math.min(window.devicePixelRatio, MAXIMUM_PIXEL_RATIO);
  }
}

export default ParticleShow;
