import {html, render} from 'https://unpkg.com/lit-html?module';

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
    <td>${row.meta}</td>
    <td>${row.oj}</td>
    <td>${row.en}</td>
</tr>`;

const resultRows = results => html`${results.map(resultRow)}`;

worker.onmessage = e => {
    console.log(e);
    const rows = resultRows(e.data.searchResults);
    render(rows, resultsBox);
}
