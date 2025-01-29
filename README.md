# Ledger NEAR Application

![Rule enforcer](https://github.com/dj8yfo/app-near-rs/actions/workflows/guidelines_enforcer.yml/badge.svg) ![Build and tests](https://github.com/dj8yfo/app-near-rs/actions/workflows/build_and_functional_tests.yml/badge.svg)

This is a Near application written in Rust for the Ledger Nano S/X/SP devices.

* Implements standard features (display address, transaction signature...),
* Has functional tests using [Ragger](https://github.com/LedgerHQ/ragger),
* Has CI workflows mandatory for app deployment in the Ledger store.


### Development log

Details of the structure of application, features and encountered problems as seen during initial 
development cycle are present in [issue](https://github.com/dj8yfo/app-near-rs/issues/3). 

### Advantages in comparison with [app-near-legacy](https://github.com/LedgerHQ/app-near-legacy)
- unlimited size of transactions
- normal support of batch transactions, [test](https://github.com/dj8yfo/app-near-rs/blob/develop/tests/test_sign_transaction/test_batch_transaction.py), [screenshots of 2/12 action](https://github.com/dj8yfo/app-near-rs/tree/develop/tests/snapshots/nanos/test_sign_batch_transaction_all_actions/2_0_next_action) ,  none of the details of actions are shown in [`app-near-legacy`](https://github.com/LedgerHQ/app-near-legacy/tree/develop/workdir/app-near/tests/snapshots/nanos/test_sign_multiple_actions_2_apdu_exchanges)  

### Links

* 📚 [Developer's documentation](https://developers.ledger.com/)<br/>
* 🗣️ [Ledger's Discord server](https://discord.gg/Ledger)

## Quick start guide

### With VS Code

You can quickly setup a development environment on any platform (macOS, Linux or Windows) to build and test your application with [Ledger's VS Code extension](https://marketplace.visualstudio.com/items?itemName=LedgerHQ.ledger-dev-tools).

By using Ledger's own developer tools [Docker image](https://github.com/LedgerHQ/ledger-app-builder/pkgs/container/ledger-app-builder%2Fledger-app-dev-tools), the extension allows you to **build** your apps with the latest SDK, **test** them on **Speculos** and **load** them on any supported device.

* Install and run [Docker](https://www.docker.com/products/docker-desktop/).
* Make sure you have an X11 server running :
  * On Ubuntu Linux, it should be running by default.
  * On macOS, install and launch [XQuartz](https://www.xquartz.org/) (make sure to go to XQuartz > Preferences > Security and check "Allow client connections").
  * On Windows, install and launch [VcXsrv](https://sourceforge.net/projects/vcxsrv/) (make sure to configure it to disable access control).
* Install [VScode](https://code.visualstudio.com/download) and add [Ledger's extension](https://marketplace.visualstudio.com/items?itemName=LedgerHQ.ledger-dev-tools).
* Open a terminal and clone `app-near-rust` with `git clone git@github.com:dj8yfo/app-near-rs.git`.
* Open the `app-near-rust` folder with VSCode.
* Use Ledger extension's sidebar menu or open the tasks menu with `ctrl + shift + b` (`command + shift + b` on a Mac) to conveniently execute actions :
  * **Build** the app for the device model of your choice with `Build`.
  * **Test** your binary on the [Speculos emulator](https://github.com/LedgerHQ/speculos) with `Run with emulator`.
  * You can also **run functional tests**, load the app on a physical device, and more.

ℹ️ The terminal tab of VSCode will show you what commands the extension runs behind the scene.

## Compilation and load

### Building

You can build the Near app with the following command executed in the root directory of the app.

```bash
cargo ledger build nanox 
```

This command will build the app for the Nano X, but you can use any supported device (`nanos`, `nanox`, `nanosplus`)

### Loading

ℹ️ Your device must be connected, unlocked and the screen showing the dashboard (not inside an application).

[cargo-ledger](https://github.com/LedgerHQ/cargo-ledger) also allows you to side load the binary with the following command line executed in the root directory of the Near app.

```bash
cargo ledger build nanox --load
```

As for the build command, you can replace `nanos` with `nanox` or `nanosplus`.

### Emulator

You can also run the app directly on the [Speculos emulator](https://github.com/LedgerHQ/speculos)

```bash
speculos --model nanox target/nanox/release/app-near-rust
```

### Testing

See [local_test_helper.sh](./local_test_helper.sh) and [Justfile](./Justfile)

#### Unit testing

See [local_unit_tests.sh](./local_unit_tests.sh).

## Continuous Integration

The following workflows are executed in [GitHub Actions](https://github.com/features/actions) :

* Ledger guidelines enforcer which verifies that an app is compliant with Ledger guidelines. The successful completion of this reusable workflow is a mandatory step for an app to be available on the Ledger application store. More information on the guidelines can be found in the repository [ledger-app-workflow](https://github.com/LedgerHQ/ledger-app-workflows)
* Compilation of the application for all supported devices in the [ledger-app-builder](https://github.com/LedgerHQ/ledger-app-builder) docker image
* End-to-end tests with the [Speculos](https://github.com/LedgerHQ/speculos) emulator and [ragger](https://github.com/LedgerHQ/ragger) (see [tests/](tests/))
* Various lint checks :
  * Source code lint checks with `cargo fmt`
  * Python functional test code lint checks with `pylint` and `mypy`
