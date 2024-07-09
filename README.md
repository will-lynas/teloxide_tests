# Teloxide tests

I am trying to make tests for teloxide by bot mocking.

The mocking shouldn't be very difficult. Complex - yes, but it isn't what was holding back that issue. The problem is an easy way to make test objects and check the request data. That's why im prioritizing it now.

## Todo

- [ ] Add dataset of all needed structs
    - [x] Add dataset of chats
    - [x] Add dataset of common messages
    - [ ] Add dataset of queries (low priority)
    - [ ] Add dataset of messages (low priority)
    - [ ] Add structs without a category (low priority)
- [ ] Add the bot mocking
- [ ] Try to think of a good way to compare the behaviour of the bot
- [ ] Make it into a library