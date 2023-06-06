const fs = require('fs');
const path = require('path');
const convert = require('fbx2gltf');

const fbxDirectory = './assets/fbx_files/';
const gltfDirectory = './assets/alt_models/';

fs.readdir(fbxDirectory, (err, files) => {
    if (err) {
        console.error('Could not list the directory.', err);
        process.exit(1);
    }

    files.forEach((file, index) => {
        if (path.extname(file) === '.fbx') {
            const fbxFilePath = path.join(fbxDirectory, file);
            const gltfFilePath = path.join(gltfDirectory, path.basename(file, '.fbx') + '.glb');

            convert(fbxFilePath, gltfFilePath, ['--khr-materials-unlit']).then(
                destPath => {
                    console.log('Conversion successful! Output file:', destPath);
                },
                error => {
                    console.error('Conversion failed:', error);
                }
            );
        }
    });
});
