let dict = null;
const parseDict = input => input.trimEnd().split('\n').map(line => line.split('\t'));

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
    const results = searchDict(dict, searchOJ, searchEN);
    postMessage({searchResults: results});
}

onmessage = e => {
    if (e.data.action == 'search') {
        console.log(e.data);
        searchOJ = e.data.oj;
        searchEN = e.data.en;
        startSearch()
    }
    else if (e.data.action == 'load') {
        dict = parseDict(e.data.rawDict);
        startSearch();
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
