//! rosetta construction api implementation for bitcoin using mentat

use bitcoin::Transaction;

use super::*;

/// rosetta construction routes for bitcoin
#[derive(Clone, Default)]
pub struct BitcoinConstructionApi;

#[async_trait]
impl ConstructionApiRouter for BitcoinConstructionApi {}

#[async_trait]
impl ConstructionApi for BitcoinConstructionApi {
    type NodeCaller = BitcoinCaller;

    async fn combine(
        &self,
        _caller: Caller,
        data: ConstructionCombineRequest,
        _node_caller: &Self::NodeCaller,
    ) -> Result<ConstructionCombineResponse> {
        let mut tx = Transaction::deserialize(
            &hex::decode(data.unsigned_transaction)
                .merr(|e| format!("transaction malformed: {e}"))?,
        )?;
        for (vin, sig) in tx.input.iter_mut().zip(data.signatures) {
            vin.script_sig =
                Script::from(hex::decode(sig.bytes).merr(|e| format!("signature malformed: {e}"))?);
        }

        Ok(ConstructionCombineResponse {
            signed_transaction: hex::encode(tx.serialize()),
        })
    }

    async fn derive(
        &self,
        _caller: Caller,
        data: ConstructionDeriveRequest,
        node_caller: &Self::NodeCaller,
    ) -> Result<ConstructionDeriveResponse> {
        // NOTE: This will get P2PKH SegWit addresses.
        // Most exchanges implement this as standard nowadays.
        let descriptor = format!("wpkh({})", encode_to_hex_string(&data.public_key.bytes));
        let address = node_caller
            .rpc_call::<String>(BitcoinJrpc::new("deriveaddresses", &[descriptor]))
            .await?;
        Ok(ConstructionDeriveResponse {
            address: None,
            account_identifier: Some(AccountIdentifier {
                address,
                sub_account: None,
                metadata: IndexMap::new(),
            }),
            metadata: IndexMap::new(),
        })
    }

    async fn hash(
        &self,
        _caller: Caller,
        data: ConstructionHashRequest,
        node_caller: &Self::NodeCaller,
    ) -> Result<TransactionIdentifierResponse> {
        let hash = node_caller
            .rpc_call::<BitcoinTransaction>(BitcoinJrpc::new(
                "decoderawtransaction",
                &[data.signed_transaction],
            ))
            .await?
            .hash;
        Ok(TransactionIdentifierResponse {
            transaction_identifier: TransactionIdentifier { hash },
            metadata: IndexMap::new(),
        })
    }

