use irc::client::data::Config;
use irc::client::PackedIrcClient;
use irc::client::prelude::*;
use tokio_core::reactor::Core;
use futures::Future;

extern crate futures;
extern crate irc;
extern crate tokio_core;

fn main() {
    let config = Config {
        nickname: Some("testbot".to_owned()),
        server: Some("localhost".to_owned()),
        port: Some(6667),
        channels: Some(vec!["#test".to_owned()]),
        ..Default::default()
    };

    let mut reactor = Core::new().unwrap();
    let future = IrcClient::new_future(reactor.handle(), &config).unwrap();


    // immediate connection errors (like no internet) will turn up here...
    let PackedIrcClient(client, future) = reactor.run(future).unwrap();

    let so_called_future = future.map(|f| {
        println!("this will never get printed: {:?}", f);
        f
    });

    client.identify().expect("failed to identify");

    let mut n = 0;
    reactor.run(
        client
            .stream()
            .for_each(move |_| {
                n += 1;
                println!("processed a message - {}", n);
                Ok(())
            })
            .inspect(|_| println!("client future finished!"))
            .join(so_called_future),
    );

    // this will never be reached :(
    println!("Hello, afterworld!");
}
