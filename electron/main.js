const rust = require('neon_bindings');

// Text example
document.getElementById('rust-text').innerText = rust.hello_world();

// Canvas example
let ctx = document.getElementById('rust-canvas').getContext('bitmaprenderer');

let imageBuffer = new Uint8ClampedArray(rust.render_image());
let imageData = new ImageData(imageBuffer, 800);

createImageBitmap(imageData).then((bitmap) => ctx.transferFromImageBitmap(bitmap));
