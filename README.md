# Bitcoin Transaction Parser

This project is a Bitcoin transaction parser implemented in Rust. It decodes a Bitcoin transaction into four components: version, inputs, outputs, and locktime.

## Features

- Parses Bitcoin transactions into their constituent parts: version, inputs, outputs, and locktime.
- Written in Rust, providing performance and safety.

## Prerequisites

To run the Bitcoin transaction parser, you need to have a Bitcoin node running on your machine. Here's a guide on how to set up and run a Bitcoin node:

1. **Install Bitcoin Core:**

   Download and install Bitcoin Core from the [official website](https://github.com/bitcoin/bitcoin/).

2. **Configure Bitcoin Core:**

   Once installed, you'll need to configure Bitcoin Core to run as a full node. Follow the [documentation](https://github.com/bitcoin/bitcoin/tree/master/doc) for instructions on how to configure Bitcoin Core for your operating system.

3. **Enable RPC Interface:**

   To allow communication with the Bitcoin node, enable the RPC interface by modifying the `bitcoin.conf` configuration file. Add the following lines:

   ```
    # Daemon Options
    server=1
    fallbackfee=0.00072
    txconfirmtarget=6
    # Network Options
    testnet=1
    [test]
    # RPC Options
    rpcuser=your_username
    rpcpassword=your_password
    wallet=your_wallet

    # ZMQ Options
    zmqpubrawblock=tcp://127.0.0.1:28332
    zmqpubrawtx=tcp://127.0.0.1:28333
    zmqpubhashtx=tcp://127.0.0.1:28332
    zmqpubhashblock=tcp://127.0.0.1:28332
   ```

   Replace `your_username` and `your_password` with your desired username and password.

4. **Restart Bitcoin Core:**

   After making changes to the `bitcoin.conf` file, restart Bitcoin Core for the changes to take effect. Run the following to start Bitcoin Core locally:

   ```
   bitcoind
   ```

## How to Run

Once your Bitcoin node is up and running, follow these steps to run the Bitcoin transaction parser:

1. Clone the repository to your local machine:

```bash
git clone git@github.com:isaack-njama/tx-parser.git
```

2. Navigate to the project directory:

```bash
cd tx-parser
```

3. Run the project using Cargo:

```bash
cargo run
```

This command will compile and execute the Bitcoin transaction parser.

## Contributing

Contributions are welcome! If you'd like to contribute to this project, feel free to fork the repository, make your changes, and submit a pull request.

## Contact

For any inquiries or feedback, please contact me at <isaacknjama@proton.me>

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---
