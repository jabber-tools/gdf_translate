// https://cloud.google.com/translate/docs/intro-to-v3
// https://cloud.google.com/translate/docs/reference/rest/v3/projects/translateText
// https://cloud.google.com/translate/docs/reference/rest/v3/projects.locations/batchTranslateText
// https://cloud.google.com/translate/docs/reference/rest/v3/projects.locations.operations#Operation
// https://cloud.google.com/translate/docs/reference/rest/v3/projects.locations.operations/get
// https://cloud.google.com/translate/docs/reference/rest/v3/projects.locations.operations/wait

/*

curl --location --request POST 'https://translation.googleapis.com/v3/projects/dummy-project-id/locations/us-central1:batchTranslateText' \
--header 'Authorization: Bearer ya29.c....' \
--header 'Content-Type: application/javascript' \
--data-raw '{
    "sourceLanguageCode": "en",
    "targetLanguageCodes": "de",
    "inputConfigs": [{
        "mimeType":  "text/html",
        "gcsSource": {
            "inputUri": "gs://translate_v3_test_in/input.tsv"
        }
    }],
    "outputConfig": {
        "gcsDestination": {
            "outputUriPrefix": "gs://translate_v3_test_out/"
        }
    }
}'


curl --location --request GET 'https://translation.googleapis.com/v3/projects/dummy-project-id/locations/us-central1/operations/20200615-11411592246465-5edebebf-0000-2598-9feb-24058877eccc' \
--header 'Authorization: Bearer ya29.c....'


curl --location --request POST 'https://translation.googleapis.com/v3/projects/dummy-project-id/locations/us-central1/operations/20200615-11581592247524-5edeccd9-0000-26b7-bd4f-30fd38139c64:wait' \
--header 'Authorization: Bearer ya29.c....' \
--header 'Content-Type: application/json' \
--data-raw '{
  "timeout": "60s"
}'

*/
