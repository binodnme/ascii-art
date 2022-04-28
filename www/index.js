import * as wasm from "ascii-art";

const fileInput = document.getElementById("file-input");
let imageInput = document.getElementById("input");
let imageOutput = document.getElementById("output");


fileInput.addEventListener('change', (e) => {
    console.log(e)
    let files = fileInput.files;
    // Pass the file to the blob, not the input[0].
    let file = files[0];
    var fileReader = new FileReader();

    // debugger;
    fileReader.onload = function (event1) {
        let input = new Uint8Array(event1.target.result);
        let inputBlob = new Blob([input]);
        let inputImageUrl = URL.createObjectURL(inputBlob);

        imageInput.src = inputImageUrl;

        try {
            let extension = file.name.split('.').slice(-1)[0]
            let output = ''
            if (extension === 'png') {
                output = wasm.create_ascii_art_from_png(input);
            } else if (extension === 'gif') {
                output = wasm.create_ascii_art_from_gif(input);
            } else {
                console.log('file format not supported')
                return
            }
            let outputBlob = new Blob([output]);
            let outputImageUrl = URL.createObjectURL(outputBlob);
            imageOutput.src = outputImageUrl;
        } catch (e) {
            console.log(e)
        }
    }
    fileReader.readAsArrayBuffer(file);

})

function loadImage(context, src) {
    var img = new Image();
    img.src = src;

    img.onload = function () {
        context.drawImage(img, 0, 0, 500, 500);
    }
}

function getBuffer(fileData) {
    return function (resolve) {
        var reader = new FileReader();
        reader.readAsArrayBuffer(fileData);
        reader.onload = function () {
            var arrayBuffer = reader.result
            var bytes = new Uint8Array(arrayBuffer);
            resolve(bytes);
        }
    }
}
