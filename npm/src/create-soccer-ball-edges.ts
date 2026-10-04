type Point = readonly [number, number, number];

const PHI = (1 + Math.sqrt(5)) / 2;

const evenPermutations = ([x, y, z]: Point): Point[] => [
  [x, y, z],
  [y, z, x],
  [z, x, y],
];

const signedEvenPermutations = ([x, y, z]: Point): Point[] => {
  const points: Point[] = [];
  const xSigns = x === 0 ? [1] : [-1, 1];
  const ySigns = y === 0 ? [1] : [-1, 1];
  const zSigns = z === 0 ? [1] : [-1, 1];

  for (const xSign of xSigns) {
    for (const ySign of ySigns) {
      for (const zSign of zSigns) {
        points.push(...evenPermutations([x * xSign, y * ySign, z * zSign]));
      }
    }
  }

  return points;
};

const distance = (left: Point, right: Point): number =>
  Math.hypot(left[0] - right[0], left[1] - right[1], left[2] - right[2]);

const createSoccerBallEdges = (): Float32Array => {
  const vertices = [
    ...signedEvenPermutations([0, 1, 3 * PHI]),
    ...signedEvenPermutations([1, 2 + PHI, 2 * PHI]),
    ...signedEvenPermutations([2, 1 + 2 * PHI, PHI]),
  ];
  let shortest = Number.POSITIVE_INFINITY;
  const pairs: Array<[number, number, number]> = [];

  for (let start = 0; start < vertices.length; start += 1) {
    const origin = vertices[start];
    if (origin === undefined) {
      continue;
    }
    for (let end = start + 1; end < vertices.length; end += 1) {
      const target = vertices[end];
      if (target === undefined) {
        continue;
      }
      const span = distance(origin, target);
      pairs.push([start, end, span]);
      if (span < shortest) {
        shortest = span;
      }
    }
  }

  const connected = pairs.filter(([, , span]) => Math.abs(span - shortest) < 1e-4);
  let radius = 0;
  for (const vertex of vertices) {
    radius = Math.max(radius, Math.hypot(vertex[0], vertex[1], vertex[2]));
  }
  const scale = 0.5 / radius;
  const edges = new Float32Array(connected.length * 6);

  connected.forEach(([start, end], index) => {
    const origin = vertices[start];
    const target = vertices[end];
    if (origin === undefined || target === undefined) {
      return;
    }
    const offset = index * 6;
    edges[offset] = origin[0] * scale;
    edges[offset + 1] = origin[1] * scale;
    edges[offset + 2] = origin[2] * scale;
    edges[offset + 3] = target[0] * scale;
    edges[offset + 4] = target[1] * scale;
    edges[offset + 5] = target[2] * scale;
  });

  return edges;
};

export default createSoccerBallEdges;
