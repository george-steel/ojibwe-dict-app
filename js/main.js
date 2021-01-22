import {html, render, nothing} from 'https://unpkg.com/lit-html?module';

const searchOJBox = document.getElementById('search-oj');
const searchENBox = document.getElementById('search-en');
const resultsBox = document.getElementById('resultslist');

const worker = new Worker('js/worker.js');

searchOJBox.oninput = e => {
    searchENBox.value = "";
    worker.postMessage({
        action: 'search',
        oj: searchOJBox.value.trim(),
        en: ''
    });
}
searchENBox.oninput = e => {
    searchOJBox.value = "";
    worker.postMessage({
        action: 'search',
        en: searchENBox.value.trim(),
        oj: ''
    });
}

const resultRow = row => html`<tr>
    <td class='oj-meta'>${row.oj.meta}</td>
    <td class='oj-word'>${row.oj.word}${row.oj.suffix ? html`<ins class='oj-suffix'>${row.oj.suffix}</ins>` : nothing}</td>
    <td class='en-words'>${row.en.map(s => html`<li>${s}</li>`)}</td>
</tr>`;

const resultRows = results => html`${results.map(resultRow)}`;

worker.onmessage = e => {
    console.log(e);
    const rows = resultRows(e.data.searchResults);
    render(rows, resultsBox);
}
