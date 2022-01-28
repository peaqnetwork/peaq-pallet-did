
# Peaq Pallet DID

#### Introduction
The peaq DID standard supports machines to discover each other, carry out transactions, verify claims, preserve privacy and  remain sovereign.

A core value proposition of peaq DID is the standardization of machine interaction through a consortium of diverse industry leaders in a self sovereign world by providing standard interfaces for discovery which allows two participants to successfully engage in a transaction.

#### Installation
* Import the pallet dependecies by adding below snippets to your `runtime/src/Cargo.toml` file.
```
# --snip--

[dependencies.peaq-pallet-did]
default-features = false
git = 'https://github.com/peaqnetwork/peaq-pallet-did.git'
version = '0.0.1'

# --snip--

[features]
default = ['std']
runtime-benchmarks = [
  # --snip--
  'peaq-pallet-did/runtime-benchmarks',
]
std = [
  'peaq-pallet-did/std',
  # --snip--
]
```

* Implement peaq did pallet on your runtime by adding below snippets to `runtime/src/lib.rs` file.
```
# --snip--

pub use peaq_pallet_did;

# --snip--

/// Config the did in pallets/did
impl peaq_pallet_did::Config for Runtime {
	type Event = Event;
	type Public = sp_runtime::MultiSigner;
	type Signature = Signature;
	type Time = pallet_timestamp::Pallet<Runtime>;
}
```

* Add PeaqDid parameter type to the runtime construct on your `runtime/src/lib.rs` file using below snippet.
```
# --snip--
PeaqDid: peaq_pallet_did::{Pallet, Call, Storage, Event<T>},
# --snip--
```

### Usage
* After installation, build your node
* Run and connect your node to Polkadorjs App
* Check for `PeaqDid` under `developer - Extrinsics` tab.


### Peaq DID Format
The peaq DID method shall be identified by the method name peaq in lower case and follow the below given format

```
peaq-did = "did:peaq:" id-string 
id-string  = 1* idchar
idchar   = 1-9 / A-H / J-N / P-Z / a-k / m-z 
```

### Implementation:
This version of the MVP will involve creating a DID Pallet that has Four Extrinsics for managing DID:

#### Add Attribute:
* Add DID attribute that will compose the DID-document
* Allow any attribute to be added 
* No DID attribute validation on this version (no restriction for now until we align on our defined schema)

#### Update Attribute:
* Update DID attribute already existing on the DID-document
* Return Not Found Error if it doesn't exists.
* Owner can only update its DID-document

#### Read Attribute:
* Read DID attribute if it exist on the DID-document else return error
* Everyone can read DID-document

#### Remove Attribute:
* Delete DID attribute already existing on the DID-document
* Dispatch error if it doesn't exists
* Owner can only remove attributes they own.

For the purpose of simplicity and MVP v1 requirements, Authentication and Authorization was not added in this version. We are trying to create a foundation where more features like those mentioned above can be implemented on different iterations. We are proposing two type of DID-documents with different attributes although no restrictions on this current version but just to reference minimum attributes required on different types of DID. The following schema is defined based on our current use-cases.

### Consumer DID
```
{
  "id": "did:pq:5HRNr4pXH7PYKEmeW1jzJVxepXyg8w2Q3YpgRNHpH8foNr5i",
  "service": [
    {
      "id": "did:pq:5HRNr4pXH7PYKEmeW1jzJVxepXyg8w2Q3YpgRNHpH8foNr5i#payment",
      "type": "payment",
      "address": "5HRNr4pXH7PYKEmeW1jzJVxepXyg8w2Q3YpgRNHpH8foNr5i"
    }
  ]
}
```

### Provider DID

```
{
  "id": "did:pq:3HRNr4pXH7PYKEmeW1jzJVxepXyg8w2Q3YpgRNHpH8foNr5i",
  "service": [
    {
      "id": "did:pq:3HRNr4pXH7PYKEmeW1jzJVxepXyg8w2Q3YpgRNHpH8foNr5i#charging-station",
      "type": "service-offered",
      "serviceEndpoint": "http://device-url-out-of-peaq-network.some.domain.com/rest-schema"
    },
    {
      "id": "did:pq:3HRNr4pXH7PYKEmeW1jzJVxepXyg8w2Q3YpgRNHpH8foNr5i#metadata",
      "type": "metadata",
      "price": "10",
      "unit": "kWh"
    },
    {
      "id": "did:pq:3HRNr4pXH7PYKEmeW1jzJVxepXyg8w2Q3YpgRNHpH8foNr5i#payment",
      "type": "payment",
      "address": "5HRNr4pXH7PYKEmeW1jzJVxepXyg8w2Q3YpgRNHpH8foNr5i"
    }
  ]
}
```



## License

[Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)

