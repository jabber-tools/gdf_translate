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

[Service Account Requirements](#service-account-requirements)

[Command Line Interface](#command-line-interface)

[Issues](#issues)

[Examples](#examples)


## Introduction
Google Translate is command line utility used for translation of Google DialogFlow agents. Ambition of this tool is to replace original Agent Toolkit that can (among other things) translate GDF agents. The advantages of this new tool (over agent toolkit) are following:

*	We are doing translation only, nothing else. Tool is because of that more intuitive, lightweight and easier to use.
*	Tool was written in technology that requires no underlying runtime. Original agent toolkit requires [Node.JS](https://nodejs.org/en/) to be installed, together with [windows-build-tools](https://www.npmjs.com/package/windows-build-tools) npm package. Making this to work on corporate laptop with lot of restrictions is usually very challenging, especially for non-IT users. New tool is single executable (by default exe file for windows) which is not requiring anything else. Just download and use!
*	Translation of intent training phrases is done piece by piece and it does not require manual annotation of entities once translated. This greatly simplifies overall translation process. If original way of translating training phrases will be needed it can be implanted as well with possibility to specify which one to use during translation
*	<b>Most important</b>: apart from Google Translate API V2 (basic) we support V3 API (advanced) as well! For details have a look at [Google Cloud Translation Documentation](https://cloud.google.com/translate/docs/editions). This is a real game changer since it enables <b>batch translations</b>. Basically instead of translating text (be it utterance, entity or response) one by one, i.e. by invoking thousands of separate HTTP transactions, we can create CVS export and upload it to Google Servers which take care of the translation. Translated CVS file is then downloaded and applied to agent. The benefit is clear: this approach is much less error prone and <b>blazingly fast</b> in comparison with V2 translation. No more 30 minutes spent by looking at translation progressing and then crashing due to network issue or Google quota being hit! More technical explanation can be found in [Developer guide](https://github.com/jabber-tools/gdf_translate/blob/readme/README-devnotes.md)


## Where To Get Binaries
Binaries are published under [Releases](https://github.com/jabber-tools/gdf_translate/releases) section of this github repository. Make sure to always use the latest release in order to have latest fixes and features amendments.

## Service Account Requirements
We are using service accounts while interacting with Google Translation APIs. 

[IAM](https://cloud.google.com/translate/docs/intro-to-v3#iam)

[SVC ACC](https://cloud.google.com/iam/docs/creating-managing-service-accounts)

[SVC ACC KEYS](https://cloud.google.com/iam/docs/creating-managing-service-account-keys)

## Command Line Interface
Simply ask for help:
```
C:\tmp>gdf_translate.exe -h
Google DialogFlow Translate 0.1.0
Adam Bezecny
Tool for automated translation of Google DialogFlow agents.

USAGE:
    gdf_translate.exe [FLAGS] [OPTIONS] --source-lang <lang ISO code> --cred-file <FILE> --agent-file <FILE> --output-folder <FOLDER> --target-lang <lang ISO code>

FLAGS:
    -d, --create-output-tsv    If this flag is specified it will preserve for V3 API downloaded output buckets. This is
                               primarily intented for debugging, no need to specify by ordinary users. For V2 API this
                               flag is ignored.
    -h, --help                 Prints help information
    -V, --version              Prints version information

OPTIONS:
    -s, --source-lang <lang ISO code>    ISO code of source language.E.g.: en
    -c, --cred-file <FILE>               Path to Google Cloud service account credentials used to run translation via
                                         Google Translate V2/V3 API. Must have respective priviledges: TBD...
    -f, --agent-file <FILE>              ZIP file with exported GDF agent
    -o, --output-folder <FOLDER>         Path to folder where translated agent will be stored. Must be exiting (ideally
                                         empty) folder.
    -t, --target-lang <lang ISO code>    ISO code of destination/target language to which agent will be translated
                                         .E.g.: de
    -a, --api-version <v2/v3>            Version of API used to translate. Can be v2/v3. If not specified defaults to
                                         v3. [default: v3]  [possible values: v2, v3, V2, V3]
    -p, --task-count <INTEGER>           Number of asynchronous and parallel tasks that will be used to call Google V2
                                         translation API. If not specified defaults to 10. Ignored when using V3 API.
                                         [default: 10]

C:\tmp>

```

## Issues
TBD

## Examples
TBD
