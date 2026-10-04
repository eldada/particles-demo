type RandomSource = () => number;

const createParticleSeeds = (
  count: number,
  random: RandomSource = Math.random,
): Float32Array => {
  const seeds = new Float32Array(Math.max(0, count) * 4);
  for (let index = 0; index < seeds.length; index += 1) {
    seeds[index] = random();
  }
  return seeds;
};

export default createParticleSeeds;
