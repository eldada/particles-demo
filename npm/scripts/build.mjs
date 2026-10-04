import { copyFile, mkdir } from 'node:fs/promises';
import { build, context } from 'esbuild';

const watch = process.argv.includes('--watch');

const copyStaticFiles = async () => {
  await mkdir('dist/assets', { recursive: true });
  await copyFile('index.html', 'dist/index.html');
  await copyFile('src/style.css', 'dist/style.css');
  await copyFile('favicon.svg', 'dist/favicon.svg');
  await copyFile('favicon.ico', 'dist/favicon.ico');
};

const options = {
  entryPoints: ['src/main.ts'],
  bundle: true,
  outfile: 'dist/assets/main.js',
  format: 'esm',
  platform: 'browser',
  target: 'es2022',
  sourcemap: true,
  minify: !watch,
  logLevel: 'info',
  plugins: [
    {
      name: 'copy-static-files',
      setup(buildResult) {
        buildResult.onEnd(async (result) => {
          if (result.errors.length === 0) {
            await copyStaticFiles();
          }
        });
      },
    },
  ],
};

if (watch) {
  const buildContext = await context(options);
  await buildContext.watch();
  const server = await buildContext.serve({
    host: '127.0.0.1',
    port: 5173,
    servedir: 'dist',
  });
  const host = server.hosts[0] ?? '127.0.0.1';
  console.log(`Dev server: http://${host}:${server.port}/`);
} else {
  await build(options);
}
