import {html, render, nothing} from 'https://unpkg.com/lit-html?module';

const elem = (id) => document.getElementById(id);

const search_worker = new Worker('js/worker.js');
console.log("worker", search_worker);

function search_oj() {
    elem('en-query').value = "";
    const query = elem('oj-query').value.trim();
    if (!query) return;
    const mode = parseInt(elem('oj-mode').value);
    search_worker.postMessage({action: 'search-oj', query, mode});
}

function search_en() {
    elem('oj-query').value = "";
    const query = elem('en-query').value.trim();
    search_worker.postMessage({action: 'search-en', query});
}

elem('oj-query').oninput = search_oj;
elem('oj-mode').oninput = search_oj;
elem('en-query').oninput = search_en;

const resultRow = row => html`<li>
    <p class='oj-term'>
        <span class='oj-rm'>${row.oj.word}${row.oj.suffix ? html`<ins class='oj-suffix'>+${row.oj.suffix}</ins>` : nothing}</span>
        <span class='oj-syll'>${row.syllabics}</span>
        <span class='oj-meta'>${row.oj.meta}</span>
    </p>
    <ul class='en-words'>${row.en.map(s => html`<li>${s}</li>`)}</ul>
</li>`;

const resultRows = results => html`${results.map(resultRow)}`;

search_worker.onmessage = e => {
    console.log(e);
    if (e.data.wholedict) {
        console.log(e.data.wholedict);
        return;
    }
    const rows = resultRows(e.data.searchResults);
    render(rows, elem('results'));
}
