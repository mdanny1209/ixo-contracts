# Contract setup

This guide describes how to instantiate the ixo DAO smart contract system, which is based on a fork of [DAO-DAO](https://github.com/DA0-DA0).

Start by compiling the `core` [CW-4 Group](https://github.com/CosmWasm/cw-plus/tree/main/contracts/cw4-group) CosmWasm Contract and any of the other modules that are required to extend the Group functionality.
For each Group Contract, multiple types of governance `proposal` modules may be added.
However, only one type of `voting` module may be used by a Group.   
Multiple `core contracts` may be instantiated at the same time.

Store all the required contracts on-chain using the `ixod` CLI client:

```
export TXFLAG="--node http://localhost:26657 --chain-id CHAIN ID --gas-prices 0.025uixo --gas auto --gas-adjustment 1.3"

ixod tx wasm store core.wasm --from ACCOUNT $TXFLAG -y --output json -b block
```

Take note of the `code ID` values that are displayed, which will be required at a later step.

##Process

Instantiating the `core` contract automatically instantiates all the dependent modules provided to it. 
The `core` contract routes messages to dependent modules for execution.
This requires providing the `core` contract with the code-ID and instatiation messages for each dependent module, which are found in the `msg.rs` file for each contract. 

The following is an example of what a core contract instatiation message might look like:

```
{
    "admin":"ixo1dpdas0gmhafxgvvumq3txs24j6wgmwgpf5e3k7",
    "name":"ixo group core",
    "description":"ixo group core module handling DAOs",
    "image_url":"https://uploads-ssl.webflow.com/5f5402e3cf2fb66d997d6331/6023b50915fe07301c257179_ixo%20logo%20Cyan%402x.png",
    "automatically_add_cw20s":true,
    "automatically_add_cw721s":true,
    "voting_module_instantiate_info":{
        "code_id":1,
        "msg":{"cw4_group_code_id":3,"initial_members":[{"addr":"ixo1dpdas0gmhafxgvvumq3txs24j6wgmwgpf5e3k7","weight":100}]},
        "admin":"ixo1dpdas0gmhafxgvvumq3txs24j6wgmwgpf5e3k7",
        "label":"voting mod"
    },
    "proposal_modules_instantiate_info":[{
        "code_id":2,
        "msg":{INSERT CONTRACT INSTANTIATION MESSAGE HERE},
        "admin":"ixo1dpdas0gmhafxgvvumq3txs24j6wgmwgpf5e3k7",
        "label":"proposal mod"
    },
    {
        "code_id":4,
        "msg":{INSERT CONTRACT INSTANTIATION MESSAGE HERE},
        "admin":"ixo1dpdas0gmhafxgvvumq3txs24j6wgmwgpf5e3k7",
        "label":"proposal mod 2"
    }],
    "initial_items":"",
    "dao_uri":""
}
```

##Core Contract Messages

###Instantiate Message

```
pub struct InstantiateMsg {
    /// Optional Admin with the ability to execute DAO messages
    /// directly. Useful for building SubDAOs controlled by a parent
    /// DAO. If no admin is specified the contract is set as its own
    /// admin so that the admin may be updated later by governance.
    pub admin: Option<String>,
    /// The name of the core contract.
    pub name: String,
    /// A description of the core contract.
    pub description: String,
    /// An image URL to describe the core module contract.
    pub image_url: Option<String>,

    /// If true the contract will automatically add received cw20
    /// tokens to its treasury.
    pub automatically_add_cw20s: bool,
    /// If true the contract will automatically add received cw721
    /// tokens to its treasury.
    pub automatically_add_cw721s: bool,

    /// Instantiate information for the core contract's voting
    /// power module.
    pub voting_module_instantiate_info: ModuleInstantiateInfo,
    /// Instantiate information for the core contract's
    /// proposal modules.
    // NOTE: the pre-propose-base package depends on it being the case
    // that the core module instantiates its proposal module.
    pub proposal_modules_instantiate_info: Vec<ModuleInstantiateInfo>,

    /// Initial information for arbitrary contract addresses to be
    /// added to the items map. The key is the name of the item in the
    /// items map. The value is an enum that either uses an existing
    /// address or instantiates a new contract.
    pub initial_items: Option<Vec<InitialItem>>,
    /// Implements the DAO Star standard: https://daostar.one/EIP
    pub dao_uri: Option<String>,
}
```

###Execute Message
```
pub enum ExecuteMsg {
    /// Callable by the Admin, if one is configured.
    /// Executes messages in order.
    ExecuteAdminMsgs { msgs: Vec<CosmosMsg<Empty>> },
    /// Callable by proposal modules. The DAO will execute the
    /// messages in the hook in order.
    ExecuteProposalHook { msgs: Vec<CosmosMsg<Empty>> },
    /// Pauses the DAO for a set duration.
    /// When paused the DAO is unable to execute proposals
    Pause { duration: Duration },
    /// Executed when the contract receives a cw20 token. Depending on
    /// the contract's configuration the contract will automatically
    /// add the token to its treasury.
    Receive(cw20::Cw20ReceiveMsg),
    /// Executed when the contract receives a cw721 token. Depending
    /// on the contract's configuration the contract will
    /// automatically add the token to its treasury.
    ReceiveNft(cw721::Cw721ReceiveMsg),
    /// Removes an item from the governance contract's item map.
    RemoveItem { key: String },
    /// Adds an item to the governance contract's item map. If the
    /// item already exists the existing value is overriden. If the
    /// item does not exist a new item is added.
    SetItem { key: String, addr: String },
    /// Callable by the admin of the contract. If ADMIN is None the
    /// admin is set as the contract itself so that it may be updated
    /// later by vote. If ADMIN is Some a new admin is proposed and
    /// that new admin may become the admin by executing the
    /// `AcceptAdminNomination` message.
    ///
    /// If there is already a pending admin nomination the
    /// `WithdrawAdminNomination` message must be executed before a
    /// new admin may be nominated.
    NominateAdmin { admin: Option<String> },
    /// Callable by a nominated admin. Admins are nominated via the
    /// `NominateAdmin` message. Accepting a nomination will make the
    /// nominated address the new admin.
    ///
    /// Requiring that the new admin accepts the nomination before
    /// becoming the admin protects against a typo causing the admin
    /// to change to an invalid address.
    AcceptAdminNomination {},
    /// Callable by the current admin. Withdraws the current admin
    /// nomination.
    WithdrawAdminNomination {},
    /// Callable by the core contract. Replaces the current
    /// governance contract config with the provided config.
    UpdateConfig { config: Config },
    /// Updates the list of cw20 tokens this contract has registered.
    UpdateCw20List {
        to_add: Vec<String>,
        to_remove: Vec<String>,
    },
    /// Updates the list of cw721 tokens this contract has registered.
    UpdateCw721List {
        to_add: Vec<String>,
        to_remove: Vec<String>,
    },
    /// Updates the governance contract's governance modules. Module
    /// instantiate info in `to_add` is used to create new modules and
    /// install them.
    UpdateProposalModules {
        // NOTE: the pre-propose-base package depends on it being the
        // case that the core module instantiates its proposal module.
        to_add: Vec<ModuleInstantiateInfo>,
        to_disable: Vec<String>,
    },
    /// Callable by the core contract. Replaces the current
    /// voting module with a new one instantiated by the governance
    /// contract.
    UpdateVotingModule { module: ModuleInstantiateInfo },
    /// Update the core module to add/remove SubDAOs and their charters
    UpdateSubDaos {
        to_add: Vec<SubDao>,
        to_remove: Vec<String>,
    },
}
```

###Query Message

```
pub enum QueryMsg {
    /// Get's the DAO's admin. Returns `Addr`.
    Admin {},
    /// Get's the currently nominated admin (if any). Returns
    /// `AdminNominationResponse`.
    AdminNomination {},
    /// Gets the contract's config. Returns Config.
    Config {},
    /// Gets the token balance for each cw20 registered with the
    /// contract.
    Cw20Balances {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Lists the addresses of the cw20 tokens in this contract's
    /// treasury.
    Cw20TokenList {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Lists the addresses of the cw721 tokens in this contract's
    /// treasury.
    Cw721TokenList {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Dumps all of the core contract's state in a single
    /// query. Useful for frontends as performance for queries is more
    /// limited by network times than compute times. Returns
    /// `DumpStateResponse`.
    DumpState {},
    /// Gets the address associated with an item key.
    GetItem { key: String },
    /// Lists all of the items associted with the contract. For
    /// example, given the items `{ "group": "foo", "subdao": "bar"}`
    /// this query would return `[("group", "foo"), ("subdao",
    /// "bar")]`.
    ListItems {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Gets all proposal modules associated with the
    /// contract. Returns Vec<ProposalModule>.
    ProposalModules {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Gets the active proposal modules associated with the
    /// contract. Returns Vec<ProposalModule>.
    ActiveProposalModules {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Returns information about if the contract is currently paused.
    PauseInfo {},
    /// Gets the contract's voting module. Returns Addr.
    VotingModule {},
    /// Returns all SubDAOs with their charters in a vec
    /// start_after is bound exclusive and asks for a string address
    ListSubDaos {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Implements the DAO Star standard: https://daostar.one/EIP
    DaoURI {},
}
```

For more information on how these contracts were designed, see the official documentation from [DAO-DAO](https://github.com/DA0-DA0/dao-contracts/wiki/DAO-DAO-Contracts-Design)
