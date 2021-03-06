


use std::collections::HashMap;
/*
 * Inverse of the fields map
 *
*/
/*
Example:

pub enum LedgerEntryType {
    FullyCanonicalSig,
}

pub enum INVERSE_FIELDS_MAP { 
    LedgerEntryType  { name: LedgerEntryType   },
}

impl INVERSE_FIELDS_MAP {
    pub fn get(&self) -> Vec<u8> {
        match *self {
            //LedgerEntryType
            INVERSE_FIELDS_MAP::LedgerEntryType { ref name } => { 
            match name {
                FullyCanonicalSig => { vec![1, 2] }
            }}
        }
    }
}

let fl = INVERSE_FIELDS_MAP::LedgerEntryType{ name: LedgerEntryType::FullyCanonicalSig };
let n = fl.get();
println!("n : {:?}", n); //n : [1, 2]


example 2:
static X: [(&'static str, [u8; 2]); 2] = [("ledgertype", [1,2]),("transaction", [1,1])];
println!("x : {:?}", X[1].0);
*/


// pub static INVERSE_FIELDS_MAP: [(&'static str, [u8; 2]); 132] = [
lazy_static! {
    pub static ref INVERSE_FIELDS_MAP: HashMap<&'static str, [u8;2]> = {
        let m: HashMap<&'static str, [u8;2]> = [
            ("LedgerEntryType", [1, 1]),
            ("TransactionType", [1, 2]),
            ("Flags", [2, 2]),
            ("SourceTag", [2, 3]),
            ("Sequence", [2, 4]),
            ("PreviousTxnLgrSeq", [2, 5]),
            ("LedgerSequence", [2, 6]),
            ("CloseTime", [2, 7]),
            ("ParentCloseTime", [2, 8]),
            ("SigningTime", [2, 9]),
            ("Expiration", [2, 10]),
            ("TransferRate", [2, 11]),
            ("WalletSize", [2, 12]),
            ("OwnerCount", [2, 13]),
            ("DestinationTag", [2, 14]),
            ("Timestamp", [2, 15]),
            ("HighQualityIn", [2, 16]),
            ("HighQualityOut", [2, 17]),
            ("LowQualityIn", [2, 18]),
            ("LowQualityOut", [2, 19]),
            ("QualityIn", [2, 20]),
            ("QualityOut", [2, 21]),
            ("StampEscrow", [2, 22]),
            ("BondAmount", [2, 23]),
            ("LoadFee", [2, 24]),
            ("OfferSequence", [2, 25]),
            ("FirstLedgerSequence", [2, 26]),
            ("LastLedgerSequence", [2, 27]),
            ("TransactionIndex", [2, 28]),
            ("OperationLimit", [2, 29]),
            ("ReferenceFeeUnits", [2, 30]),
            ("ReserveBase", [2, 31]),
            ("ReserveIncrement", [2, 32]),
            ("SetFlag", [2, 33]),
            ("ClearFlag", [2, 34]),
            ("RelationType", [2, 35]),
            ("Method", [2, 36]),
            ("AppType", [2, 37]),
            ("Contracttype", [2, 39]),
            ("IndexNext", [3, 1]),
            ("IndexPrevious", [3, 2]),
            ("BookNode", [3, 3]),
            ("OwnerNode", [3, 4]),
            ("BaseFee", [3, 5]),
            ("ExchangeRate", [3, 6]),
            ("LowNode", [3, 7]),
            ("HighNode", [3, 8]),
            ("OfferFeeRateNum", [3,9]),
            ("OfferFeeRateDen", [3,10]),
            ("EmailHash", [4, 1]),
            ("LedgerHash", [5, 1]),
            ("ParentHash", [5, 2]),
            ("TransactionHash", [5, 3]),
            ("AccountHash", [5, 4]),
            ("PreviousTxnID", [5, 5]),
            ("LedgerIndex", [5, 6]),
            ("WalletLocator", [5, 7]),
            ("RootIndex", [5, 8]),
            ("AccountTxnID", [5, 9]),
            ("BookDirectory", [5, 16]),
            ("InvoiceID", [5, 17]),
            ("Nickname", [5, 18]),
            ("Amendment", [5, 19]),
            ("TicketID", [5, 20]),
            ("Amount", [6, 1]),
            ("Balance", [6, 2]),
            ("LimitAmount", [6, 3]),
            ("TakerPays", [6, 4]),
            ("TakerGets", [6, 5]),
            ("LowLimit", [6, 6]),
            ("HighLimit", [6, 7]),
            ("Fee", [6, 8]),
            ("SendMax", [6, 9]),
            ("MinimumOffer", [6, 16]),
            ("JingtumEscrow", [6, 17]),
            ("DeliveredAmount", [6, 18]),
            ("PublicKey", [7, 1]),
            ("MessageKey", [7, 2]),
            ("SigningPubKey", [7, 3]),
            ("TxnSignature", [7, 4]),
            ("Generator", [7, 5]),
            ("Signature", [7, 6]),
            ("Domain", [7, 7]),
            ("FundCode", [7, 8]),
            ("RemoveCode", [7, 9]),
            ("ExpireCode", [7, 10]),
            ("CreateCode", [7, 11]),
            ("MemoType", [7, 12]),
            ("MemoData", [7, 13]),
            ("MemoFormat", [7, 14]),
            ("Payload", [7, 15]),
            ("ContractMethod", [7, 17]),
            ("Parameter", [7, 18]),
            ("Account", [8, 1]),
            ("Owner", [8, 2]),
            ("Destination", [8, 3]),
            ("Issuer", [8, 4]),
            ("Target", [8, 7]),
            ("RegularKey", [8, 8]),
            ("FeeAccountID", [8, 9]),
            ("undefined", [15, 1]),
            ("TransactionMetaData", [14, 2]),
            ("CreatedNode", [14, 3]),
            ("DeletedNode", [14, 4]),
            ("ModifiedNode", [14, 5]),
            ("PreviousFields", [14, 6]),
            ("FinalFields", [14, 7]),
            ("NewFields", [14, 8]),
            ("TemplateEntry", [14, 9]),
            ("Memo", [14, 10]),
            ("Arg", [14, 11]),
            ("SigningAccounts", [15, 2]),
            ("TxnSignatures", [15, 3]),
            ("Signatures", [15, 4]),
            ("Template", [15, 5]),
            ("Necessary", [15, 6]),
            ("Sufficient", [15, 7]),
            ("AffectedNodes", [15, 8]),
            ("Memos", [15, 9]),
            ("Args", [15, 10]),
            ("CloseResolution", [16, 1]),
            ("TemplateEntryType", [16, 2]),
            ("TransactionResult", [16, 3]),
            ("ContractParamsType", [16, 4]),
            ("TakerPaysCurrency", [17, 1]),
            ("TakerPaysIssuer", [17, 2]),
            ("TakerGetsCurrency", [17, 3]),
            ("TakerGetsIssuer", [17, 4]),
            ("Paths", [18, 1]),
            ("Indexes", [19, 1]),
            ("Hashes", [19, 2]),
            ("Amendments", [19, 3]),
        ].iter().cloned().collect();

        m
    };
}
