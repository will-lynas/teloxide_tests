# Teloxide tests

I am trying to make tests for teloxide using fake server and a bot with an url to that server.

Basic dataset is done, now of concern is a fake server.

I am going right now by the following steps:

1) Make it work
2) Make it beautiful for the user
3) Make it beautiful on the inside

## Structure

- ./dataset has different mocked structs, that are easy to implement and use
- ./proc_macros has proc macros, cuz for some reason it has to be a separate crate
- ./telegram_test_server has a server that mimicks the real one
- ./mock_bot has a mocked version of a bot, that sends requests to the fake server

## Where are the examples of a mocked bot?

`./mock_bot/src/tests.rs`

## How to implement it?

Hopefully it is as easy as doing what happens in `./mock_bot/src/tests.rs`

1) Import the dataset
2) Create a mocked bot with something that can be turned into an update, like MockMessageText or MockMessagePhoto
3) Add dependencies and/or a different bot using .dependencies(deps![]) and .me(MockedMe::new().build())
4) Dispatch it with .dispatch().await
5) Get the responces with .get_responces()
6) Do the testing with the gotten responces

## Todo

- [x] Add dataset
    - [x] Add dataset of chats
    - [x] Add dataset of common messages
    - [ ] Add dataset of queries (low priority)
    - [ ] Add dataset of messages (low priority)
    - [ ] Add structs without a category (low priority)
- [x] Add fake server
    - [ ] Add most common endpoints
    - [ ] Add tests
    - [ ] Add all common messages (low priority)
    - [ ] Add all queries (low priority)
    - [ ] Add all messages (super low priority)
    - [ ] Add everything else (may never be done)
- [x] Make mocked bot that sends requests to fake server
- [x] Make it into a library
- [ ] Publish it when it is ready
