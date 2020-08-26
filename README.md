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
      <i>User guide</i>
    </a>
    <span> | </span>
    <a href="https://github.com/jabber-tools/gdf_translate/blob/readme/README-devnotes.md">
      Developer guide
    </a>
    <span> | </span>
    <a href="https://github.com/jabber-tools/gdf_translate/releases">
      Releases
    </a>
  </h3>
</div>

<br/>

[Introduction](#introduction)

[Where To Get Binaries](#where-to-get-binaries)

[Command Line Interface](#command-line-interface)

[Examples](#examples)


## Introduction
Google Translate is command line utility used for translation of Google DialogFlow agents. Ambition of this tool is to replace original Agent Toolkit that can (among other things) translate GDF agents. The advantages of this new tool (over agent toolkit) are following:

*	We are doing translation only, nothing else. Tool is because of that more intuitive, lightweight and easier to use.
*	Tool was written in technology that requires no underlying runtime. Original agent toolkit requires Node.JS to be installed, together with windows-build-tools package. Making this to work on corporate laptop with lot of restrictions is usually very challenging, especially for non-IT users. New tool is single executable (by default exe file for windows) which is not requiring anything else. Just download and use!
*	Translation of intent training phrases is done piece by piece and it does not require manual annotation of entities once translated. This greatly simplifies overall translation process. If original way of translating training phrases will be needed it can be implanted as well with possibility to specify which one to use during translation
*	Most important: apart from Google Translate API V2 we support V3 API as well! This is a real game changer since it enables batch translations. Basically instead of translating text (be it utterance, entity or response) one by one, i.e. by invoking thousands of separate HTTP transactions we can create CVS export which is uploaded to Google Servers which take care of translation. Translated CVS file is then downloaded and applied to agent. The benefit is clear: this approach is much less error prone and blazingly fast in comparison with V2 translation. No more 30 minutes spent by looking at translation progressing and then crashing due to network issue or Google quota being hit!


## Where To Get Binaries
TBD

## Command Line Interface
TBD

## Examples
TBD
