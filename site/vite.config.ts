import { sentrySvelteKit } from "@sentry/sveltekit";
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import wasmPack from 'vite-plugin-wasm-pack';

export default defineConfig({
	plugins: [
        sentrySvelteKit({
        sourceMapsUploadOptions: {
            org: "jupiterp",
            project: "jupiterp"
        }
        }), 
        sveltekit(),
        wasmPack('../rust-lib')
    ],
    server: {
        fs: {
            allow: ['..'],
        }
    }
});