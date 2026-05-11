## Reflection

a. How much data your publisher program will send to the message broker in one run?  
The publisher sends 5 events in one run, one `UserCreatedEventMessage` per `publish_event` call, for user IDs 1 through 5 (Amir, Budi, Cica, Dira, Emir), all published to the `user_created` queue.

b. The url of: “amqp://guest:guest@localhost:5672” is the same as in the subscriber program, what does it mean?
It means both programs connect to the same RabbitMQ message broker instance. The publisher sends events to that broker, and the subscriber listens for events from that same broker. This is the core of event-driven architecture: the broker acts as the shared intermediary, decoupling the two programs so they do not communicate directly with each other.

## RabbitMQ Screenshot
![alt text](rabbitmq.png)

## Screenshot after run the Subscriber and Publisher
![alt text](eventdriven.png)
When the publisher runs, it sends 5 `UserCreatedEventMessage` events to the RabbitMQ message broker, one for each user (Amir, Budi, Cica, Dira, Emir). The subscriber, which is already running and listening on the `user_created` queue, receives and processes each event as it arrives, printing the message contents to the console. This demonstrates the decoupled nature of event-driven architecture. The publisher and subscriber operate independently, communicating only through the broker.