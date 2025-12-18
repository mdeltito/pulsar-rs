# test-consumer

```sh
docker compose up -d
RUST_LOG="info,pulsar::connection_manager=debug" cargo run -p test-consumer

docker compose down -v
```


```txt
2025-12-18T04:01:26.679991Z DEBUG poll_next:poll_next:poll_next:get_topics_of_namespace:get_base_connection:get_connection: pulsar::connection_manager: Looking for connection to pulsar://localhost:6650...
2025-12-18T04:01:26.680116Z DEBUG poll_next:poll_next:poll_next:get_topics_of_namespace:get_base_connection:get_connection: pulsar::connection_manager: [] no connection for pulsar://localhost:6650
2025-12-18T04:01:26.680196Z  INFO poll_next:poll_next:poll_next:get_topics_of_namespace:get_base_connection:get_connection: pulsar::connection_manager: No existing connection, creating new for pulsar://localhost:6650
2025-12-18T04:01:26.685620Z  INFO poll_next:poll_next:poll_next:get_topics_of_namespace:get_base_connection:get_connection:connect:connect_inner: pulsar::connection_manager: Connected n°ddfaccc6-c98b-4626-847c-b8d046157376 to pulsar://localhost:6650 in 5ms
2025-12-18T04:01:26.691270Z DEBUG poll_next:poll_next:lookup_partitioned_topic:lookup_partitioned_topic:lookup_topic:get_base_connection:get_connection: pulsar::connection_manager: Looking for connection to pulsar://localhost:6650...
2025-12-18T04:01:26.691313Z DEBUG poll_next:poll_next:lookup_partitioned_topic:lookup_partitioned_topic:lookup_topic:get_base_connection:get_connection: pulsar::connection_manager: [connected] returning valid connection for pulsar://localhost:6650
...snip...
2025-12-18T04:01:27.679903Z  INFO check_connections: pulsar::connection_manager: cleaning invalid or unused connections
2025-12-18T04:01:28.680896Z  INFO check_connections: pulsar::connection_manager: cleaning invalid or unused connections
2025-12-18T04:01:28.686949Z ERROR pulsar::connection_manager: strong connection was dropped, stopping keepalive task
2025-12-18T04:01:29.681327Z  INFO check_connections: pulsar::connection_manager: cleaning invalid or unused connections
2025-12-18T04:01:30.679671Z  INFO check_connections: pulsar::connection_manager: cleaning invalid or unused connections
...snip...
2025-12-18T04:01:36.680936Z DEBUG poll_next:poll_next:poll_next:get_topics_of_namespace:get_base_connection:get_connection: pulsar::connection_manager: Looking for connection to pulsar://localhost:6650...
2025-12-18T04:01:36.681038Z DEBUG poll_next:poll_next:poll_next:get_topics_of_namespace:get_base_connection:get_connection: pulsar::connection_manager: [] no connection for pulsar://localhost:6650
2025-12-18T04:01:36.681200Z  INFO poll_next:poll_next:poll_next:get_topics_of_namespace:get_base_connection:get_connection: pulsar::connection_manager: No existing connection, creating new for pulsar://localhost:6650
```
