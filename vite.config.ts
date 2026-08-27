import * as vite from "vite";
import * as svelte from "@sveltejs/vite-plugin-svelte";
import * as path from "path";

export default vite.defineConfig({
    plugins: [svelte.svelte()],
    resolve: {
        alias: {
            "pdfjs-dist": path.resolve(__dirname, "node_modules/pdfjs-dist"),
        },
    },
    optimizeDeps: {
        include: ["fp-ts", "fuse.js"],
    },
    build: {
        rollupOptions: {
            output: {
                manualChunks(id: string): string {
                    if (id.includes("pdfjs-dist")) {
                        return "pdfjs";
                    }
                    return "";
                },
            },
        },
    },
});
