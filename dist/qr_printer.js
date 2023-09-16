function print_qr_code(str) {
    let data = make_qr(str);
    let width = Math.round(Math.sqrt(data.length));
    const canvas = document.getElementById("qr_canvas");
    canvas.width = width;
    canvas.height = width;
    const ctx = canvas.getContext("2d");
    let imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
    for(let i = 0; i < data.length; i++) {
        imageData.data[i * 4] = data[i] == 1? 0 : 255;
        imageData.data[i * 4 + 1] = data[i] == 1? 0 : 255;
        imageData.data[i * 4 + 2] = data[i] == 1? 0 : 255;
        imageData.data[i * 4 + 3] = 255;
    }
    ctx.putImageData(imageData,0,0);
}