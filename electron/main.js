const rust = require('neon_bindings');

let root = document.getElementById('root');
root.innerHTML = rust.hello();
