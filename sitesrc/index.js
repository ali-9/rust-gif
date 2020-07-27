const gif = import('../mylib/pkg/mylib.js');

var canvas = document.getElementById('canvas');
var ctx = canvas.getContext('2d');
ctx.fillStyle = "red";
ctx.fillRect(10, 10, 50, 50);

const sleep = (m) => new Promise((r) => setTimeout(r, m))

fetch('test1.gif')
  .then(function (response) {
    if (!response.ok) {
      throw new Error("HTTP error, status = " + response.status);
    }
    return response.arrayBuffer();
  })
  .then(function (buffer) {
    console.log("gifjs", buffer)
    const v = new Uint8Array(buffer)
    gif.then((async _gif => {
      console.time("gifTime")
      const r = _gif.test(v)
      console.log("r", r)
      console.timeEnd("gifTime")

      const frames_info = r[r.length - 1]
      const width = frames_info[frames_info.length-1].gif_width;
      const height = frames_info[frames_info.length-1].gif_height;
      canvas.width = width
      canvas.height = height
     
     while(true){
      for (let i = 0; i < r.length - 1; i++) {

        const textureRaw = new Uint8ClampedArray(r[i]) // new Uint8ClampedArray(memory.buffer, texture.offset(), texture.size());

        const delay = frames_info[i].delay * 10


        const image = new ImageData(textureRaw, width, height);
        ctx.putImageData(image, 0, 0);
        await sleep(delay)
      }
    }

      //  console.log(r)
    }))
  })