# GemaTokenAccountingCli
Rust Command Line tool to manage GEMA loyalty tokens.

## Why This Tool

We as GEMA created a tool to manage and account our tokens and also for devs to manage their tokens. Issues regarding it are welcome.

## How to use

The goal of this program is to create crypto accounts or any accounts and add entries to them.
Accounts are composed of :
* a token name
* a currency
* a balance

Make sure you have rust installed then open the folder and run:
```shell
cargo build 
```

The exe file will then be created and stored in the directory /target/debug/accountscli.exe
Navigate using your terminal to the directory.

We also included a guide on how to use the commands which pops with just running the tool without any command or the
 ```shell 
 --help 
 ``` 
 command.   

```shell
$ ./gemaaccountscli.exe 
```

```shell
$ ./gemaaccountscli.exe --help
```



To create a new **account**, use:
```shell
$ ./gemaaccountscli.exe --new my-crypto-account # Creates an account.
```

result:
```
Account 'my-crypto-account' created.
```

Entries are compose of :
* a label/name of token
* an amount
* it's type (withdrawal or deposit)
* a date (optional)
* notes (optional)

To create an **entry**, use:
```shell
$ ./gemaaccountscli.exe --new my-crypto-account "BITCOIN" "0.0010" + # Add an entry with "BITCOIN" as a label and "0.0010" as the amount. With + it is a deposit and - acts as a withdrawal.

$ ./gemaaccountscli.exe --new my-crypto-account "NEAR" "21" + "10/06/2022" "Purchased at Binance" # this one has a date and a note, and it is a withdrawal.
```
result:
```
New entry 'BITCOIN' created.
New entry 'NEAR' created.

$ ./gemaaccountscli.exe --list my-crypto-account

+-------+---------+--------------------------+-------------------------------+
| Label | Amount  | Date                     | Note                          |
+-------+---------+--------------------------+-------------------------------+
| BITCOIN  | + 0.0010 Coins | Tue Jun 14 06:52:21 2022 |                               |
+-------+---------+--------------------------+-------------------------------+
| NEAR  | + 21 Coins  | 10/06/2022               | Purchased at Binance  |
+-------+---------+--------------------------+-------------------------------+
'my-crypto-account' balance: 21.0010 Coins.
```

You can **add**, **list**, **remove**, **rename**, change **currency** of your accounts and entries, and check your token **balance**.

## RUST CRATES
- prettytable-rs -> A library for printing pretty formatted tables in terminal
- serde_json -> A JSON serialization file format
- colored -> The most simple way to add colors in your terminal
- chrono -> Date and Time for Rust
- serde -> A generic serialization/deserialization framework