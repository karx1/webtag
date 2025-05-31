function arrayFromArrayBuffer(buf) {
    let uint8Array = new Uint8Array(buf);
    let arr = Array.from(uint8Array);
    return arr;
}

function clearInput(id) {
    document.getElementById(id).value = null;
}

function downloadFile(download, filename) {
    const anchor = document.getElementById("download_anchor");
    anchor.href = download;
    anchor.download = filename;
    anchor.click();
}
