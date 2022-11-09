# ether

- EVM실행후에 확인 가능하여 정상적인 거래

## 스마트컨트렉트

- state를 두고 새로운 input과 state를 evm에서 트랜잭션 실행해서 나온 결괏값과 동일하면 승인

## evm

- 스마트컨트렉트를 실행시킬수 있는환경

## stoage

- 실행결과 값을 저장

## state기반 Account

- 비트코인경우 utxo기반으로 상택밧 관리
- utxo사용여부를 업데이트 하는 식으로 사용자의 잔액이 관리
- 이더리움의 경우 state기반으로 잔액,데이터 상태값이 저장되어 블록이 생기때마다 잔액을 업데이트

## Account종류

- EOA :프라이빗키를 소유한 사람이 관리하는 방식으로 비트코인 PKI와 동일한 방식으로 관리,계정생성에 비용이 발생하지 않는다
- CA :생성한 사용자의 정보에 의해서 네트워크에서 Account를 생성,계정생성에 코드의 크기에 따른 비용이 발생,비용은 생성자가 지불

## Account state

- nonce :계정에서 전송한 트랜잭션의 수
- balance:계정의 잔고
- codehash:CA만 가진 데이터,Contract Code의 Hash의정보가 들어간다
- storageRoot:Merkle Patricia Tree의 root node의 Hash값이다.

## Patricia Merkle Tree

- 사용자의 해시값을 쉽게 가져올수있는

## Eth의 tries

- 이더리움에서 사용하고 있는 트리는 storage trie,state trie,transaction trie,Receipts Trie
- 이중 Blockheader에 Root Hash값이 저장되는 갓은 State trie,transaction trie,Receipts Trie

## EOA생성(타원곡성 )

- Keccak -256 Hash (결과값 하위 20bytes)->주소
- private key
- public key

## CA

## Address Checksum

- 이더리움의경우 Checksum이 들어가지 않기 떄문에 정확한 주소인지 확인하기 어렵다

## ENS

- DNS와 같이 이해하기 어려운 언어로 등록하여 사용

## 블록구조

- 헤더
- uncles : 사이드블록체인들
- transactions
  //caches 생성됫다가 사라짐
- hash
- size
- ReceivedAt :
- ReceivedFrom

## 블록헤더구조

- ParentHash :이번블록의 해시
- UncleHash: uncle해시
- CoinBase: 코인베이스
- Root:state Root
- TxHash:트랜잭션패트리샤루트루트
- ReceipHash:영수증 머클패트리샤루트
- Bloom:logsBloom
- Difficulty:난이더
- Number:몇번쨰블록인지
- GasLimit:가스리밋 ,
- GasUsed:가스값의 총합
- Time:현재시간
- Extra:블록에 저장하고싶은 데이터
- MixDigest:노력증명
- Nonce

- BaseFee:

## 블록생성

- Mempool Tx선택
- EVM실행(이전트리 불로엄)
- Transaction실행
- Block Header생성
- FindNonce,MixHash(ethHash)
- Block전파

## Uncle Block

- 블록체인에서 Fork된 OrphanBlock
- 블록헤더만 존재, 트랜잭션 없는 블록
- GHOST Protocol로 인해 인정받는다,이를 채굴한 채굴자와 이를 포함한 채굴자 모두 보상을 얻게된다.
- 사이드로 생성한것을 인정

## Block Gas Limit

- 블록 크기는 따로 제한이 업지만
- GasLimit값이 존재하며 GasLimit이 넘는 Tx데이터는 들어갈수 없다.
