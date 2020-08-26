<img src="./examples/docs/img/rust-logo.png" align="right" />

# Google DialogFlow Translate

#### *delightful google dialogflow agent tRanslation*

Command line tool for automated translation of Google DialogFlow agents

---
[![made-with-Rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://GitHub.com/jabber-tools/gdf_translate/graphs/commit-activity)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://github.com/jabber-tools/gdf_translate/blob/readme/LICENSE-APACHE)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/jabber-tools/gdf_translate/blob/readme/LICENSE-MIT)

<div align="center">
  <h3>
    <a href="https://github.com/jabber-tools/gdf_translate/blob/readme/README.md">
      User guide
    </a>
    <span> | </span>
    <a href="https://github.com/jabber-tools/gdf_translate/blob/readme/README-devnotes.md">
      <i>Developer guide</i>
    </a>
    <span> | </span>
    <a target="_blank" href="https://github.com/jabber-tools/gdf_translate/releases">
      API Docs
    </a>
    <span> | </span>
    <a href="https://github.com/jabber-tools/gdf_translate/releases">
      Releases
    </a>
  </h3>
</div>

<br/>


## How does it work?

First Google Dialogflow agent is exported into ZIP file and this file is provided to translation utility. Translation utility parses the file and deserialises its content into internal structures stored in memory.</br>
<img width="600" height="200" src="./examples/docs/img/zip-to-struct.png" /></br>

In fact structures themselves are stored on stack whereas its content is stored on heap. Each entry on heap has its address (referred from stack) and data/content (e.g. string that needs to be translated!)</br>
<img width="400" height="600" src="./examples/docs/img/stack-and-heap.png" /></br>

Translation utility traverses all structures created in deserialization step and creates table to be translated. This table (hashmap) has original heap address as a key and data/content as a value.</br>

| Address      |      Data      |
|--------------|:--------------:|
| 7f06092ac6d4 |  Germany       |
| 7f06092ac6d1 |    Hello       |
| 7f06092ac6d2 | Feels Rusty    |
|7f06092ac6d0  |This is response|


Table (i.e. data column) is translated. Two approaches are used:
<ul>
  <li>Google V2 translation API
    <ul>
      <li>Each row is translated as separate HTTP transaction</li>
    </ul>
  </li>
  <li>Google V3 translation API
    <ul>
      <li>Hashmap is converted to CSV file</li>
      <li>CSV file is uploaded into Google Cloud Storage Bucket</li>
      <li>Batch translation is started</li>
      <li>Batch translation status is checked regularly up to the point where result is produced again as CSV file in Google Cloud Storage Bucket</li>
      <li>Output Google Cloud Storage Bucket content is downloaded and transformed from CVS file into hashmap again. Something like:</br>
                <table>
                  <tbody>
                    <tr>
                      <th align="center">Address</th>
                      <th align="center">Data</th>
                    </tr>
                    <tr>
                      <td>7f06092ac6d4</td>
                      <td align="center">Deutschland</td>
                    </tr>
                    <tr>
                      <td>7f06092ac6d1</td>
                      <td align="center">Hallo</td>
                    </tr>
                    <tr>
                      <td>7f06092ac6d2</td>
                      <td align="center">Fühlt sich rostig an</td>
                    </tr>
                    <tr>
                      <td>7f06092ac6d0</td>
                      <td align="center">Dies ist eine Antwort</td>
                    </tr>
                  </tbody>
                </table>        
      </li>
      <li>Agent structure is traversed again (same as when creating original translation table/hashmap). For every address we are doing lookup (by address) into translated hashmap and replacing the value accordingly</li>
      <li>Agent is then serialized and packed into ZIP file.</li>
    </ul>
  </li>
</ul>
 
| Address      |      Data            |
|--------------|:--------------------:|
| 7f06092ac6d4 |  Deutschland         |
| 7f06092ac6d1 |    Hallo             |
| 7f06092ac6d2 | Fühlt sich rostig an |
|7f06092ac6d0  | Dies ist eine Antwort|       










