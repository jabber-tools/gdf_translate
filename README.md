<img src="./examples/docs/img/rust-logo.png" align="right" />

# Google DialogFlow Translate

#### *delightful google dialogflow agent tRanslation*

Command line tool for automated translation of Google DialogFlow agents

---
[![made-with-Rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](../../graphs/commit-activity)
[![License](https://img.shields.io/badge/License-Apache-blue.svg)](LICENSE-APACHE)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)
[![Build Status](https://travis-ci.org/jabber-tools/gdf_translate.svg?branch=master)](https://travis-ci.org/jabber-tools/gdf_translate)

<div align="center">
  <h3>
    <a href="README.md">
      <i>User guide</i>
    </a>
    <span> | </span>
    <a href="README-devnotes.md">
      Developer guide
    </a>
    <span> | </span>
    <a href="../../releases">
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
Google Translate is command line utility used for translation of Google DialogFlow agents. Ambition of this tool is to replace original [Agent Toolkit](https://git.dhl.com/VA-Platform-2175/va-dialogflow-agent-toolkit) that can (among other things) translate GDF agents. The advantages of this new tool (over agent toolkit) are following:

*	We are doing translation only, nothing else. Tool is because of that more intuitive, lightweight and easier to use.
*	Tool was written in technology that requires no underlying runtime. Original agent toolkit requires [Node.JS](https://nodejs.org/en/) to be installed, together with [windows-build-tools](https://www.npmjs.com/package/windows-build-tools) npm package. Making this to work on corporate laptop with lot of restrictions is usually very challenging, especially for non-IT users. New tool is single executable (by default exe file for windows) which is not requiring anything else. Just download and use!
*	Translation of intent training phrases is done piece by piece and it does not require manual annotation of entities once translated. This greatly simplifies overall translation process. If original way of translating training phrases will be needed it can be implanted as well with possibility to specify which one to use during translation.
* Original Agent Toolkit does support only following Google DialogFlow message types: <i>0, 1, 2, 3, 4</i>. We do support much more currently: <i>0, 4, 2, 1, 3, table_card, custom_payload, basic_card, suggestion_chips, list_card, link_out_chip, carousel_card, browse_carousel_card, media_content, simple_response</i>. Additional message types can be implemented on demand.

*	<b>Most important</b>: apart from Google Translate API V2 (basic) we support V3 API (advanced) as well! For details have a look at [Google Cloud Translation Documentation](https://cloud.google.com/translate/docs/editions). This is a real game changer since it enables <b>batch translations</b>. Basically instead of translating text (be it utterance, entity or response) one by one, i.e. by invoking thousands of separate HTTP transactions, we can create CVS export and upload it to Google Servers which take care of the translation. Translated CVS file is then downloaded and applied to agent. The benefit is clear: this approach is much less error prone and <b>blazingly fast</b> in comparison with V2 translation. No more 30 minutes spent by looking at translation progressing and then crashing due to network issue or Google quota being hit! More technical explanation can be found in [Developer guide](https://github.com/jabber-tools/gdf_translate/blob/master/README-devnotes.md).


## Where To Get Binaries
Binaries are published under [Releases](../../releases) section of this github repository. Make sure to always use the latest release in order to get the latest fixes and features.

Release overview:
| Version           | Binary          | OS     |
|-------------------|:---------------:|--------|
| v0.1.0-beta       | [Download here](../../releases/download/v0.1.0-beta/gdf_translate_v0.1.0-beta.zip) | Windows |

## Service Account Requirements
We are using service accounts when interacting with Google Translation APIs. Respective Google Cloud Project must have Google Translation API enabled and billing configured accordingly. Service account should have ideally following role assigned: <b>Cloud Translation API Admin</b>. More details on permissions can be found [here](https://cloud.google.com/translate/docs/intro-to-v3#iam). For V3 translations service account should also include roles for managing Google Storage Buckets (creation of the bucked, upload into bucket and deletion of buckets). Details [here](https://cloud.google.com/storage/docs/access-control/iam-roles). Additional links:

[Managing Google Cloud Service Accounts](https://cloud.google.com/iam/docs/creating-managing-service-accounts)

[Managing Google Cloud Service Account Keys](https://cloud.google.com/iam/docs/creating-managing-service-account-keys)

## Command Line Interface
Simply ask for help:
```

C:\tmp>gdf_translate.exe -h
Google DialogFlow Translate v0.1.0-beta
Adam Bezecny
Tool for automated translation of Google DialogFlow agents.

USAGE:
    gdf_translate.exe [FLAGS] [OPTIONS] --source-lang <lang ISO code> --cred-file <FILE> --agent-file <FILE> --output-folder <FOLDER> --target-lang <lang ISO code>

FLAGS:
    -d, --create-output-tsv    If this flag is specified it will preserve for V3 API downloaded output buckets. This is
                               primarily intented for debugging, no need to specify by ordinary users. For V2 API this
                               flag is ignored.
    -h, --help                 Prints help information
    -e, --skip-entities        If present entities are not translated
    -r, --skip-responses       If present responses are not translated
    -u, --skip-utterances      If present utterances are not translated
    -V, --version              Prints version information

OPTIONS:
    -s, --source-lang <lang ISO code>    ISO code of source language.E.g.: en
    -c, --cred-file <FILE>               Path to Google Cloud service account credentials used to run translation via
                                         Google Translate V2/V3 API. Must have respective priviledges: See github README
                                         for more details.
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
It might happen your agent will be not translated properly or it will be not translated at all due to some unexpected error. Should this happen raise the issue [here](../../issues). Don't forget to include following:
* Exact command you did use to run the translation
* Standard output with error details
* Don't forget to attach zip file with agent export!
* <b>DO NOT</b> attach service account JSON file! Contact us over email/skype/etc. so that we can agree on secure way of providing service account file.

## Examples

Translate sample-agent.zip from english to german language. Translate all (i.e. utterances, entities, responses). Uses credential file credentials.json. Use default translation mode, i.e. Google Translate API V3.
```
gdf_translate.exe --agent-file C:/tmp/sample-agent.zip --output-folder c:/tmp/out --source-lang en --target-lang de --cred-file C:/tmp/cred/credentials.json
```

Translate using Google Translate API V2.
```
gdf_translate.exe --agent-file C:/tmp/sample-agent.zip --output-folder c:/tmp/out --source-lang en --target-lang de --cred-file C:/tmp/cred/credentials.json --api-version v2
```

Translate using Google Translate API V2. Use 4 parallel task executors (default is 10).
```
gdf_translate.exe --agent-file C:/tmp/sample-agent.zip --output-folder c:/tmp/out --source-lang en --target-lang de --cred-file C:/tmp/cred/credentials.json --api-version v2 --task-count 4
```

Translate using Google Translate API V3. API version is explicitly specified via --api-version v3
```
gdf_translate.exe --agent-file C:/tmp/sample-agent.zip --output-folder c:/tmp/out --source-lang en --target-lang de --cred-file C:/tmp/cred/credentials.json --api-version v3
```

Translate using Google Translate API V3. Preserve file with exact content of Google Cloud output content. This file can be used for troubleshooting, it is not intented to be used by ordinary users.
```
gdf_translate.exe --agent-file C:/tmp/sample-agent.zip --output-folder c:/tmp/out --source-lang en --target-lang de --cred-file C:/tmp/cred/credentials.json --api-version v3 --create-output-tsv
```

Translate only reponses and utterances. Do not translate entities.
```
gdf_translate.exe --agent-file C:/tmp/sample-agent.zip --output-folder c:/tmp/out --source-lang en --target-lang de --cred-file C:/tmp/cred/credentials.json --skip-entities
```

