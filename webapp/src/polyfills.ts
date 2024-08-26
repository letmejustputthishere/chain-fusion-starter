window.global = window.global ?? window;
window.process = window.process ?? { env: {} }; // Minimal process polyfill

export {};