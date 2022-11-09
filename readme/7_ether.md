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

## Block검증

- 신규블록수신
- 블록구조 일치여부
- 재계산 Block header Hash ==Block header Hash
- Block difficulty Check
- Block GasLimit>30,000,000
- Uncle Block Check
- Transaction재실행
- Event,Receipt생성
- Bloom Filter 비교
- Receipt Root,Merkle Root비교
- LevelDB기록

## Ghost Protocol

- 가장 많은 서브트리가 있는것을 체택
- Uncleq블록이 많은 블록

## Smart Contract

- 서비스 Logic을 네트워크에 등록
- 사용자는 Contract를 실행하게 되면 그 Contract의 State를 변경 조회

## 개발언어

- Solidity :가장 많이 사용
- Vyper :python기반
- Yul :어려운언어
- FE

## Compile Contract

- 스마트컨트렉트를 컴파일하게되면
- BYtecode:네트워크배포
- OPCODE:gas계산용
- ABI:로직을 제외하고 명시,통신

## Contract접근

- ABI,ContractAddress가필요

## Contract 호출

- 조회하는경우
- 수정하게되면 state변경하게되면 트랙젹션 기록

## Events

- 이벤트를 포함한 스마트컨트렉 배포
- 스마트컨트렉트 상에서 사용자가 알림을 받고싶은 내용을 등록
- 이를 통하여 거래의 생성이나 변경등을 확인

## Event 등록

- emit을 이용햇 Event빌셍
- LOG로 보여주고 싶은 데이터를 지정

## DAPP

- 운영자 없이 사용자들이 smart contract상에서 운영되는 서비스
- Defi,nft market
- 중앙화 서비스 센터의 장점을 이용할수 없음

## 네트워크 접근방법

- node:GETH-노드운영,조회
- Library:web3.js, caver.js
- Wallet이용:월렛이용

## Lottery System

- 누구나확인가능하기때문에 조작이불가능

## Vote Sysyem

- 익명성을이용 투표가능

## Inital Coin Offering

- ICO란 블록체인 플랫폼 개발 자금 확보를 위해서 투자자들이 개발자금을 투입하면 해당하는 금액만큼 Token을 발행해주는 것ㄴ
- Soft Cap,Hard Cap등이 존재,특정자금이상이 모이지 않으면 자동 취소

- 가상자산 시장이 활발하게이루어짐

## Token Disrtibution

- ICO 의 가장 큰 장점은 서비스 오픈전 사용자에게 투자금을 지급받고 개발자와 서비스개발에 필요한 토큰을 미리 할당할수 있다.

## ICO 대안

- IEO:중앙화된 거래소에서 초기발행을 도와주는방법,무분별한 시장대신 거래소가 선별하는 효과가 있음
- IDO:탈중앙화된 거래소에서 지원하는 발행방식.해당 DEX의 투표를 통해서 지원 여부를 결정

## ERC20장점

- 1.메인넷 개발없이 사용자에게 가치를 가질수 있는 토큰을 스마트컨트렉트 상으로 게발하여배포할수 있다
- 발행량이 제한되어있는 ETH와 달리 초기 발행량을 조절
- Eth와 동일한 보안성

- ETH와 마찬가지로 블록체인상에서 관리되기에 투명성 제공이 가능
- CA의 스토리지상에 저장

## ERC20 표준

- EIP20으로 제안된 새로운 토큰의 표준스펙
- 9가지의 Function 2가지의 Event를 기본적으로 포함되어야 ERC20으로 보고있다.

## Stable Coin (3가지 유형)

- 1달라의 가치에 페깅되는 토큰,항상 1달라의 가치를 유지
- MarkerDAO DAI가 대표적,Maker이라는 담보 관리자가 일정 수수료를 지급받으며 담보의 가치를 조정 ,리스크 관리 업무를 증명
- 유동성 차이를 이용한 알고리즘을 통해서 관리

## Wrapped ETH

- ERC20:ETH를 맡기고 동일한 수량의 wETH로변환.wBTC,wTerra등 다양한 메인넷 코인들의 ERC20버전이 존재
- Defi에서활용:Smart Contract상에서 대부분ERC20토큰들을 기반으로 설계
- Unwrap:ETH와 동일한 가치를 가지기 떄문에 Unwrap을 스마트컨트렉에 요청하게 되면 동일한 수만큼 ETH로 돌려받을수 있다.

## Defi

- 중앙화 없이 금융서비스를 이용하는 것을 의미

## Uniswap

- pair로 인한 가격관리

```
Dex인 Uniswap에서는 x * y =k인 CPM모델 을사용
x의 공급이 늘어나면 y의 공급이 감소하게 된다.
유동성 공급나는 항상 같은 금액의 pair를 예치
```

- IL(비영구적 손실)

```
유동성 풀에 유동성을 공급한 후 예치한 후 자산이 가격이 변하는 경우 실제로 이자를 받는 보상보다 예치한 금액의 손실이 큰 경우를 의미하여 IL라고 부른다.
```

- Slippage(슬리피지)

