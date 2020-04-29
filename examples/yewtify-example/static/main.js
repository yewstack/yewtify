import init, { run_app } from './yewtify_example.js';
async function main() {
   await init('./yewtify_example_bg.wasm');
   run_app();
}
main()
