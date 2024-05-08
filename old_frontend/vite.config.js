import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { viteStaticCopy } from 'vite-plugin-static-copy'
import path from 'path';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    svelte(),
    viteStaticCopy({
      targets: [
        {
          src: 'src/assets/themes/[!.]*',
          dest: 'assets/themes',
        }
      ]
    })
  ],
  resolve: {
    alias: {
      $lib: path.resolve('src/lib'),
    },
  },
})
