#![deny(warnings)]
#[macro_use]
mod support;
use self::support::*;

t! {
    get_1,
    client:
        request:
            uri: "/",
            ;
        response:
            status: 200,
            ;
    server:
        request:
            uri: "/",
            ;
        response:
            ;
}

t! {
    get_implicit_path,
    client:
        request:
            uri: "",
            ;
        response:
            status: 200,
            ;
    server:
        request:
            uri: "/",
            ;
        response:
            ;
}

t! {
    get_body,
    client:
        request:
            uri: "/",
            ;
        response:
            status: 200,
            headers: {
                "content-length" => 11,
            },
            body: "hello world",
            ;
    server:
        request:
            uri: "/",
            ;
        response:
            headers: {
                "content-length" => 11,
            },
            body: "hello world",
            ;
}

t! {
    get_strip_connection_header,
    client:
        request:
            uri: "/",
            ;
        response:
            status: 200,
            headers: {
                // h2 doesn't actually receive the connection header
            },
            body: "hello world",
            ;
    server:
        request:
            uri: "/",
            ;
        response:
            headers: {
                // http2 should strip this header
                "connection" => "close",
            },
            body: "hello world",
            ;
}

t! {
    get_strip_keep_alive_header,
    client:
        request:
            uri: "/",
            ;
        response:
            status: 200,
            headers: {
                // h2 doesn't actually receive the keep-alive header
            },
            body: "hello world",
            ;
    server:
        request:
            uri: "/",
            ;
        response:
            headers: {
                // http2 should strip this header
                "keep-alive" => "timeout=5, max=1000",
            },
            body: "hello world",
            ;
}

t! {
    get_strip_upgrade_header,
    client:
        request:
            uri: "/",
            ;
        response:
            status: 200,
            headers: {
                // h2 doesn't actually receive the upgrade header
            },
            body: "hello world",
            ;
    server:
        request:
            uri: "/",
            ;
        response:
            headers: {
                // http2 should strip this header
                "upgrade" => "h2c",
            },
            body: "hello world",
            ;
}

t! {
    get_body_chunked,
    client:
        request:
            uri: "/",
            ;
        response:
            status: 200,
            headers: {
                // h2 doesn't actually receive the transfer-encoding header
            },
            body: "hello world",
            ;
    server:
        request:
            uri: "/",
            ;
        response:
            headers: {
                // http2 should strip this header
                "transfer-encoding" => "chunked",
            },
            body: "hello world",
            ;
}

t! {
    post_outgoing_length,
    client:
        request:
            method: "POST",
            uri: "/hello",
            body: "hello, world!",
            ;
        response:
            ;
    server:
        request:
            method: "POST",
            uri: "/hello",
            headers: {
                "content-length" => "13",
            },
            body: "hello, world!",
            ;
        response:
            ;
}

t! {
    post_chunked,
    client:
        request:
            method: "POST",
            uri: "/post_chunked",
            headers: {
                // http2 should strip this header
                "transfer-encoding" => "chunked",
            },
            body: "hello world",
            ;
        response:
            ;
    server:
        request:
            method: "POST",
            uri: "/post_chunked",
            body: "hello world",
            ;
        response:
            ;
}

t! {
    get_2,
    client:
        request:
            uri: "/1",
            ;
        response:
            status: 200,
            ;
        request:
            uri: "/2",
            ;
        response:
            status: 200,
            ;
    server:
        request:
            uri: "/1",
            ;
        response:
            ;
        request:
            uri: "/2",
            ;
        response:
            ;
}

// In rare cases, the h2 client connection does not shutdown, resulting
// in this test simply hanging... :(
#[cfg(feature = "__internal_flaky_tests")]
t! {
    http2_parallel_10,
    parallel: 0..10
}

