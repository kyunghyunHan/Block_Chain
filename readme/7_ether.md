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

## Transaction 구조

- Transacion은 EOA가 EOA에게 Eth를 전송하거나 EOA가 CA를 호출할떄 사용되는 구조
- 이 데이터는 블록체인 상에 기록

- txType : 거래버전
- copy :

- chainID : 1001 같은 테스트넷,메인넷 구별
- accessList :거의사용안함
- data : 중요한정보,컨트렉트호출정보
- gas: 가스 예상값 송금 21000고정
- gasPrice: 가스 지정값
- gasTipCap:
- gasFeeCap:
- value :송금하는 양
- nonce: 이중지불방지
- to:누구에게 보내는지 주소

- rawSignatrueValues :서명데이터(v,r,s)
- setSignatureValues :

## 서명(r,s,v)

- r,s:실제서명값
- v :복구값

## Message 구조

- CA가 CA를 호출할떄 발생하는 구조
- to
- from
- nonce
- amount
- gasLimit
- gasPrice
- gasFeeCap
- gasTipCap
- data :함수호출하는정보
- accessList
- isFake

## Receipt구조

- type
- PostState
- Status :성공실패여부
- CumulativeGasUsed :실제 연산 가스값
- Bloom
- Logs :이벤트 로그들

- TxHash :트랜잭션해시
- ContractAddress
- GasUsed:예측가스값

- BlockHash
- BlockNumber
- TransactionIndex :몇번쨰트랜잭션

## Meta Transacion

- User의 거래를 대신 실행해주는 거래방식
- ERC20토큰전송을 할떄 사용자가 ERC20 CA호출을 위해서 지불해야하는 ETH수수료를 대행업체가 대신납부할수 있다.

## EIP -2770

- smart contract코드상에서 사용자의 서명 검증하는 부분

## Gas

- gas란 이더리움에서 수수료에 사용되는 값을 의미
- smart Contract상 코드가 Compile되고 나오는 Byte Code를 OP_CODE테이블상에 지정된 Gas값으로 변환하여 수수료계산

## Gas Limit(Used)

- BlockGasLimit:해당 블록에 들어간 전체 트랜잭션 GasUsed합
- Tracnsaction GasUsed:해당 트랜잭션 실행에 들어간 Gas양의 합

## GasPrice

- 해당 트랜잭션을 얼마나 빠르게 실행할지를 결저아기 위한 값

## Gas계산법

- 거래수수료 :GasLimit \* GasPrice
- Block 보상:채굴보상+거래수수료 총합
- Uncle보상:특정 계산

## GasRefund

- Run Smart Contract:예상되는 가스 금액을 먼저계산하여 지출한뒤 실제 사용된 가스 제외 남은금액 리턴
- Out of Gas:가스가 부족한 경우 리턴
  - Selfdestruct:

## EIP-1559

- gas price값의 변동성이 너무커 일반사영자들이 사용하기 어려울 떄 지불할수 있는 수수료의 범위를 지정하고 최소값으로 지불할수 있는 방안

## Burn Fee

- EIP - 1559 의 경우 소각되는 ETH 양이만아 가격이 줄어드는것을 방지

## Ethash

- 이더리움의 pos
- Dag추가

## Difficulty

- 비트코인은 2016블록마다
- 이더리움은 매블록마다 13초마다
