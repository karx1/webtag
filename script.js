function arrayFromArrayBuffer(buf) {
    let uint8Array = new Uint8Array(buf);
    let arr = Array.from(uint8Array);
    return arr;
}