    async fn metadata(
        &self,
        _caller: Caller,
        _data: ConstructionMetadataRequest,
        node_caller: &Self::NodeCaller,
    ) -> Result<ConstructionMetadataResponse> {
        let suggested_fee = node_caller
            .rpc_call::<FeeEstimate>(BitcoinJrpc::new(
                "estimatesmartfee",
                // NOTE: this might produce slower to confirm transactions, but they will be
                // cheaper.
                // May want to look into making this configurable.
                &[6],
            ))
            .await?
            .feerate;

        Ok(ConstructionMetadataResponse {
            metadata: Default::default(),
            suggested_fee: vec![Amount {
                value: suggested_fee.to_string(),
                currency: Currency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: Default::default(),
                },
                metadata: Default::default(),
            }],
        })
    }

    async fn parse(
        &self,
        _caller: Caller,
        data: ConstructionParseRequest,
        node_caller: &Self::NodeCaller,
    ) -> Result<ConstructionParseResponse> {
        let tx = BitcoinTransaction::from(Transaction::deserialize(
            &hex::decode(data.transaction).merr(|e| format!("transaction malformed: {e}"))?,
        )?);

        Ok(ConstructionParseResponse {
            operations: tx
                .clone()
                .into_transaction(0, node_caller)
                .await?
                .operations,
            signers: Vec::new(),
            account_identifier_signers: if data.signed {
                let vin_len = tx.vin.len();
                let hash = tx.hash.clone();
                tx.vout
                    .into_iter()
                    .enumerate()
                    .filter_map(|(i, vout)| vout.into_operation(i + vin_len, &hash).account)
                    .collect()
            } else {
                Vec::new()
            },
            metadata: Default::default(),
        })
    }

    // todo 0rphon: can clean this up once generalized jsonrpc_call is merged into
    // this branch
    async fn payloads(
        &self,
        _caller: Caller,
        data: ConstructionPayloadsRequest,
        node_caller: &Self::NodeCaller,
    ) -> Result<ConstructionPayloadsResponse> {
        let mut tx = Transaction {
            version: 2,
            lock_time: PackedLockTime(0),
            input: vec![],
            output: vec![],
        };

        let coins = data
            .metadata
            .get("coins")
            .merr(|| "no coins provided")?
            .as_array()
            .merr(|| "malformed coins field in metadata")?;
        for coin in coins {
            let (txid, vout) = coin
                .get("coin_identifier")
                .merr(|| "no coin identifier on coin struct")?
                .as_str()
                .merr(|| "coin identifier is wrong type")?
                .split_once(':')
                .merr(|| "invalid coin identifier format")?;
            tx.input.push(TxIn {
                previous_output: OutPoint {
                    txid: Txid::from_str(txid).merr(|e| format!("invalid txid `{txid}`: {e}"))?,
                    vout: vout
                        .parse::<u32>()
                        .merr(|e| format!("invalid vout field `{vout}`: {e}"))?,
                },
                // This gets filled in later in `combine`.
                script_sig: Script::new(),
                sequence: Sequence(u32::MAX),
                witness: Witness::new(),
            });
        }

        let mut payloads = vec![];
        for (i, input) in tx.input.iter().enumerate() {
            let script_pub_key = node_caller
                .rpc_call::<BitcoinTransaction>(BitcoinJrpc::new(
                    "getrawtransaction",
                    &[json!(input.previous_output.txid.to_string()), json!(true)],
                ))
                .await?
                .vout
                .into_iter()
                .nth(input.previous_output.vout as usize)
                .unwrap()
                .scriptPubKey;

            payloads.push(SigningPayload {
                address: None,
                account_identifier: None,
                bytes: tx
                    .signature_hash(i, &script_pub_key.try_into()?, 0)
                    .to_vec(),
                signature_type: Some(SignatureType::Ecdsa),
            });
        }

        for op in data.operations {
            if op.type_ == "OUTPUT" {
                tx.output.push(TxOut {
                    value: op
                        .amount
                        .merr(|| "no amount for payment operation")?
                        .value
                        .parse::<isize>()
                        .merr(|e| format!("invalid value: {e}"))? as u64,
                    script_pubkey: Script::new_p2pkh(
                        &PubkeyHash::from_str(
                            &op.account
                                .merr(|| "no account for payment operation")?
                                .address,
                        )
                        .merr(|e| format!("invalid address: {e}"))?,
                    ),
                })
            }
        }

        Ok(ConstructionPayloadsResponse {
            unsigned_transaction: hex::encode(tx.serialize()),
            payloads,
        })
    }

    async fn preprocess(
        &self,
        _caller: Caller,
        data: ConstructionPreprocessRequest,
        _node_caller: &Self::NodeCaller,
    ) -> Result<ConstructionPreprocessResponse> {
        let mut options = IndexMap::new();

        let coins: Vec<Coin> = data
            .operations
            .iter()
            .filter_map(|operation| {
                if let (Some(coin_change), Some(amount)) =
                    (&operation.coin_change, &operation.amount)
                {
                    Some(Coin {
                        coin_identifier: coin_change.coin_identifier.clone(),
                        amount: amount.clone(),
                    })
                } else {
                    None
                }
            })
            .collect();

        options.insert("coins".to_string(), json!(coins));
        Ok(ConstructionPreprocessResponse {
            options,
            required_public_keys: data
                .operations
                .into_iter()
                .filter_map(|operation| {
                    if operation.account.is_some() {
                        operation.account
                    } else {
                        None
                    }
                })
                .collect(),
        })
    }

    async fn submit(
        &self,
        _caller: Caller,
        data: ConstructionSubmitRequest,
        node_caller: &Self::NodeCaller,
    ) -> Result<TransactionIdentifierResponse> {
        let hash = node_caller
            .rpc_call::<String>(BitcoinJrpc::new(
                "sendrawtransaction",
                &[data.signed_transaction],
            ))
            .await?;
        Ok(TransactionIdentifierResponse {
            transaction_identifier: TransactionIdentifier { hash },
            metadata: IndexMap::new(),
        })
    }
}