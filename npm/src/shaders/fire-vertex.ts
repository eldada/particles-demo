const fireVertexShader = /* glsl */ `
  attribute vec4 aSeed;

  uniform float uTime;
  uniform float uPixelRatio;
  uniform float uExplosion;
  uniform float uCubeLimit;
  uniform vec2 uDirection;

  varying float vLife;

  vec3 randomDirection(vec3 seed) {
    float z = seed.x * 2.0 - 1.0;
    float angle = seed.y * 6.28318530718;
    float radius = sqrt(max(0.0, 1.0 - z * z));
    return vec3(radius * cos(angle), z, radius * sin(angle));
  }

  void main() {
    float speed = mix(0.12, 0.28, aSeed.w);
    float life = fract(aSeed.x + uTime * speed);
    vec3 heading = randomDirection(aSeed.xyz);
    float reach = mix(0.35, 1.55, aSeed.z);
    vec3 flame = heading * reach * life;

    float curl = sin(uTime * 1.2 + life * 9.0 + aSeed.y * 12.0);
    float curlTwo = cos(uTime * 0.85 + life * 7.0 + aSeed.z * 14.0);
    flame += heading.yzx * curl * 0.16 * life;
    flame += heading.zxy * curlTwo * 0.12 * life;
    flame += vec3(uDirection, 0.0) * life * 2.4;

    vec3 burstDirection = randomDirection(aSeed.xyz);
    vec3 launched = flame + burstDirection * mix(1.8, 5.8, aSeed.w);
    float launchedLength = length(launched);
    if (launchedLength > uCubeLimit) {
      launched *= uCubeLimit / launchedLength;
    }
    flame = mix(flame, launched, uExplosion);

    vec4 modelPosition = modelMatrix * vec4(flame, 1.0);
    vec4 viewPosition = viewMatrix * modelPosition;
    float viewDistance = max(-viewPosition.z, 0.001);
    float distanceScale = 1.0 / max(viewDistance, 11.4);

    gl_Position = projectionMatrix * viewPosition;
    gl_PointSize = 6.0 * uPixelRatio * distanceScale * 4.0;

    vLife = life;
  }
`;

export default fireVertexShader;
