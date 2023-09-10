use ethers::providers::Middleware;
use eyre::Result;
use std::env;
use std::process::Command;

// const target_directory: &str = "/Users/ouhuang/Documents/hello_axum/hello_wtf";
const TARGET_DIRECTORY: &str = "/Users/ouhuang/Documents/hello_axum/hello_wtf";

#[tokio::test]
async fn deploy_contract_test() {
    // 切换到目标目录
    let target_directory = "/Users/ouhuang/Documents/hello_axum/hello_wtf";
    if let Err(err) = env::set_current_dir(target_directory) {
        eprintln!("Failed to change directory: {}", err);
        return;
    }

    // deploy_verify_counter().await;
    // forge create --rpc-url https://eth-goerli.g.alchemy.com/v2/wlXHln-ov4d7diXYhbVCKIm9s3pjnf2l \
    // --constructor-args 1000000000000000000000 \
    // --private-key 0x0d93fe15801e6ea1cda06391e59fa69cd97fdd45e4038ebb29cf65e882d0d4b3 \
    // --etherscan-api-key Q2TH6RVMM4UMBRSP7VDCR4CGF5T2INC3HZ \
    // --verify \
    // src/CleanErc20.sol:GLDToken

    let private_key = "0x0d93fe15801e6ea1cda06391e59fa69cd97fdd45e4038ebb29cf65e882d0d4b3";
    let rpc_url = "https://eth-goerli.g.alchemy.com/v2/wlXHln-ov4d7diXYhbVCKIm9s3pjnf2l";

    let etherscan_api_key = "Q2TH6RVMM4UMBRSP7VDCR4CGF5T2INC3HZ";
    // let verify_args = "src/CleanErc20.sol:GLDToken";
    // let constructor_args = "999999999999999999999999999";
    let constructor_args =
        "pixiu2 px2 18 888888888888888888 0x04D178F683Be8aD48ed718Bb49A71405DD6dA2f5";
    let verify_args = "src/Pixu.sol:PigLido";
    let deployed_res = deploy_verify_contract(
        rpc_url,
        constructor_args,
        private_key,
        etherscan_api_key,
        verify_args,
    )
    .await;
    match deployed_res {
        Ok(res) => println!("deployed_res: {}", res),
        Err(err) => println!("deployed_res err: {}", err),
    }
}
pub async fn deploy_a_clean_erc20() -> Result<String> {
    // 切换到目标目录
    let target_directory = "/Users/ouhuang/Documents/hello_axum/hello_wtf";
    if let Err(err) = env::set_current_dir(target_directory) {
        eprintln!("Failed to change directory: {}", err);
        return Ok(("Failed to change directory").to_string());
    }
    let private_key = "0x0d93fe15801e6ea1cda06391e59fa69cd97fdd45e4038ebb29cf65e882d0d4b3";
    let rpc_url = "https://eth-goerli.g.alchemy.com/v2/wlXHln-ov4d7diXYhbVCKIm9s3pjnf2l";

    let etherscan_api_key = "Q2TH6RVMM4UMBRSP7VDCR4CGF5T2INC3HZ";
    let verify_args = "src/CleanErc20.sol:GLDToken";
    let constructor_args = "999999999999999999999999999";
    // let constructor_args =
    //     "pixiu2 px2 18 888888888888888888 0x04D178F683Be8aD48ed718Bb49A71405DD6dA2f5";
    // let verify_args = "src/Pixu.sol:PigLido";
    let deployed_res = deploy_verify_contract(
        rpc_url,
        constructor_args,
        private_key,
        etherscan_api_key,
        verify_args,
    )
    .await;

    match deployed_res {
        Ok(res) => {
            // get transaction hash from res. the hash str start with 0x and length is 66
            let hash_str = res.split("0x").collect::<Vec<&str>>()[1];
            return Ok(hash_str.to_string());
        }
        Err(err) => Ok(err.to_string()),
    }
}

pub async fn deploy_verify_contract(
    rpc_url: &str,
    constructor_args: &str,
    private_key: &str,
    etherscan_api_key: &str,
    verify_args: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // 切换到目标目
    if let Err(err) = env::set_current_dir(TARGET_DIRECTORY) {
        eprintln!("Failed to change directory: {}", err);
        return Ok(("Failed to change directory").to_string());
    }
    let output = Command::new("forge")
        .arg("create")
        .arg("--rpc-url")
        .arg(rpc_url)
        .arg("--constructor-args")
        .args(constructor_args.split(" "))
        .arg("--private-key")
        .arg(private_key)
        .arg("--etherscan-api-key")
        .arg(etherscan_api_key)
        .arg("--verify")
        .arg(verify_args)
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // return transaction hash
        let transaction_hash = stdout.split("Transaction hash: ").collect::<Vec<&str>>()[1];
        return Ok(transaction_hash.to_string());
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // return error reason
        return Err(stderr.into());
    }
}

