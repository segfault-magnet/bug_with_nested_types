use fuels::{prelude::*, tx::ContractId};
use fuels_abigen_macro::abigen;

// Load abi from json
abigen!(MyContract, "out/debug/some_issue-abi.json");

async fn get_contract_instance() -> (MyContract, ContractId) {
    // Launch a local network and deploy the contract
    let wallet = launch_provider_and_get_single_wallet().await;

    let id = Contract::deploy("./out/debug/some_issue.bin", &wallet, TxParameters::default())
        .await
        .unwrap();

    let instance = MyContract::new(id.to_string(), wallet);

    (instance, id)
}

#[tokio::test]
async fn can_get_contract_id() {
    let (instance, _id) = get_contract_instance().await;
    let arg1 = AnotherEnum::En1(SomeStruct { par_1: SomeEnum::V1(1), par_2: 2 });
    let arg2 = SomeStruct { par_1: SomeEnum::V1(3), par_2: 4 };
    let arg3 = SomeEnum::V1(5);
    let result = instance.test_function(arg1, arg2, arg3).call().await.unwrap();

    // Now you have an instance of your contract you can use to test each function
}
