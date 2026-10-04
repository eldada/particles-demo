const fireworkLimit = (cubeSize: number, particleScale: number): number => {
  if (particleScale <= 0) {
    return 0;
  }

  return cubeSize / (2 * particleScale);
};

export default fireworkLimit;
