// How to Sync the balance of a descriptor using bdk
// How to generate addresses from the wallet
// How to create transaction and see details as well as get it in psbt form


use bdk::bitcoin::Network;
use bdk::electrum_client::Client;
use bdk::blockchain::ElectrumBlockchain;
use bdk::{FeeRate, Wallet, SyncOptions};
use bdk::database::MemoryDatabase;
use bdk::wallet::AddressIndex::New;


fn main() -> Result<(), bdk::Error> {
    println!("Hello, world!");

    let network = Network::Bitcoin; // Or this can be Network::Bitcoin, Network::Signet or Network::Regtest
    println!("Network {network}");

    let client = Client::new("ssl://electrum.blockstream.info:60002")?;
    let blockchain = ElectrumBlockchain::from(client);
    let wallet = Wallet::new(
        "wsh(sortedmulti(3,[2f2430b6/48h/0h/0h/2h]xpub6FPZJj8Y1fy5riJgZqzmN5g34c8enH2VemdaC6N3HYWjCYSxopMLzgkJkhaixAAn4snGF4z9LbUed7dSHGwJPq3SrVtN2kKnJM6kXyyt7gz/0/*,[b19af141/48h/0h/0h/2h]xpub6F7yTLHw5MW91JaTTjvvADRXK36popopiRZzB16pkiu6cehD3D5bH5mhvnxZqx7YcBvtNHky3WejCM2PrJhoERdGUJCwiLrRqe5h5fE5YAm/0/*,[fe638cd1/48h/0h/0h/2h]xpub6ExFG1vPZ3ye5eiG5gPy2nWPJ7GjiRmB3xRza5NCvJ4RKdFoBQKMDKLoeSDyQxAqfUdfEKyVsiWznQcUegJp5ucZutFTKt7zzGpmbMaoKxC/0/*,[5a63faae/48h/0h/0h/2h]xpub6ExoYunk777M2pMe4ctzrAjzNkw4R3ZFYrdyWqKtLauyPoxXo4pWrySRSnhNR2rMdyDLyKdGLqDVspWaywHGGgE7Hx73Q5HDmDcrLSqaox6/0/*,[cadb92cb/48h/0h/0h/2h]xpub6EggpYzGXQ5sfEi8TmghXWdvVLNFMiq37f38rezHhy7rTUFam4fojiZcszreBHwo5MspGt7nqhGpErTtZC1mpWra3NwzuXkDynLbacRb3MZ/0/*))#8j8h6hd3",
        Some("wsh(sortedmulti(3,[2f2430b6/48h/0h/0h/2h]xpub6FPZJj8Y1fy5riJgZqzmN5g34c8enH2VemdaC6N3HYWjCYSxopMLzgkJkhaixAAn4snGF4z9LbUed7dSHGwJPq3SrVtN2kKnJM6kXyyt7gz/1/*,[b19af141/48h/0h/0h/2h]xpub6F7yTLHw5MW91JaTTjvvADRXK36popopiRZzB16pkiu6cehD3D5bH5mhvnxZqx7YcBvtNHky3WejCM2PrJhoERdGUJCwiLrRqe5h5fE5YAm/1/*,[fe638cd1/48h/0h/0h/2h]xpub6ExFG1vPZ3ye5eiG5gPy2nWPJ7GjiRmB3xRza5NCvJ4RKdFoBQKMDKLoeSDyQxAqfUdfEKyVsiWznQcUegJp5ucZutFTKt7zzGpmbMaoKxC/1/*,[5a63faae/48h/0h/0h/2h]xpub6ExoYunk777M2pMe4ctzrAjzNkw4R3ZFYrdyWqKtLauyPoxXo4pWrySRSnhNR2rMdyDLyKdGLqDVspWaywHGGgE7Hx73Q5HDmDcrLSqaox6/1/*,[cadb92cb/48h/0h/0h/2h]xpub6EggpYzGXQ5sfEi8TmghXWdvVLNFMiq37f38rezHhy7rTUFam4fojiZcszreBHwo5MspGt7nqhGpErTtZC1mpWra3NwzuXkDynLbacRb3MZ/1/*))#8j8h6hd3"),
        network,
        MemoryDatabase::default(),
    )?;

    wallet.sync(&blockchain, SyncOptions::default())?;

    println!("Descriptor balance: {} SAT", wallet.get_balance()?);

    println!("Address #0: {}", wallet.get_address(New)?);
    println!("Address #1: {}", wallet.get_address(New)?);
    println!("Address #2: {}", wallet.get_address(New)?);


    let send_to = wallet.get_address(New)?;
    let (psbt, details) = {
        let mut builder =  wallet.build_tx();
        builder
            .add_recipient(send_to.script_pubkey(), 50_000)
            .enable_rbf()
            .do_not_spend_change()
            .fee_rate(FeeRate::from_sat_per_vb(5.0));
        builder.finish()?
    };

    println!("Transaction details: {:#?}", details);
    println!("Unsigned PSBT: {}", &psbt);



    Ok(())
}