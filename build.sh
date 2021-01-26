#! /bin/bash

wasm-pack build --target no-modules
cp -r js/ index.html style.css data/ out/
cp pkg/ojibwe_dictsearch.js pkg/ojibwe_dictsearch_bg.wasm out/pkg/
