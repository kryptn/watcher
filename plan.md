

Source:id    | Source:id  || created_at | endpoint_type | rate
Sink:id        | Sink:id      || created_at | sink_type | sink_data
Signal:id   | Signal:id || created_at | contents



(:Source)-MEASURED->(:State)
Source:id    | State:Timestamp || contents

(:State)-OF->(:Source)
State:id | Source:id           || created_at | contents

(:Source)-SUBSCRIBER->(:Sink)
Source:id    | Sink:id               || created_at

(:Source)-SENT->(:Signal)
Source:id    | Signal:id          || created_at

(:Signal)-SENT_TO->(:Source)
Signal:id   | Sink:id               || created_at | result



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
create (Source:id | Source:id)
send SNS:SourceUpdated


SNS:SourceUpdated
if rate is not null and schedule_name is null, create schedule:
    if not rate:
        derive rate from endpoint
        - check for syndication rate limit
        - check for headers
    create schedule
    - observe Source:id
    - with rate
if rate is null and schedule_name is populated
    delete schedule with schedule_name
    unset schedule_name
update (Source:id | Source:id) with schedule_name


attach sink to endpoint
create: Source:id | Sink:id
if send_last_broadcast:
    query (Source:id | Signal:#Latest )
    create (Sink:id | Signal:id)
    send: SNS:Signal



detach sink from endpoint
delete: (Source:id | Sink:id )



add endpoint observation
write: s3://some-bucket/endpoint-type-endpoint-id/observation-timstamp
create: (State:Timestamp | State:Timestamp | s3_key | ttl?)
create: (Source:id | State:Timestamp)
update: (State:#Latest -> State:#Previous)
create: (State:Timestamp | State:#Latest)
send: SNS:SourceObserved (State:Id)



SNS:SourceObserved
get observation
get previous observation
if diff:
    add sink broadcast for diff



add sink broadcast
create: (Signal:id | Signal:id)
query (Source:id | Sink:id)
    create: (Sink:id | Signal:id)
    send: SNS:Signal



SNS:Signal
query (Signal:id)
get broadcast
get sink
transform broadcast for sink
send sink
record response