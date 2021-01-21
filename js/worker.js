importScripts('../pkg/ojibwe_dictsearch.js');

const mwasm = wasm_bindgen('../pkg/ojibwe_dictsearch_bg.wasm');
const mrawdict = fetch('../data/dictionary.tsv').then(r => r.arrayBuffer())

let dict = null;

Promise.all([mwasm, mrawdict]).then(([wasm, rawdict]) => {
    dict = wasm_bindgen.parse_dict(new Uint8Array(rawdict));
    startSearch();
});

let searchOJ = "";
let searchEN = "";
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
    if (searchOJ)
        results = dict.search_oj_js(searchOJ);
    else
        results = dict.search_en_js(searchEN);
    postMessage({searchResults: results});
}

onmessage = e => {
    if (e.data.action == 'search') {
        console.log(e.data);
        searchOJ = e.data.oj;
        searchEN = e.data.en;
        startSearch()
    }
};

////////////////////////////////////////////////////////////////////////////////

function searchDict(entries, queryOJ, queryEN) {
    const maxResults = 100;
    let results = [];
    for ([meta, oj, en] of entries) {
        if (queryOJ && !oj.includes(queryOJ)) continue;
        if (queryEN && !en.includes(queryEN)) continue;
        results.push({meta,oj,en});
        if (results.length >= maxResults) break;
    }
    return results;
}
