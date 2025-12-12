import asyncio
import builtins
from typing import Optional, List, overload

from .events import Events
from .rusmppyc import (
    BindReceiverResp,
    BindTransceiverResp,
    BindTransmitterResp,
    CommandStatus,
    DataCoding,
    EsmClass,
    InterfaceVersion,
    MessageSubmissionRequestTlvValue,
    Npi,
    SubmitSm,
    SubmitSmResp,
    Ton,
    ReplaceIfPresentFlag,
    RegisteredDelivery,
)

__all__ = ["Client"]

class Client:
    """
    `SMPP` Client.

    The client is a handle to communicate with the `SMPP` server through a managed connection in the background.
    """

    @classmethod
    async def connect(
        cls,
        url: builtins.str,
        max_command_length: builtins.int = 4096,
        enquire_link_response_timeout: builtins.int = 2000,
        enquire_link_interval: Optional[builtins.int] = 5000,
        auto_enquire_link_response: bool = True,
        response_timeout: Optional[builtins.int] = 2000,
        interface_version_check: bool = True,
    ) -> tuple["Client", Events]:
        """
        Connect to an SMPP server.

        This method establishes a connection to the given SMPP server and returns
        a client for sending SMPP commands and an event stream for receiving messages or errors.
        The connection automatically manages timeouts, enquire links, and other protocol details.

        Parameters
        ----------
        url : str
            The URL of the SMPP server to connect to. Supported schemes:
            - ``smpp`` for plain TCP
            - ``ssmpp`` or ``smpps`` for TLS connections
            If no port is specified, the default port 2775 is used.
        max_command_length : int, default=4096
            Maximum length in bytes of incoming SMPP commands.
        enquire_link_response_timeout : int, default=2000
            Time in milliseconds to wait for an EnquireLink response before
            considering it failed.
        enquire_link_interval : Optional[int], default=5000
            Interval in milliseconds between automatic EnquireLink commands.
            Set to ``None`` to disable EnquireLink.
        auto_enquire_link_response : bool, default=True
            If ``True``, automatically responds to incoming EnquireLink commands
        response_timeout : Optional[int], default=2000
            Time in milliseconds to wait for any command response. Set to
            ``None`` to wait indefinitely.
        interface_version_check : bool, default=True
            If ``False``, disables interface version validation.
            This library uses ``SMPP v5`` implementation to encode and decode commands.
            Binding to a server with another SMPP version may cause issues encoding and decoding commands.
            Disable interface version check to allow binding to servers with any SMPP version.

        Returns
        -------
        tuple[Client, Events]
            A tuple containing the connected client object and the event stream.

        Raises
        ------
        ConnectException
            If any of the following occur:
            - The URL is invalid.
            - The URL scheme is unsupported.
            - The host is missing or cannot be resolved.
            - The connection fails.
            - The TLS handshake fails.

        Notes
        -----
        Path and query parameters in the URL are ignored silently.

        Examples
        --------
        Connect to a local SMPP server:

        >>> from rusmppyc import Client
        >>> async def example():
        ...     client, events = await Client.connect("smpp://localhost:2775")

        Connect using TLS:

        >>> from rusmppyc import Client
        >>> async def example():
        ...     client, events = await Client.connect("smpps://localhost:2775")
        """
        ...

    @classmethod
    async def connected(
        cls,
        read: asyncio.StreamReader,
        write: asyncio.StreamWriter,
        max_command_length: builtins.int = 4096,
        enquire_link_response_timeout: builtins.int = 2000,
        enquire_link_interval: Optional[builtins.int] = 5000,
        auto_enquire_link_response: bool = True,
        response_timeout: Optional[builtins.int] = 2000,
    ) -> tuple["Client", Events]:
        """
        Create a client from an existing asyncio connection.

        This method wraps an already established connection using the provided
        ``StreamReader`` and ``StreamWriter`` objects. It returns a client for sending
        SMPP commands and an event stream for receiving messages or errors. The
        connection automatically manages timeouts, enquire links, and other protocol
        details.

        Parameters
        ----------
        read : asyncio.StreamReader
            The asyncio stream reader associated with the existing connection.
        write : asyncio.StreamWriter
            The asyncio stream writer associated with the existing connection.
        max_command_length : int, default=4096
            Maximum length in bytes of incoming SMPP commands.
        enquire_link_response_timeout : int, default=2000
            Time in milliseconds to wait for an EnquireLink response before
            considering it failed.
        enquire_link_interval : Optional[int], default=5000
            Interval in milliseconds between automatic EnquireLink commands.
            Set to ``None`` to disable EnquireLink.
        auto_enquire_link_response : bool, default=True
            If ``True``, automatically responds to incoming EnquireLink commands
        response_timeout : Optional[int], default=2000
            Time in milliseconds to wait for any command response. Set to
            ``None`` to wait indefinitely.
        interface_version_check : bool, default=True
            If ``False``, disables interface version validation.
            This library uses ``SMPP v5`` implementation to encode and decode commands.
            Binding to a server with another SMPP version may cause issues encoding and decoding commands.
            Disable interface version check to allow binding to servers with any SMPP version.

        Returns
        -------
        tuple[Client, Events]
            A tuple containing the connected client object and the event stream.
        """
        ...

    async def bind_transmitter(
        self,
        system_id: builtins.str = "",
        password: builtins.str = "",
        system_type: builtins.str = "",
        interface_version: InterfaceVersion = InterfaceVersion.Smpp5_0(),
        addr_ton: Ton = Ton.Unknown(),
        addr_npi: Npi = Npi.Unknown(),
        address_range: builtins.str = "",
        status: CommandStatus = CommandStatus.EsmeRok(),
    ) -> BindTransmitterResp:
        """
        Sends a ``BindTransmitter`` command to the server and waits for a successful ``BindTransmitterResp``.


        Parameters
        ----------
        system_id : str, default=""
            The system ID used to identify the ESME to the SMSC.
        password : str, default=""
            The password used to authenticate the bind request.
        system_type : str, default=""
            The system type identifying the nature of the ESME system.
        interface_version : InterfaceVersion, default=InterfaceVersion.Smpp5_0()
            The SMPP protocol version to use for the bind request.
        addr_ton : Ton, default=Ton.Unknown()
            The Type of Number (TON) for the address range.
        addr_npi : Npi, default=Npi.Unknown()
            The Numbering Plan Indicator (NPI) for the address range.
        address_range : str, default=""
            The address range associated with the bind request.
        status : CommandStatus, default=CommandStatus.EsmeRok()
            The command status to include in the bind request.

        Returns
        -------
        BindTransmitterResp
            The response returned by the server upon a successful bind.

        Raises
        ------
        RusmppycException
        """
        ...

    async def bind_receiver(
        self,
        system_id: builtins.str = "",
        password: builtins.str = "",
        system_type: builtins.str = "",
        interface_version: InterfaceVersion = InterfaceVersion.Smpp5_0(),
        addr_ton: Ton = Ton.Unknown(),
        addr_npi: Npi = Npi.Unknown(),
        address_range: builtins.str = "",
        status: CommandStatus = CommandStatus.EsmeRok(),
    ) -> BindReceiverResp:
        """
        Sends a ``BindReceiver`` command to the server and waits for a successful ``BindReceiverResp``.


        Parameters
        ----------
        system_id : str, default=""
            The system ID used to identify the ESME to the SMSC.
        password : str, default=""
            The password used to authenticate the bind request.
        system_type : str, default=""
            The system type identifying the nature of the ESME system.
        interface_version : InterfaceVersion, default=InterfaceVersion.Smpp5_0()
            The SMPP protocol version to use for the bind request.
        addr_ton : Ton, default=Ton.Unknown()
            The Type of Number (TON) for the address range.
        addr_npi : Npi, default=Npi.Unknown()
            The Numbering Plan Indicator (NPI) for the address range.
        address_range : str, default=""
            The address range associated with the bind request.
        status : CommandStatus, default=CommandStatus.EsmeRok()
            The command status to include in the bind request.

        Returns
        -------
        BindReceiverResp
            The response returned by the server upon a successful bind.

        Raises
        ------
        RusmppycException
        """
        ...

    async def bind_transceiver(
        self,
        system_id: builtins.str = "",
        password: builtins.str = "",
        system_type: builtins.str = "",
        interface_version: InterfaceVersion = InterfaceVersion.Smpp5_0(),
        addr_ton: Ton = Ton.Unknown(),
        addr_npi: Npi = Npi.Unknown(),
        address_range: builtins.str = "",
        status: CommandStatus = CommandStatus.EsmeRok(),
    ) -> BindTransceiverResp:
        """
        Sends a ``BindTransceiver`` command to the server and waits for a successful ``BindTransceiverResp``.


        Parameters
        ----------
        system_id : str, default=""
            The system ID used to identify the ESME to the SMSC.
        password : str, default=""
            The password used to authenticate the bind request.
        system_type : str, default=""
            The system type identifying the nature of the ESME system.
        interface_version : InterfaceVersion, default=InterfaceVersion.Smpp5_0()
            The SMPP protocol version to use for the bind request.
        addr_ton : Ton, default=Ton.Unknown()
            The Type of Number (TON) for the address range.
        addr_npi : Npi, default=Npi.Unknown()
            The Numbering Plan Indicator (NPI) for the address range.
        address_range : str, default=""
            The address range associated with the bind request.
        status : CommandStatus, default=CommandStatus.EsmeRok()
            The command status to include in the bind request.

        Returns
        -------
        BindTransceiverResp
            The response returned by the server upon a successful bind.

        Raises
        ------
        RusmppycException
        """
        ...

    @overload
    async def submit_sm(
        self,
        *,
        service_type: builtins.str = "",
        source_addr_ton: Ton = Ton.Unknown(),
        source_addr_npi: Npi = Npi.Unknown(),
        source_addr: builtins.str = "",
        dest_addr_ton: Ton = Ton.Unknown(),
        dest_addr_npi: Npi = Npi.Unknown(),
        destination_addr: builtins.str = "",
        esm_class: EsmClass = EsmClass.default(),
        protocol_id: builtins.int = 0,
        priority_flag: builtins.int = 0,
        schedule_delivery_time: builtins.str = "",
        validity_period: builtins.str = "",
        registered_delivery: RegisteredDelivery = RegisteredDelivery.default(),
        replace_if_present_flag: ReplaceIfPresentFlag = ReplaceIfPresentFlag.DoNotReplace(),
        data_coding: DataCoding = DataCoding.McSpecific(),
        sm_default_msg_id: builtins.int = 0,
        short_message: builtins.bytes = b"",
        tlvs: builtins.list[MessageSubmissionRequestTlvValue] = [],
        status: CommandStatus = CommandStatus.EsmeRok(),
    ) -> SubmitSmResp:
        """
        Sends a ``SubmitSm`` command to the server and waits for a successful ``SubmitSmResp``.

        Parameters
        ----------
        service_type : str, default=""
            The service type (e.g., ``"CMT"``, ``"WAP"``, or vendor-defined).
        source_addr_ton : Ton, default=Ton.Unknown()
            The Type of Number (TON) for the source address.
        source_addr_npi : Npi, default=Npi.Unknown()
            The Numbering Plan Indicator (NPI) for the source address.
        source_addr : str, default=""
            The source address (e.g., sender ID).
        dest_addr_ton : Ton, default=Ton.Unknown()
            The Type of Number (TON) for the destination address.
        dest_addr_npi : Npi, default=Npi.Unknown()
            The Numbering Plan Indicator (NPI) for the destination address.
        destination_addr : str
            The destination address (recipient phone number).
        esm_class : EsmClass, default=EsmClass.default()
            The message mode and type (e.g., delivery receipt request, datagram mode).
        protocol_id : int, default=0
            The protocol identifier.
        priority_flag : int, default=0
            The priority level of the message.
        schedule_delivery_time : str, default=""
            The scheduled delivery time in SMPP absolute or relative format.
        validity_period : str, default=""
            The validity period for the message in SMPP absolute or relative format.
        registered_delivery : RegisteredDelivery, default=RegisteredDelivery.default()
            Controls whether delivery receipts or intermediate notifications are requested.
        replace_if_present_flag : ReplaceIfPresentFlag = ReplaceIfPresentFlag.DoNotReplace(),
            Indicates whether to replace an existing message with the same ID.
        data_coding : DataCoding, default=DataCoding.McSpecific()
            The data coding scheme to use for the message.
        sm_default_msg_id : int, default=0
            The default short message ID.
        short_message : bytes, optional
            The message payload (up to 254 bytes). Ignored if ``message_payload`` is provided.
        tlvs: List[MessageSubmissionRequestTlvValue], default=[]
            The Message Submission Request TLVs.
        status : CommandStatus, default=CommandStatus.EsmeRok()
            The command status to include in the ``SubmitSm`` request.

        Returns
        -------
        SubmitSmResp
            The response returned by the server upon a successful ``SubmitSm`` request.

        Raises
        ------
        RusmppycException
        """
        ...

    # TODO: docs
    @overload
    async def submit_sm(
        self,
        submit_sm: SubmitSm,
        status: CommandStatus = CommandStatus.EsmeRok(),
    ) -> SubmitSmResp:
        """"""
        ...

    async def deliver_sm_resp(
        self,
        sequence_number: builtins.int,
        message_id: builtins.str = "",
        status: CommandStatus = CommandStatus.EsmeRok(),
    ) -> None:
        """
        Sends a ``DeliverSmResp`` command to the server.

        Parameters
        ----------
        sequence_number : int
            The sequence number of the corresponding ``DeliverSm`` request.
            This value must match the sequence number of the original ``deliver_sm`` PDU.
        message_id : str, default=""
            The message ID associated with the received message. Typically used when
            responding to delivery receipts.
        status : CommandStatus, default=CommandStatus.EsmeRok()
            The command status to include in the ``DeliverSmResp`` response.

        Returns
        -------
        None

        Raises
        ------
        RusmppycException
        """
        ...

    async def unbind(self) -> None:
        """
        Sends an ``Unbind`` command to the server and wait for a successful ``UnbindResp``.

        Returns
        -------
        None

        Raises
        ------
        RusmppycException
        """
        ...

    async def unbind_resp(
        self,
        sequence_number: builtins.int,
        status: CommandStatus = CommandStatus.EsmeRok(),
    ) -> None:
        """
        Sends an ``UnbindResp`` command to the server.

        Parameters
        ----------
        sequence_number : int
            The sequence number of the corresponding ``Unbind`` request.
            This value must match the sequence number of the original ``Unbind`` PDU.
        status : CommandStatus, default=CommandStatus.EsmeRok()
            The command status to include in the ``UnbindResp`` response.

        Returns
        -------
        None

        Raises
        ------
        RusmppycException
        """
        ...

    async def generic_nack(
        self,
        sequence_number: builtins.int,
        status: CommandStatus = CommandStatus.EsmeRok(),
    ) -> None:
        """
        Sends a ``GenericNack`` command to the server.

        Parameters
        ----------
        sequence_number : int
            The sequence number of the PDU being negatively acknowledged.
        status : CommandStatus, default=CommandStatus.EsmeRok()
            The command status to include in the ``GenericNack`` response.

        Returns
        -------
        None

        Raises
        ------
        RusmppycException
        """
        ...

    async def close(self) -> None:
        """
        Closes the connection to the SMPP server.

        This coroutine initiates a graceful shutdown of the connection. It stops reading
        from the server, halts internal timers, closes the requests channel, flushes any
        pending requests, and terminates the underlying connection.

        After calling this method, the client can no longer send requests to the server.

        Raises
        ------
        RusmppycException
            If an error occurs while closing the connection.
        """
        ...

    async def closed(self) -> None:
        """
        Waits until the connection to the SMPP server is fully closed.

        This coroutine completes once the connection has been completely terminated,
        including stopping all reads, flushing pending requests, and releasing all
        associated resources.
        """
        ...

    def is_closed(self) -> bool:
        """
        Checks whether the connection to the SMPP server is closed.

        Returns
        -------
        bool
            ``True`` if the connection is fully closed, ``False`` otherwise.

        Notes
        -----
        If this method returns ``False``, the connection may still be in the process
        of closing and is not necessarily active.

        See Also
        --------
        is_active : Check if the connection is currently active.
        """
        ...

    def is_active(self) -> bool:
        """
        Checks whether the connection to the SMPP server is active.

        The connection is considered active if all of the following are true:

        - ``close()`` has not been called.
        - The connection has not encountered an error.
        - The connection can still receive requests from the client.

        Returns
        -------
        bool
            ``True`` if the connection is active, ``False`` otherwise.

        Notes
        -----
        If this method returns ``False``, it does not necessarily mean the connection
        is closed; it may be in the process of shutting down.

        See Also
        --------
        is_closed : Check if the connection is fully closed.
        """
        ...

    # TODO: docs, short_message is str and not optional. there is no data coding or esm class
    def submit_sm_multipart(
        cls,
        short_message: builtins.str,
        max_short_message_size: builtins.int = 140,
        reference: builtins.int = 0,
        service_type: builtins.str = "",
        source_addr_ton: Ton = Ton.Unknown(),
        source_addr_npi: Npi = Npi.Unknown(),
        source_addr: builtins.str = "",
        dest_addr_ton: Ton = Ton.Unknown(),
        dest_addr_npi: Npi = Npi.Unknown(),
        destination_addr: builtins.str = "",
        esm_class: EsmClass = EsmClass.default(),
        protocol_id: builtins.int = 0,
        priority_flag: builtins.int = 0,
        schedule_delivery_time: builtins.str = "",
        validity_period: builtins.str = "",
        registered_delivery: RegisteredDelivery = RegisteredDelivery.default(),
        replace_if_present_flag: ReplaceIfPresentFlag = ReplaceIfPresentFlag.DoNotReplace(),
        data_coding: DataCoding = DataCoding.McSpecific(),
        sm_default_msg_id: builtins.int = 0,
        tlvs: builtins.list[MessageSubmissionRequestTlvValue] = [],
    ) -> List[SubmitSm]:
        """"""
        ...
