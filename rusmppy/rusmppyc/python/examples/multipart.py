import asyncio
import logging

from rusmppyc import (
    Client,
    CommandId,
    RegisteredDelivery,
    SubmitSm,
    Event,
    Events,
    SubmitSmResp,
)
from rusmppyc.exceptions import RusmppycException


async def handle_events(events: Events, client: Client):
    async for event in events:
        match event:
            case Event.Incoming(cmd):
                logging.debug(f"Received Command: {cmd.id}")

                match cmd.id:
                    case CommandId.DeliverSm():
                        try:
                            await client.deliver_sm_resp(
                                cmd.sequence_number, "the message id"
                            )
                        except RusmppycException as e:
                            logging.error(f"Failed to send DeliverSm response: {e}")

            case Event.Error(err):
                logging.error(f"Error occurred: {err}")
            case _:
                logging.warning(f"Unknown event: {event}")


async def main():
    try:
        client, events = await Client.connect(
            url="smpp://rusmpps.rusmpp.org:2775",
            enquire_link_interval=5000,
            enquire_link_response_timeout=2000,
            response_timeout=2000,
            max_command_length=4096,
        )

        asyncio.create_task(handle_events(events, client))

        await client.bind_transceiver(system_id="test", password="test")

        multipart: list[SubmitSm] = client.submit_sm_multipart(
            short_message="Hi how are you?" * 20,
            registered_delivery=RegisteredDelivery.request_all(),
        )

        logging.info(f"Multipart: {multipart}")

        for i, submit_sm in enumerate(multipart):
            logging.info(f"Sending part {i + 1} of {len(multipart)}: {submit_sm}")

            submit_sm_resp: SubmitSmResp = await client.submit_sm(submit_sm)

            logging.info(f"SubmitSm Response {i + 1}: {submit_sm_resp}")

        await asyncio.sleep(5)

        await client.unbind()
        await client.close()
        await client.closed()

        logging.debug("RUSMPP connection closed")

    except RusmppycException as e:
        logging.error(f"An error occurred: {e}")


if __name__ == "__main__":
    # Blue
    logging.addLevelName(
        logging.DEBUG, "\033[1;34m%s\033[1;0m" % logging.getLevelName(logging.DEBUG)
    )
    # Green
    logging.addLevelName(
        logging.INFO, "\033[1;32m%s\033[1;0m" % logging.getLevelName(logging.INFO)
    )
    # Yellow
    logging.addLevelName(
        logging.WARNING, "\033[1;33m%s\033[1;0m" % logging.getLevelName(logging.WARNING)
    )
    # Red
    logging.addLevelName(
        logging.ERROR, "\033[1;31m%s\033[1;0m" % logging.getLevelName(logging.ERROR)
    )
    # White on Red Background
    logging.addLevelName(
        logging.CRITICAL,
        "\033[1;37;41m%s\033[1;0m" % logging.getLevelName(logging.CRITICAL),
    )

    logging.basicConfig(
        format="%(asctime)-15s %(levelname)s %(name)s %(filename)s:%(lineno)d %(message)s"
    )

    logging.getLogger().setLevel(logging.DEBUG)

    logging.getLogger("hickory_proto").setLevel(logging.WARNING)
    logging.getLogger("hickory_resolver").setLevel(logging.WARNING)
    logging.getLogger("rusmpp").setLevel(logging.INFO)
    logging.getLogger("rusmppc").setLevel(logging.DEBUG)
    logging.getLogger("rusmppyc").setLevel(logging.DEBUG)

    asyncio.run(main())
