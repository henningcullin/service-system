import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import path from 'path';

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [svelte()],
    resolve: {
        alias: {
            $components: path.resolve('src/lib/components'),
            $lib: path.resolve('src/lib'),
            $routes: path.resolve('src/routes'),
        },
    },
});