```
항상 일정한 금액의 매수를 선택하면 그것을 보장하는 중앙화 거래소 와는달리 Dex에서는 유동성 풀에 충분한 양이 없다면 매수한 금액의 차이가 발생할수 있다.이를 슬리피지라 부른다,
```

## Compound(대출)

- Supply& Borrow

```
담보대출방식으로 supply로 일정 수량의 Token을 공급하고 이에 담보 비율 만큼의 원하는 자산을 대출빌릴수 있다.
담보 자산의 가치가 하락하면 청산이 발생할수 있다.
```

- c토큰

```
유동성 풀의 지분을 나타내는 것으로 ETH를 예치하게 되면 해당 지분만큼의 cETH를 받게된다.
지분만큼 사용자에게 이자 수익을 발생할수 있다.
```

- COMP토큰

```
컴파운드 생태계에서 플랫폼의 방향성에 투표를 할수 있는 권리를 나타내는 거버넌스 토큰
유동성 제공자와 대출인이 50대50으로 나눈다.
```

- APY & APR

```
이자율을 나타내는 값으로 APY는 복리 이자 금액 APR은 단리 이자 금액을 나타낸다.
대부분 Defi서비스는 APY로 표시한다.
```

## Defi Pulse

- TVL:현재 예치된 금액의 총합

## Anchor

- 테라 사태를 키운 Defi서비스
- 사용자가 Terra를 예치하면 항상 20%의 이자를 지급하는 것을 내세운 이자 농사,
- 예치금이 늘어나면서유동성 공급자에게 제공할 이자금액이 바닥나게 되면서 테라폭락이 발생

## NFT

- NonFungible Token
- 대채불가능한 토큰이라는 의미로 유일한 토큰을 의미

## NFT발행방식

- Off-Chain방식은 NFT로 저장될 데이터를 외부 저장소에 저장한뒤 그 주소 값을 NFT발급시 데이터로 저장하는 방식:블록사이즈 리밋
- On-Chain방식은 NFT에 저장될 데이터 전체를 이더리움 데이터 상에 저장하여 수정이 불가능하도록 만드는 형태

## DAO

- 탈중앙앙화 운영조직
- 일반적으로 조직의 결정사항에 대해 투표로 진행되며 수평적임에 비해 결정속도가 느린단점이 존재

## DAO종류

- prorocol DAO

```
서비스의 정책 가치등을 결정하는 조직
거버넌스 토큰을 보유
네트워크 가치를 보장하기 위해 악의적인 행위를 하지 않을거라는 믿음으로 진행
```

- Investing DAO

```
구매하기 어려운 고가치 NFT나 상품을 구매하기 위해 모인 조직
쪼개기 투자
```

- Social DAO

```
일반적인 웹 커뮤니티를 운영하는 조직
가입시 긴청서를 작성하고 커뮤니티 토큰을 구매해야 조직에 참여가능
커뮤니티 가입시에 커뮤니티 이벤트 참여등이 가능
```

## MakerDAO

- DAI스테이블코인의 가치를 1달로 유지하기 위해 모든 조직원들이 리스크 관리와 담보물 선정들을 결정하게 된다.
- Vote Delegate:모든 DAO참여자들이 투표율 향상을 위해서 투표권을 위임할수 있는 기능
- MIP:서비스의 발전을 위해서 사용자들이 직접 문제를 제기하고 이에 대한 해결책을 제시

## Oracle

- 블록체인상에는 이미지 파일이 저장되지는 않고 url이 저장되기 때문에 위변조 대도 알수가 없가.

## Bridage

- 블록체인 네트워크는 서로 분리되어 있으며 네트워크 간의 자산의 이동은 불가능하다.이는 다른 네트워크 자산의 위변조 여부 판단이 불가능한 Oracle문재와도 연관이 있다.
- 이를 해결하기 위해 나온것이 Bridage를 통한 블록체인 간의 자산이동이다.

## Bridage동작방식

- Lockad Mint:이동전 네트워크에서 자산을 스마트컨트렉트상에 보관하고 해당하는 금액만큼의 자산을 이동하려는 네트워크 상에 신규 발행

- 유동석방식:유동성풀에 이동할 자산을 입금하고 이를 확인한 중개자가 이동할 네트워크 유동성 풀에서 사용자에게 해당토큰을 전송

## Atomic Swap

- 별개의 네트워크 자산을 서로 교환
- Hash Lock기능을 사용

## WBTC

- 비트코인을 ERC20토큰으로 변환하여 Defi의 담보자산으로 활용
- WBTC로 변환하게 되면 이더리움 네트워크상에서 BTC가치를 그대로 이용하면서 이를 담보로 농사등 레버리지 투자가 가능하다.

## IBC

- 블록체인간의 통신을 지원하는 방식
- COSMOS블록체인상에서 동작하는 프로토콜
- Lock And Mint방식이 스마트컨트렉트 상이 아닌 IBC상에서 이루어져 네트워크간 자산이동이 가능

- 이와같은 자산 이동은 COSMOS네트워크의 PBFT알고리즘에 의해 보장된다.

## Ethereum2.0

- pos로전환
