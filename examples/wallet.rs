
extern crate rand;
use rand::Rng;

static ALPHABET: &[u8] = b"jpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65rkm8oFqi1tuvAxyz";
const PASSWORD_LEN: usize = 16;
pub const JTM: &'static [u8; 58] = b"jpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65rkm8oFqi1tuvAxyz";

////////////////////////////////////////////////////////////////////////////////////////

//Keypairs
pub struct Keypairs {

}

impl Keypairs {

    /*
    after encode x :  ssLGemsXGu9rKnhnotfhT2bbfvMYY
secret :  ssLGemsXGu9rKnhnotfhT2bbfvMYY
keypair :  { privateKey:
   '00F97FCE78D5CC0B5F960FE0EAE1DFEBA7D7A3541028ADC83C03527A9DF3115C0E',
  publicKey:
   '02AE68BD169ADB3D76690416B532F2931272AF928EA14A5730C64F3A6928170409' }
    */
    pub fn deriveKeyPair(secret: String) {
        let prefix = "00";

        extern crate bs58;
        let buf = bs58::decode("ssndDcNoc4FvwVPveY3KbfWh8fNh3")
            .with_alphabet(JTM)
            .into_vec().unwrap();
        println!("deriveKeyPair decoded: {:?}", buf);

        //取值index范围，1 ~ 倒数第5
        let entropy = &buf[1..buf.len()-4];
        println!("entropy : {:?}", entropy);

        // let privateKey = prefix.to_owned() + derivePrivateKey(entropy).toString(16, 64).toUpperCase();
        // var publicKey = bytesToHex(ec.keyFromPrivate(privateKey.slice(2)).getPublic().encodeCompressed());

        // return { privateKey: privateKey, publicKey: publicKey };

    }
    // pub fn derivePrivateKey(entropy: &Vec<u8>) {
    //     var order = ec.curve.n;

    //     var privateGen = secp256k1.ScalarMultiple(seed);
    //     var publicGen = ec.g.mul(privateGen);
    //     return secp256k1.ScalarMultiple(publicGen.encodeCompressed(), 0).add(privateGen).mod(order);

    // }


    pub fn brorand(len: usize) -> Vec<u8> {
        let u: Vec<u8> = (0..len).map(|_| {
            let idx: u8 = rand::thread_rng().gen();
            
            //pub fn format(args: Arguments) -> String
            let hexs = format!("{:x}", idx);
            println!("hexs : {}", hexs);

            // hexs
            idx
        }).collect::<Vec<u8>>();

        u
    }

    pub fn generateSeed() {
        //1. Generete 16 random data
        let mut u: Vec<u8> = Keypairs::brorand(PASSWORD_LEN);
        println!("u: {:?}", u);

        //2. add secp256k1
        let opt = "secp256k1";

        //3. encodeSeed function
        let mut version: Vec<u8> = [33].to_vec();

        //4. concat args
        Keypairs::concat_args(&mut version, &u);

        //5. encodechecked.
        let mut checked: Vec<u8> = Keypairs::encode_checked(&mut version);

        //6. concat args
        Keypairs::concat_args(&mut version, &checked);

        let secret: String = Keypairs::encode_raw(&mut version);
        println!("secret : {}", secret);
    }

    pub fn concat_args(left: &mut Vec<u8>, right: &Vec<u8>) {
        println!("before concat args: {:?}", left);

        //append vs.extend
        left.extend(right); 
        
        println!("after concat args : {:?}", left);
    }

    pub fn encode_checked(x: &mut Vec<u8>) -> Vec<u8> {
        use ring::{digest, test};

        //let vv: &[u8] = &[ 33, 228, 98, 120, 229, 208, 105, 36, 76, 162, 155, 0, 178, 95, 45, 115, 89 ];
        let vv: &[u8] = x.as_slice();
        
        let ac = digest::digest(&digest::SHA256, vv);
        let checked = digest::digest(&digest::SHA256, &ac.as_ref());
        let xx: Vec<u8> = checked.as_ref().iter().map(|c| {
            let x = format!("{:x}", c);
            x.as_str().chars().nth(0).unwrap() as u8
        }).collect::<Vec<u8>>();
        println!("checked : {:?}", xx.get(..4));

        xx.get(..4).unwrap().to_vec()
    }


//         let raw: &[u8] =  &[ 33,
//   228,
//   98,
//   120,
//   229,
//   208,
//   105,
//   36,
//   76,
//   162,
//   155,
//   0,
//   178,
//   95,
//   45,
//   115,
//   89,
//   57,
//   48,
//   147,
//   236 ];
//         Keypairs::encode_raw(raw);//Keypairs::concat_args(buffer, check));


    pub fn encode_raw(x: &mut Vec<u8>) -> String {
            Keypairs::encode(x.as_mut_slice())
    }

    pub fn encode(source: &[u8]) -> String {
            // if (source.length === 0) return ''

            println!("source : {:?} and len : {}", source, source.len());
            
        // let source = &source[..4];
        let BASE = ALPHABET.len() as u16;

        let mut digits: Vec<u16> = vec![0u16; 1];
        
        let mut i = 0;
        while i < source.len() {

            let mut j = 0;
            let mut carry: u16 = source[i] as u16;
            
            let digits_len = digits.len();
            while j < digits_len {
                carry += digits.as_slice()[j] << 8;

                digits.as_mut_slice()[j] = carry % (BASE as u16);

                carry = (carry / (BASE as u16)) | 0;

                j += 1;
            }

            while carry > 0 {
                digits.push(carry % (BASE as u16));
                carry = (carry / BASE) | 0;
            }

            i += 1;
        }

        let mut string = "".to_string();

        // deal with leading zeros
        let mut k = 0;
        while source[k] == 0 && k < source.len() - 1 {

            string += ALPHABET[0].to_string().as_str();

            k += 1;
        }        
        // convert digits to a string
        let mut q: i32 = (digits.len() - 1) as i32;
        while q >= 0 {

            // string += ALPHABET[digits[q as usize] as usize].to_string().as_str();

            let uu: u8 = ALPHABET[digits[q as usize] as usize];
            let xx = uu as char;
            string.push( xx );

            // println!("string : {}", xx);
            q -= 1;
        }

        String::from(string.as_str())
    }
}

////////////////////////////////////////////////////////////////////////////////////////
//Wallet

pub struct Wallet {
    pub keypairs: String, //...
    pub secret: String,//
}

impl Wallet {
    //generate
    pub fn generate() {
        let secret = Keypairs::generateSeed();        
        // let keypair = Keypairs::deriveKeyPair(secret);
        // let address = Keypairs::deriveAddress(keypair.publicKey);

        // GenReturn {
        //     secret: secret,
        //     address: address
        // }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////
fn main() {
    // Wallet::generate();

    Keypairs::deriveKeyPair("to".to_string());
}