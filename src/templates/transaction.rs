use super::*;

#[derive(Boilerplate)]
pub(crate) struct TransactionHtml {
  blockhash: Option<BlockHash>,
  chain: Chain,
  inscription: Option<InscriptionId>,
  transaction: Transaction,
  txid: Txid,
}

impl TransactionHtml {
  pub(crate) fn new(
    transaction: Transaction,
    blockhash: Option<BlockHash>,
    inscription: Option<InscriptionId>,
    chain: Chain,
  ) -> Self {
    Self {
      txid: transaction.txid(),
      blockhash,
      chain,
      inscription,
      transaction,
    }
  }
}

impl PageContent for TransactionHtml {
  fn title(&self) -> String {
    format!("Transaction {}", self.txid)
  }
}

#[cfg(test)]
mod tests {
  use {
    super::*,
    bitcoin::{blockdata::script, PackedLockTime, TxOut},
  };

  #[test]
  fn html() {
    let transaction = Transaction {
      version: 0,
      lock_time: PackedLockTime(0),
      input: Vec::new(),
      output: vec![
        TxOut {
          value: 50 * COIN_VALUE,
          script_pubkey: script::Builder::new().push_int(0).into_script(),
        },
        TxOut {
          value: 50 * COIN_VALUE,
          script_pubkey: script::Builder::new().push_int(1).into_script(),
        },
      ],
    };

    let txid = transaction.txid();

    pretty_assert_eq!(
      TransactionHtml::new(transaction, None, None, Chain::Mainnet).to_string(),
      format!(
        "
        <h1>Transaction <span class=monospace>{txid}</span></h1>
        <h2>2 Outputs</h2>
        <ul class=monospace>
          <li>
            <a href=/output/{txid}:0 class=monospace>
              {txid}:0
            </a>
            <dl>
              <dt>value</dt><dd>5000000000</dd>
              <dt>script pubkey</dt><dd class=data>OP_0</dd>
            </dl>
          </li>
          <li>
            <a href=/output/{txid}:1 class=monospace>
              {txid}:1
            </a>
            <dl>
              <dt>value</dt><dd>5000000000</dd>
              <dt>script pubkey</dt><dd class=data>OP_PUSHNUM_1</dd>
            </dl>
          </li>
        </ul>
      "
      )
      .unindent()
    );
  }

  #[test]
  fn with_blockhash() {
    let transaction = Transaction {
      version: 0,
      lock_time: PackedLockTime(0),
      input: Vec::new(),
      output: vec![
        TxOut {
          value: 50 * COIN_VALUE,
          script_pubkey: script::Builder::new().push_int(0).into_script(),
        },
        TxOut {
          value: 50 * COIN_VALUE,
          script_pubkey: script::Builder::new().push_int(1).into_script(),
        },
      ],
    };

    assert_regex_match!(
      TransactionHtml::new(transaction, Some(blockhash(0)), None, Chain::Mainnet),
      "
        <h1>Transaction <span class=monospace>[[:xdigit:]]{64}</span></h1>
        <dl>
          <dt>block</dt>
          <dd><a href=/block/0{64} class=monospace>0{64}</a></dd>
        </dl>
        .*
      "
      .unindent()
    );
  }
}
