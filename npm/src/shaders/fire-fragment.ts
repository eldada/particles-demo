const fireFragmentShader = /* glsl */ `
  uniform float uVisibility;

  varying float vLife;

  void main() {
    vec2 centered = gl_PointCoord - 0.5;
    float radius = length(centered) * 2.0;
    float dot = smoothstep(1.0, 0.9, radius);
    float fade = smoothstep(1.0, 0.70, vLife);

    vec3 ember = vec3(0.23, 0.025, 0.006);
    vec3 amber = vec3(0.50, 0.15, 0.025);
    vec3 color = mix(amber, ember, vLife);

    float alpha = dot * fade * uVisibility;
    if (alpha < 0.012) discard;
    gl_FragColor = vec4(color, alpha);
  }
`;

export default fireFragmentShader;
