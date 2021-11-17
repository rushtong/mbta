## MBTA API Explorer

Simple POC for listing MBTA routes and stops

See also:
* https://api-v3.mbta.com/portal
* https://api-v3.mbta.com/docs/swagger/index.html

### Running
Create a config.yaml in the root directory in the form of:
```yaml
api_key: <your MBTA api key goes here>
```
An API key is not necessary, but api calls will be rate-limited without one.

### TODO
* More and better error handling
* Testing
* Documentation
* Build a graph of all stops/routes
* Find routes between stops
