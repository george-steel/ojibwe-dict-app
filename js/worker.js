importScripts('../pkg/ojibwe_dictsearch.js');

const mwasm = wasm_bindgen('../pkg/ojibwe_dictsearch_bg.wasm');
const mrawdict = fetch('../data/dictionary.tsv').then(r => r.arrayBuffer())

mrawdict.then(() => console.log("Fetched the dictionary"));

let dict = null;

Promise.all([mwasm, mrawdict]).then(([wasm, rawdict]) => {
    console.log(`Begin loading dictionary`);
    dict = wasm_bindgen.parse_dict(new Uint8Array(rawdict));
    console.log(`Loaded the dictionary with ${dict.size()} entries`);
    startSearch();
});

let search = null;
let searchTriggered = false;
function startSearch() {
    if (!searchTriggered) {
        searchTriggered = true;
        setTimeout(runSearch, 0);
    }
}
function runSearch() {
    searchTriggered = false;
    if (!dict) return;

    let results = null;
    if (search == null)
        results = dict.search_en_js("");
    else if (search.action == "search-oj")
        results = dict.search_oj_js(search.query, search.mode);
    else if (search.action == "search-en")
        results = dict.search_en_js(search.query);
    postMessage({searchResults: results});
}

onmessage = e => {
    console.log(e.data);
    if (e.data.action === "dump") {
        let buf = dict.to_tsv();
        postMessage({wholedict: buf});
    } else {
        search = e.data;
        startSearch();
    }
};

