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

        let input = new Uint8Array(this.result);
        var inputBlob = new Blob([input], { type: "image/png" });
        var inputImageUrl = URL.createObjectURL(inputBlob);
        console.log('input image url ', inputImageUrl)

        imageInput.src = inputImageUrl;

        let output = wasm.create_ascii_art(input);
        var outputBlob = new Blob([output]);
        var outputImageUrl = URL.createObjectURL(outputBlob);
        imageOutput.src = outputImageUrl;
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
