import { sveltekit } from '@sveltejs/kit/vite';
import { execSync } from 'child_process';

const COMMIT_SHA =
  process.env.VITE_COMMIT_SHA ||
  execSync('git describe --always').toString().trim();

/** @type {import('vite').UserConfig} */
const config = {
  build: {
    sourcemap: true,
  },
  define: {
    __BUILD_DATE__: JSON.stringify(+new Date()),
    __COMMIT_SHA__: JSON.stringify(COMMIT_SHA),
    __VERSION__: JSON.stringify(process.env.npm_package_version),
  },
  server: {
    port: 3000,
  },
  plugins: [sveltekit()],
};

export default config;
