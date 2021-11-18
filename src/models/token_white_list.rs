#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WhiteTokenList {
    pub tokens: Vec<WhiteToken>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WhiteToken {
    pub address: String,
}

#[cfg(test)]
mod test {
    use crate::models::token_white_list::WhiteTokenList;

    #[tokio::test]
    async fn white_list_test_parse() {
        let str = r#"{
    "$schema": "https://raw.githubusercontent.com/broxus/ton-assets/master/schemas/manifest.json",
    "name": "TIP3 Tokens List",
    "version": {
        "major": 0,
        "minor": 0,
        "patch": 2
    },
    "keywords": [
        "ton",
        "tip3",
        "tokens"
    ],
    "timestamp": "2021-05-08T12:15:51.633Z",
    "tokens": [
        {
            "name": "Wrapped TON",
            "chainId": 1,
            "symbol": "WTON",
            "decimals": 9,
            "address": "0:0ee39330eddb680ce731cd6a443c71d9069db06d149a9bec9569d1eb8d04eb37",
            "logoURI": "https://raw.githubusercontent.com/broxus/ton-assets/master/icons/WTONv3/logo.svg",
            "version": 4
        },
        {
            "name": "Tether",
            "chainId": 1,
            "symbol": "USDT",
            "decimals": 6,
            "address": "0:751b6e22687891bdc1706c8d91bf77281237f7453d27dc3106c640ec165a2abf",
            "logoURI": "https://raw.githubusercontent.com/broxus/ton-assets/master/icons/USDTv3/logo.svg",
            "version": 4
        },
        {
            "name": "USD Coin",
            "chainId": 1,
            "symbol": "USDC",
            "decimals": 6,
            "address": "0:1ad0575f0f98f87a07ec505c39839cb9766c70a11dadbfc171f59b2818759819",
            "logoURI": "https://raw.githubusercontent.com/broxus/ton-assets/master/icons/USDCv3/logo.svg",
            "version": 4
        },
        {
            "name": "Dai Stablecoin",
            "chainId": 1,
            "symbol": "DAI",
            "decimals": 18,
            "address": "0:95934aa6a66cb3eb211a80e99234dfbba6329cfa31600ce3c2b070d8d9677cef",
            "logoURI": "https://raw.githubusercontent.com/broxus/ton-assets/master/icons/DAIv3/logo.svg",
            "version": 4
        },
        {
            "name": "Wrapped BTC",
            "chainId": 1,
            "symbol": "WBTC",
            "decimals": 8,
            "address": "0:6e76bccb41be2210dc9d7a4d0f3cbf0d5da592d0cb6b87662d5510f5b5efe497",
            "logoURI": "https://raw.githubusercontent.com/broxus/ton-assets/master/icons/WBTCv3/logo.svg",
            "version": 4
        },
        {
            "name": "Wrapped Ether",
            "chainId": 1,
            "symbol": "WETH",
            "decimals": 18,
            "address": "0:45f682b7e783283caef3f268e10073cf08842bce20041d5224c38d87df9f2e90",
            "logoURI": "https://raw.githubusercontent.com/broxus/ton-assets/master/icons/WETHv3/logo.svg",
            "version": 4
        },
        {
            "name": "Uniswap V2: USDT-WTON",
            "chainId": 1,
            "symbol": "UNI-V2-USDT-WTON",
            "decimals": 18,
            "address": "0:53abe27ec16208973c9643911c35b5d033744fbb95b11b5672f71188db5a42dc",
            "version": 4
        },
        {
            "name": "TON Bridge",
            "chainId": 1,
            "symbol": "BRIDGE",
            "decimals": 9,
            "address": "0:a453e9973010fadd4895e0d37c1ad15cba97f4fd31ef17663343f79768f678d9",
            "logoURI": "https://raw.githubusercontent.com/broxus/ton-assets/master/icons/BRIDGE/logo.svg",
            "version": 4
        },
        {
            "name": "Frax",
            "chainId": 1,
            "symbol": "FRAX",
            "decimals": 18,
            "address": "0:f8b0314571f5f00f6d9a61a914b9b5e1d910442d09b80c27efeb46631d74e961",
            "logoURI": "https://raw.githubusercontent.com/broxus/ton-assets/master/icons/FRAX/logo.svg",
            "version": 4
        },
        {
            "name": "Frax Share",
            "chainId": 1,
            "symbol": "FXS",
            "decimals": 18,
            "address": "0:0cddd021d2488c882041a31ba44e4ee69643a45223f068571e05b5a4c45bb7f6",
            "logoURI": "https://raw.githubusercontent.com/broxus/ton-assets/master/icons/FXS/logo.svg",
            "version": 4
        },
        {
            "name": "Sushi Token",
            "chainId": 1,
            "symbol": "SUSHI",
            "decimals": 18,
            "address": "0:8d3c9d6e1803d1c3ee22130a08b370c075c99eca9f4eb6dffa1d5bcc34c45eac",
            "logoURI": "https://raw.githubusercontent.com/broxus/ton-assets/master/icons/SUSHI/logo.svg",
            "version": 4
        },
        {
            "name": "Uniswap",
            "chainId": 1,
            "symbol": "UNI",
            "decimals": 18,
            "address": "0:471c9d737254a0044695c7e50ec5b8f6f94eadd49511b298d4a331b95106652b",
            "logoURI": "https://raw.githubusercontent.com/broxus/ton-assets/master/icons/UNI/logo.svg",
            "version": 4
        },
        {
            "name": "Aave Token",
            "chainId": 1,
            "symbol": "AAVE",
            "decimals": 18,
            "address": "0:b2e341c01da068d43cfa0eae6dae36b12b476e55cf2c3eeb002689f44b9ddef9",
            "logoURI": "https://raw.githubusercontent.com/broxus/ton-assets/master/icons/AAVE/logo.svg",
            "version": 4
        },
        {
            "name": "Compound",
            "chainId": 1,
            "symbol": "COMP",
            "decimals": 18,
            "address": "0:bc77ba7f3cbbebcca393e85ed479ef44df63cdee4fb572c3e0f904fb9fc63e25",
            "logoURI": "https://raw.githubusercontent.com/broxus/ton-assets/master/icons/COMP/logo.svg",
            "version": 4
        },
        {
            "name": "Curve DAO Token",
            "chainId": 1,
            "symbol": "CRV",
            "decimals": 18,
            "address": "0:7dd7ae82835848dc6b490515ec4034968a8ceff893a6d5f31ab3cdfcfb79bbb6",
            "logoURI": "https://raw.githubusercontent.com/broxus/ton-assets/master/icons/CRV/logo.svg",
            "version": 4
        },
        {
            "name": "STASIS EURS Token",
            "chainId": 1,
            "symbol": "EURS",
            "decimals": 2,
            "address": "0:6b2baa777b89da66dddaf9f1602142987b13ca565bbb170da929ea945f5ce9fb",
            "logoURI": "https://raw.githubusercontent.com/broxus/ton-assets/master/icons/EURS/logo.svg",
            "version": 4
        },
        {
            "name": "TORN Token",
            "chainId": 1,
            "symbol": "TORN",
            "decimals": 18,
            "address": "0:387609364f765017fa3fa5815e08d420e054c88a86426cd6d5aaf2a1ee46ff5a",
            "logoURI": "https://raw.githubusercontent.com/broxus/ton-assets/master/icons/TORN/logo.svg",
            "version": 4
        },
        {
            "name": "yearn.finance",
            "chainId": 1,
            "symbol": "YFI",
            "decimals": 18,
            "address": "0:e114f1f7d21ac6566d988c983315e0cdd5bee7b43c08918537d1117dea7e4534",
            "logoURI": "https://raw.githubusercontent.com/broxus/ton-assets/master/icons/YFI/logo.svg",
            "version": 4
        },
        {
            "name": "1INCH Token",
            "chainId": 1,
            "symbol": "1INCH",
            "decimals": 18,
            "address": "0:3c66e3e0ce0a909ce8779b31509db773e544132d8fa6f6641c00bce257c79d9c",
            "logoURI": "https://raw.githubusercontent.com/broxus/ton-assets/master/icons/1INCH/logo.svg",
            "version": 4
        },
        {
            "name": "dArtflex Token",
            "chainId": 1,
            "symbol": "DAF",
            "decimals": 18,
            "address": "0:bf1c7c0e8a187d9d5ba6069bf768b69a982df8b22ef8430b31dcc4f97263507e",
            "logoURI": "https://raw.githubusercontent.com/broxus/ton-assets/master/icons/DAF/logo.svg",
            "version": 4
        },
        {
            "name": "Fortune",
            "chainId": 1,
            "symbol": "FRTN",
            "decimals": 8,
            "address": "0:7ffa7b7e72224a9a2fba27386dfa71ed379bd9d541662671ab096e22110e5e96",
            "logoURI": "https://raw.githubusercontent.com/broxus/ton-assets/master/icons/FRTN/logo.svg",
            "version": 4
        },
        {
            "name": "EUR-Fiat-Based",
            "chainId": 1,
            "symbol": "EUPi",
            "decimals": 8,
            "address": "0:f4a912b0c3be422e02c0a8295590671cae5b38c75d397da8d1da33882888bbcb",
            "logoURI": "https://raw.githubusercontent.com/broxus/ton-assets/master/icons/EUPi/logo.svg",
            "version": 4
        }
    ]
}"#;
        let white_list: WhiteTokenList = serde_json::from_str(str).unwrap();
        let white_list: WhiteTokenList = reqwest::get(
            "https://raw.githubusercontent.com/broxus/ton-assets/master/manifest.json",
        )
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

        println!("{:#?}", white_list);
    }
}
