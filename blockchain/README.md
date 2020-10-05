# blockchain

```
➜  blockchain git:(master) ✗ cargo run
   Compiling blockchain v0.1.0 (/Users/kangxiaoning/workspace/rust-exercises/blockchain)
    Finished dev [unoptimized + debuginfo] target(s) in 1.43s
     Running `/Users/kangxiaoning/workspace/rust-exercises/target/debug/blockchain`
input a miner address: kangxiaoning
Difficuty: 2
generating genesis block!
Block hash: 009abff068f811304ae746bc5d1b91ee1ec8199cc99f3627711b913b4986
Block {
    header: Blockheader {
        timestamp: 1601908956,
        nonce: 81549,
        pre_hash: "0000000000000000000000000000000000000000000000000000000000000000",
        merkle: "1b82ab5c76588664e573f714a35cd924896a3a99e356be5c6e66b4a8f815fc4",
        difficulty: 2,
    },
    count: 1,
    tranactions: [
        Transaction {
            sender: "Root",
            receiver: "kangxiaoning",
            amount: 100.0,
        },
    ],
}
Menu
1) New Transaction
2) Mine block
3) Change Difficulty
4) Change Reward
0) Exit
Enter your choice: 1

enter sender address: someone
enter receiver address: kangxiaoning
Enter amount: 50
transaction added
Menu
1) New Transaction
2) Mine block
3) Change Difficulty
4) Change Reward
0) Exit
Enter your choice: 2

Generating block
Block hash: 00366e8c7c83e4635738c389b6b8677f3c3584f2248d2fc6c1d8cf6ad
Block {
    header: Blockheader {
        timestamp: 1601909163,
        nonce: 43798,
        pre_hash: "009abff068f811304ae746bc5d1b91ee1ec8199cc99f3627711b913b4986",
        merkle: "41f9289b3d18c23a9a69cae3e74e812213df434465e08a3c487c471cba6c625",
        difficulty: 2,
    },
    count: 2,
    tranactions: [
        Transaction {
            sender: "Root",
            receiver: "kangxiaoning",
            amount: 100.0,
        },
        Transaction {
            sender: "someone",
            receiver: "kangxiaoning",
            amount: 50.0,
        },
    ],
}
Block generated successfully
Menu
1) New Transaction
2) Mine block
3) Change Difficulty
4) Change Reward
0) Exit
Enter your choice: 3

enter new difficulty: 1
Updated Difficulty
Menu
1) New Transaction
2) Mine block
3) Change Difficulty
4) Change Reward
0) Exit
Enter your choice: 2

Generating block
Block hash: 017bf1c20e0896cd9e278a16bc5d30bf6556c84eb363ed75c3c81f3a68c7
Block {
    header: Blockheader {
        timestamp: 1601909236,
        nonce: 224,
        pre_hash: "00366e8c7c83e4635738c389b6b8677f3c3584f2248d2fc6c1d8cf6ad",
        merkle: "1b82ab5c76588664e573f714a35cd924896a3a99e356be5c6e66b4a8f815fc4",
        difficulty: 1,
    },
    count: 1,
    tranactions: [
        Transaction {
            sender: "Root",
            receiver: "kangxiaoning",
            amount: 100.0,
        },
    ],
}
Block generated successfully
Menu
1) New Transaction
2) Mine block
3) Change Difficulty
4) Change Reward
0) Exit
Enter your choice: 4

Enter new reward: 150.15
Updated reward
Menu
1) New Transaction
2) Mine block
3) Change Difficulty
4) Change Reward
0) Exit
Enter your choice: 2

Generating block
Block hash: 02eb980982d4f9666953e294a48138803ce51fc9d54278793387342288485
Block {
    header: Blockheader {
        timestamp: 1601909314,
        nonce: 63,
        pre_hash: "017bf1c20e0896cd9e278a16bc5d30bf6556c84eb363ed75c3c81f3a68c7",
        merkle: "3283d897d5e1ef53277e27c09ce038e1d23e31fe6bc4392f38643c034de802f",
        difficulty: 1,
    },
    count: 1,
    tranactions: [
        Transaction {
            sender: "Root",
            receiver: "kangxiaoning",
            amount: 150.15,
        },
    ],
}
Block generated successfully
Menu
1) New Transaction
2) Mine block
3) Change Difficulty
4) Change Reward
0) Exit
Enter your choice: 0

exiting!
➜  blockchain git:(master) ✗
```
