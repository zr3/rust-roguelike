import { defineConfig } from "vite"
import { viteSingleFile } from "vite-plugin-singlefile"

export default defineConfig({
    plugins: [viteSingleFile()],
    server: {
        proxy: {
            '/api': {
                target: 'https://wild-thyme.zakreynolds.dev',
                changeOrigin: true,
            },
        },
    },
})
