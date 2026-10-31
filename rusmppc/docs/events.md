# Events

The event stream must be consumed to avoid blocking the connection task.

Set the channel capacity using [`events()`](Self::events).[`channel_capacity`](EventsConnectionBuilder::channel_capacity)
or [`events()`](Self::events).[`no_channel_capacity`](EventsConnectionBuilder::no_channel_capacity).

- If [`channel_capacity`](EventsConnectionBuilder::channel_capacity) is used, the channel will be bounded and will block the connection task if it is full.
  Make sure to consume the events from the channel to avoid blocking the connection task.
- If [`no_channel_capacity`](EventsConnectionBuilder::no_channel_capacity) is used, the channel will be unbounded and will never block the connection task.
  But this will lead to unbounded memory usage if the event stream is not consumed.
- If you are not interested in the events, use [`events()`](Self::events).[`discard()`](EventsConnectionBuilder::discard) to discard the events completely.
