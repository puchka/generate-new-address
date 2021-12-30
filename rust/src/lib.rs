use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;

use bdk::Wallet;
use bdk::database::MemoryDatabase;
use bdk::bitcoin::Network;
use bdk::wallet::AddressInfo;
use bdk::wallet::AddressIndex::New;

fn get_new_address(desc: String) -> Result<AddressInfo, bdk::Error> {
    let mut external_descriptor = "wpkh(".to_owned();
    external_descriptor.push_str(desc.as_str());
    external_descriptor.push_str("/84'/0'/0'/0/*)");
    let mut internal_descriptor = String::from("wpkh(").to_owned();
    internal_descriptor.push_str(desc.as_str());
    internal_descriptor.push_str("/84'/0'/0'/1/*)");
    
    let wallet = Wallet::new_offline(
        external_descriptor.as_str(),
        Some(internal_descriptor.as_str()),
        Network::Testnet,
        MemoryDatabase::new(),
    )?;
    let address = wallet.get_address(New)?;
    return Ok(address);
}

#[no_mangle]
pub extern "system" fn Java_GenerateNewAddress_getNewAddress(env: JNIEnv,
                                                             _class: JClass,
                                                             desc: JString)
                                                             -> jstring {
    let desc: String =
        env.get_string(desc).expect("Couldn't get java string!").into();
    
    let output = env.new_string(format!("Generated Address: {}",
                                        get_new_address(desc).unwrap()))
        .expect("Couldn't create java string!");
    
    return output.into_inner();
}
