# Project README

![Screenshot of chat interface](screenshot-20241009.png)

## Project Structure

- [common](common/): Common code shared by contracts, delegates and UI
- [ui](ui/): User interface code ([Dioxus](https://dioxuslabs.com))

# This is the original Github Issue and state design which may be out-of-date relative to the code

# Goal

A proof-of-concept of a simple decentralized group chat system on Freenet that could potentially
replace the Freenet Matrix channel.

# Features

- Initially, a simple command-line client interface will be used, which will later be replaced by a
  web interface.
- Anyone can read the chat. To post new chat messages, a user must be invited by an existing user
  unless they are the "owner."
- When a user is invited, the inviter assigns a nickname to them.
- Users can be banned by the user that invited them or any "upstream" user in the invite chain.
- If a user is banned, they will no longer be able to post, and their messages will be removed.
- The contract will store a specified number of recent messages.
- Uses [bincode](https://docs.rs/bincode/latest/bincode/) for efficient serialization.
- An upgrade mechanism allows the owner to replace the chat contract with a new version, which may
  require the user to update the command-line client to one that understands the new contract.

# Limitations

- Users must be invited, which means they'll need some external means to ask for an invitation (in
  the early stages, they can ask in the freenet-dev Matrix channel).
- A command-line interface will limit usage to technical users, but a friendly web interface will
  come later (hopefully using the same contract for a smooth upgrade).

Absolutely, let's refine it for a more concise and technical approach, akin to an RFC (Request for
Comments):

# Permissioning Mechanism

To address problems like spam, permissioning governs who can speak through a hierarchical structure
known as the invitation tree.

## Invitation Tree

Each room is created and owned by a designated user, forming the root of the invitation tree. Users
invited to the room branch off from the owner, creating a hierarchical structure.

```
Room: freenet (Owner: owner)
│
├─── User: alice
│    │
│    ├─── User: bob
│    │    │
│    │    └─── User: charlie
│    │
│    ├─── User: dave
│    │
│    └─── User: eve
│
└─── User: frank
```

## Permissioning Example

Consider the scenario where "alice" invites "bob", who subsequently invites "charlie". If "alice"
decides to ban "charlie" from the room, she can directly enforce this action, exercising authority
over users invited by her or those invited further down the chain.

```
Room: freenet (Owner: owner)
│
├─── User: alice
│    │
│    ├─── User: bob
│    │    │
│    │    └─── Banned User: charlie
│    │
│    ├─── User: dave
│    │
│    └─── User: eve
│
└─── User: frank
```

In this example:

- "alice", being higher in the invite chain, has the authority to ban "charlie" directly,
  irrespective of "bob" inviting "charlie" to the room.
- This illustrates how permissioning cascades down the invitation tree, enabling users higher in the
  hierarchy to enforce rules and manage the behavior of users beneath them.

# Command Line App

## Command Usage

### Create a New Room

To create a new room, use the `create-room` command. This command requires a room name.

```bash
# Create a new room with a specified name
$ freenet-chat create-room --name "freenet"
Room 'freenet' created successfully and stored locally.
```

### Create a New User

To create a new user, use the `create-user` command. This command requires a nickname.

```bash
# Create a new user with a specified nickname
$ freenet-chat create-user --nickname "newuser123"
User 'newuser123' created successfully and stored.
```

### Join a Room and Chat

To join an existing room, use the `join-room` command. This command requires the room's public key
or name if it has been joined before.

```bash
# Example: Start chatting in the room
$ freenet-chat join-room --pubkey "ABC123DEFG..."
Joined room 'freenet-dev'
sanity: I had pasta for dinner
gogo: Nobody cares
> I care!

# Example where user isn't yet a member
$ freenet-chat join-room --pubkey "ABC123DEFG..."
You are not yet a member of 'freenet-dev', ask a current member to invite you
using your nickname and public key: "sanity:WXYZ123ABC456..."
[/] Waiting for invitation (ctrl+c to cancel)
```

### Invite a User

To invite a new user, use the `invite-user` command. This command requires the public key of the
user to invite and a nickname.

```bash
# Invite a new user by their public key and nickname
$ freenet-chat invite-user --room "freenet-dev" --user "sanity:ABCD1234EFGH5678..."
User 'sanity' has been successfully invited to 'freenet-dev'.
```

### Ban a User

To ban a user, use the `ban-user` command. This command requires the public key of the user to ban.

```bash
# Example with confirmation message
$ freenet-chat ban-user --room "freenet-dev" --user "sanity"
User 'sanity' banned successfully from 'freenet-dev'
```

# Design

## Contract Parameters

```rust
#[derive(Serialize, Deserialize)]
pub struct ChatParameters {
    pub owner: ECPublicKey,
}
```

## Contract State

Represents the state of a chat room contract, including its configuration, members, recent messages,
recent user bans, and information about a replacement chat room contract if this one is out-of-date.

```rust
#[derive(Serialize, Deserialize)]
pub struct ChatState {
    pub configuration: AuthorizedConfiguration,

    /// Any members excluding any banned members (and their invitees)
    pub members: HashSet<AuthorizedMember>,
    pub upgrade: Option<AuthorizedUpgrade>,

    /// Any authorized messages (which should exclude any messages from banned users)
    pub recent_messages: Vec<AuthorizedMessage>,
    pub ban_log: Vec<AuthorizedUserBan>,
}

#[derive(Serialize, Deserialize)]
pub struct AuthorizedConfiguration {
    pub configuration: Configuration,
    pub signature: ECSignature,
}

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub configuration_version: u32,
    pub name: String,
    pub max_recent_messages: u32,
    pub max_user_bans: u32,
}

#[derive(Serialize, Deserialize)]
pub struct AuthorizedMember {
    pub member: Member,
    pub invited_by: ECPublicKey,
    pub signature: ECSignature,
}

#[derive(Serialize, Deserialize)]
pub struct Member {
    pub public_key: ECPublicKey,
    pub nickname: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthorizedUpgrade {
    pub upgrade: Upgrade,
    pub signature: ECSignature,
}

#[derive(Serialize, Deserialize)]
pub struct Upgrade {
    pub version: u8,
    pub new_chatroom_address: Blake3Hash,
}

#[derive(Serialize, Deserialize)]
pub struct AuthorizedMessage {
    pub time: SystemTime,
    pub content: String,
    pub signature: ECSignature, // time and content
}

#[derive(Serialize, Deserialize)]
pub struct AuthorizedUserBan {
    pub ban: UserBan,
    pub banned_by: ECPublicKey,
    pub signature: ECSignature,
}

#[derive(Serialize, Deserialize)]
pub struct UserBan {
    pub banned_at: SystemTime,
    pub banned_user: ECPublicKey,
}
```

## Contract Summary

Summarizes the state of a chat room contract, must be compact but must contain enough information
about the contract to create a delta that contains whatever is in the state that isn't in the
summarized state.

```rust
#[derive(Serialize, Deserialize)]
pub struct ChatSummary {
    pub configuration_version: u32,
    pub member_hashes: HashSet<u64>,
    pub upgrade_version: Option<u8>,
    pub recent_message_hashes: HashSet<u64>,
    pub ban_log_hashes: Vec<u64>,
}
```

## Contract Delta

Efficiently represents the difference between two chat room contract states, including
configuration, members, recent messages, recent user bans, and a replacement chat room contract. It
can be assumed that once replaced, no further changes will be made to a chat room (which means it
will be a footgun, so extreme care will be necessary when upgrading a contract).

```rust
#[derive(Serialize, Deserialize)]
pub struct ChatDelta {
    pub configuration: Option<AuthorizedConfiguration>,
    pub members: HashSet<AuthorizedMember>,
    pub upgrade: Option<AuthorizedUpgrade>,
    pub recent_messages: Vec<AuthorizedMessage>,
    pub ban_log: Vec<AuthorizedUserBan>,
}
```

# Local Data Storage

Use a library like [confy](https://crates.io/crates/confy) for storing local configuration in
whatever way is standard for the OS. The CLI should store:

- Users, nickname, public key, and private key
- Rooms - name and Freenet identifier

# Best Practices

1. **Help Command**: Ensure there's a `--help` or `-h` flag for each command to provide users with
   guidance on usage directly from the CLI.
2. **Error Handling**: Consider including error messages and handling for scenarios such as:
   - Attempting to create a room with a duplicate name.
   - Creating a user with an existing nickname.
   - Joining a room with an invalid or incorrect public key.
   - Inviting a user who is already a member.
   - Banning a user who is not a member.
3. **Command Aliases**: Provide shorter aliases for frequently used commands (e.g., `cr` for
   `create-room`, `cu` for `create-user`).
4. **Interactive Mode**: Consider an interactive mode for users who prefer not to remember command
   syntax. This could guide them through the process step-by-step.
5. **Configuration Management**: Allow users to list and modify their stored rooms and user
   configurations directly from the CLI (e.g., `list-rooms`, `remove-room`).