pub async fn create_new_wallet() -> Result<(String, String)> {
    let output = Command::new("cast")
        .arg("wallet")
        .arg("new")
        .output()
        .unwrap();
    if output.status.success() {
        // 命令执行成功
        let stdout = String::from_utf8_lossy(&output.stdout);
        // 在输出中查找 "Address:" 和 "Private key:" 字符串
        let address_start = stdout.find("Address:").unwrap_or(0);
        let private_key_start = stdout.find("Private key:").unwrap_or(0);
        // 提取地址和私钥字符串，并去掉前面的 "Address:" 和 "Private key:" 部分
        let address_str = stdout[address_start + "Address:".len()..private_key_start].trim();
        let private_key_str = stdout[private_key_start + "Private key:".len()..].trim();
        return Ok((address_str.to_string(), private_key_str.to_string()));
    } else {
        // 命令执行失败
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed with error: {}", stderr);
        return Ok(("error".to_string(), "error".to_string()));
    }
}
// cast estimate \
// --rpc-url https://eth-goerli.g.alchemy.com/v2/wlXHln-ov4d7diXYhbVCKIm9s3pjnf2l \
// --create $(forge inspect src/Pixu.sol:PigLido bytecode)00000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000e000000000000000000000000000000000000000000000000000000000000000120000000000000000000000000000000000000000000000000c55f7bc23038e3800000000000000000000000004d178f683be8ad48ed718bb49a71405dd6da2f50000000000000000000000000000000000000000000000000000000000000005706978697500000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000027078000000000000000000000000000000000000000000000000000000000000
// 实时计算合约部署的gas费用 估算
pub async fn estimate_deploy_gas_fee(
    rpc_url: &str,
    data: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // 切换到目标目
    if let Err(err) = env::set_current_dir(TARGET_DIRECTORY) {
        eprintln!("Failed to change directory: {}", err);
        return Ok(("Failed to change directory").to_string());
    }
    let mut bytecode_str = String::new();
    // 先执行 $(forge inspect src/Pixu.sol:PigLido bytecode)
    let bytecode = Command::new("forge")
        .arg("inspect")
        .arg("src/Pixu.sol:PigLido")
        .arg("bytecode")
        .output()
        .expect("Failed to execute command");
    if bytecode.status.success() {
        let stdout = String::from_utf8_lossy(&bytecode.stdout);
        // remove \n
        bytecode_str = stdout.replace("\n", "");
    } else {
        let stderr = String::from_utf8_lossy(&bytecode.stderr);
        return Err(stderr.into());
    }
    let output = Command::new("cast")
        .arg("estimate")
        .arg("--rpc-url")
        .arg(rpc_url)
        .arg("--create")
        .arg(bytecode_str + data)
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // remove \n
        let stdout = stdout.replace("\n", "");
        return Ok(stdout.to_string());
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // return error reason
        return Err(stderr.into());
    }
}

#[tokio::test]
pub async fn estimate_deploy_gas_fee_test() {
    let rpc_url = "https://eth-goerli.g.alchemy.com/v2/wlXHln-ov4d7diXYhbVCKIm9s3pjnf2l";
    // new ethers provider
    let provider = ethers::providers::Provider::try_from(rpc_url).unwrap();
    //  get gas price gwei
    let (max_fee_wei, _max_priority_fee_wei) = provider.estimate_eip1559_fees(None).await.unwrap();
    let signature_str = "constructor(string memory name_, string memory symbol_, uint8 decimals_, uint256 totalSupply_ , address UGHFDGD)";
    let args_string = "pixiu2 px2 18 888888888888888888 0x04D178F683Be8aD48ed718Bb49A71405DD6dA2f5";
    let data = abi_encode(signature_str, args_string).await.unwrap();
    let res = estimate_deploy_gas_fee(rpc_url, &data).await.unwrap();
    println!("res: {:?}", res.replace("\n", ""));
    // callculate gas cost = gas_price * gas_used * 10^9
    let gas_used_wei = res.parse::<u64>().unwrap();
    println!("max gas price wwei: {:?}", max_fee_wei);
    let gas_cost = max_fee_wei * gas_used_wei;
    // format uints to readable string
    let gas_cost = ethers::utils::format_units(gas_cost, "ether").unwrap();

    // convert gas_cost to readable string
    println!("gas_cost: {:?}", gas_cost);
}

// abi encode
async fn abi_encode(
    signature_str: &str,
    args_string: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // open the target dir
    // 切换到目标目
    if let Err(err) = env::set_current_dir(TARGET_DIRECTORY) {
        eprintln!("Failed to change directory: {}", err);
        return Ok(("Failed to change directory").to_string());
    }
    // call commad
    let output = Command::new("cast")
        .arg("abi-encode")
        .arg(signature_str)
        .args(args_string.split(" "))
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // remove \n and 0x
        let stdout = stdout.replace("\n", "").replace("0x", "");
        return Ok(stdout.to_string());
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // return error reason
        return Err(stderr.into());
    }
}

// test abi encode
#[tokio::test]
pub async fn abi_encode_test() {
    let signature_str = "constructor(string,string,uint8,uint256,address)";
    let args_string = "pixiu2 px2 18 888888888888888888 0x04D178F683Be8aD48ed718Bb49A71405DD6dA2f5";
    let res = abi_encode(signature_str, args_string).await.unwrap();
    println!("res: {:?}", res);
}

// test
#[tokio::test]
pub async fn create_new_wallet_test() {
    let res = create_new_wallet().await.unwrap();
    println!("res: {:?}", res);
}

fn au8_to_string(signature_code: Vec<u8>) -> String {
    let mut private_key = String::new();
    for a in signature_code.iter() {
        let fstr = format!("{:02x}", a); //将二进制元素转换为16进制输出
        private_key.push_str(&fstr);
    }
    private_key
}
