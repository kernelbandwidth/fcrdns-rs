pub struct DnsMessageHeader {
    id: u16,
    query: bool,
    opcode: Opcode,
    aa: bool,
    tc: bool,
    rd: bool,
    ra: bool,
    rcode: DnsResponseCode
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

struct DnsMessageQuestion {
    qname: Vec<Vec<u8>>,
    qtype: DnsQueryType,
    qclass: DnsQueryClass,
}

struct DnsMessageQuery {
    header: DnsMessageHeader,
    body: Vec<DnsMessageQueryBody>
}