

Endpoint:id    | Endpoint:id  || created_at | endpoint_type | rate
Sink:id        | Sink:id      || created_at | sink_type | sink_data
Broadcast:id   | Broadcast:id || created_at | contents



(:Endpoint)-MEASURED->(:Observation)
Endpoint:id    | Observation:Timestamp || contents

(:Observation)-OF->(:Endpoint)
Observation:id | Endpoint:id           || created_at | contents

(:Endpoint)-SUBSCRIBER->(:Sink)
Endpoint:id    | Sink:id               || created_at

(:Endpoint)-SENT->(:Broadcast)
Endpoint:id    | Broadcast:id          || created_at

(:Broadcast)-SENT_TO->(:Endpoint)
Broadcast:id   | Sink:id               || created_at | result



## Nodes

endpoint:
- created_at
- endpoint_type
- rate
- schedule_name

endpoint types:
- rss
- atom
- https

observation:
- created_at
- s3_key
- headers
- status_code
- body
- ttl

sink:
- created_at
- sink_type
- connection_data
- last_status_code

sink types:
- discord
- slack
- teams
- sqs
- sns

broadcast:
- created_at
- contents
- ttl

## Edges

- has_observation
- subscriber
- sent
- sent_to







add endpoint:
create (Endpoint:id | Endpoint:id)
send SNS:EndpointUpdated


SNS:EndpointUpdated
if rate is not null and schedule_name is null, create schedule:
    if not rate:
        derive rate from endpoint
        - check for syndication rate limit
        - check for headers
    create schedule
    - observe Endpoint:id
    - with rate
if rate is null and schedule_name is populated
    delete schedule with schedule_name
    unset schedule_name
update (Endpoint:id | Endpoint:id) with schedule_name


attach sink to endpoint
create: Endpoint:id | Sink:id
if send_last_broadcast:
    query (Endpoint:id | Broadcast:#Latest )
    create (Sink:id | Broadcast:id)
    send: SNS:Broadcast



detach sink from endpoint
delete: (Endpoint:id | Sink:id )



add endpoint observation
write: s3://some-bucket/endpoint-type-endpoint-id/observation-timstamp
create: (Observation:Timestamp | Observation:Timestamp | s3_key | ttl?)
create: (Endpoint:id | Observation:Timestamp)
update: (Observation:#Latest -> Observation:#Previous)
create: (Observation:Timestamp | Observation:#Latest)
send: SNS:EndpointObserved (Observation:Id)



SNS:EndpointObserved
get observation
get previous observation
if diff:
    add sink broadcast for diff



add sink broadcast
create: (Broadcast:id | Broadcast:id)
query (Endpoint:id | Sink:id)
    create: (Sink:id | Broadcast:id)
    send: SNS:Broadcast



SNS:Broadcast
query (Broadcast:id)
get broadcast
get sink
transform broadcast for sink
send sink
record response