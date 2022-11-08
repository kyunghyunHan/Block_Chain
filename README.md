# BlockChain

- 블록체인 만들기

## RSA

RSA

1. PKI 공개키
2. 큰수의 소인수분해 ->양자컴퓨터 ->대부분의 암호화시스템 무용지물
3. 비대칭키:누구든지 검증가능 ->블록체인
4. RSA-2048 대부분의 인터넷 뱅킹

   RSA암화방식 :복호화 방식
   hash방식:비밀번호 찾기 하면 무조건 바까야함

## 큰수의 소인수분해

큰수의 소인수분해

이산대수

## 비트코인 암호화

1. 익명성 ->ECDSA
2. 부인방지 :개인키로 서명하기 떄문에 ->ECDSA
3. 위변조방지 :거래 위변조를방지

## Ecc

공개키 암호기술 구현방식중 하나
RSA에 비해 더 작은 데이터로 RSA와 비슷한 보안성능
실제 디지털 서명방식으로 구현된 알고리즘을 ECDSA
비트코인에서는 secp256k1이라는 타원곡선사용

## 비트코인 address

key Conversion -> publickey ,Ucompressed Public key생성

## Hash

mod 함수
y= x(mod n)
n= 7일떄
1= 1(mod7)
3= 10(mod 7)
6=20(mod 7)
2=30(mod 7)
5=40(mod 7)
1=50(mod 7)
일정한 임이의 256비트
단방향 알고리즘

## newwork

client server
client -> Gateway -> was -> server -> database

p2p Network

- 어느서버에다가도 하던지 동일한 데이터를 받을수 있다
- 토렌트

## TCP

서버와 client간에 데이터를 신뢰성 있게 전달하기 위해 만들어진 프로토콜

데이터 전송을 위한 연결을 만드는 연결지향 프로토콜

데이터 전송 과정에서 손실이나 순서가 바뀌는 경우 교정 및 순사 재조합 지원

IPC소켓통신 방법으로 보통 지원

## http

get
post
put

delete
head 서버 헤드 정보 획득 요청
options 서버 옵션 확인 요창

## web socket

양방향
한별연결이 수립되면 클라이언트와 서버 자유롭게 데이터 전송가능
실시간 시세 데이터 ,채팅 솔루션 등에 사용

## RPC

원격 서버의 함수를 함출 호출할수 있는 통신기술
IDL을 사용해서 호출 규약을 정의하고 이를 통해 stub코드를 생성
program 에서는 stud을 call함으로써 개발자는 네트워크의 대한 지식 업이 원격 함수 호출이 가능하다.

## gRPC

구글에서 개발한 RPC통신
양방향 스트링 데이터 처리 MSA구조의 서비스에서 활용

## protobuf

grpc의 IDL protobuffer의 줄임말 프로그램 상에서 이를 사용하기 위해 .proto stub이 생성되어야 한다.json,xml통신보다 데이터 전송 크기가 작고 성능이 빠르다.
proto3를 사용

## 블록 검증

신규블록 수신
블록구조 일치여부
재계산 block header hash== block header hash
block timestamp <now()+2hours
block size <1mb
coinbace transaction check
transaction check
mempool update ->데이터베이스 업데이트
levelDB insert New Block
Block 전파

## 트랙잭션 전피

트랜잭션을 다른 노드에게서 전파받는다
이미 받은 트랜잭션인지 확인
)없는 경우 다른 노드에게 전파
상대노드가 없는 경우 getdata를 요청받는다
새로운 트랜잭션에 전달
연결딘 모든 노드에게 전달될떄까지 수행한다

## 트랜잭션 검증

신규 투랜잭션 수신
트랜잭션 구조 일치 여부
in,out list 존재여부

트랜잭션 사이즈 <1mb
output value <2100만 btc
mempool존재 여부
block 존재여부
input check(double spending)
input check(orphan tx)
input check(coin base) 보상
input check(Not UTXO)
input >output value
check input script
add mempool
트랜잭션 전파

## 블록전파

- 마이닝에 성공한 블록은 아래 방법 없이 블록 전체 데이터를 모든 노드에게 전달
- network에 블록체인 다운 받기 위해 언결된 다른 노드들에 ping전송
- 전달받은 block header전달
- 아직 전달 받지 못한 block인 경우 header와 getdata를 모두 요처
  새로운 블록전달

## 블록 구조

Block size 4bytes <1mb
BlockHeader 80bytes
Transaction Counter 1~9 bytes
Transaction variable

## 블록헤더 구조

version 버전정보
previous block hash 이전 블록의 헤더 해시
merkle root 트랜잭션들의 hashroot
timestamp 블록 생성시간
Difficulty Target pow의 어려움 정도
nonce

## 블록생성

mempool tx선택
coinbase tx 생성
merkle root 연산
block header구성
find nonce
block전파

## UTXO

아직 사용되지 않는 Output을 지칭
UTXO 사용 여부를 통해서 자산의 안정성을 확인
input 사용자가 내는금액
output 받는금액

## 트랜잭션 내부구조

version 현재값1
Flag Witnesses Tx여부에 따라 달라짐
Number of inputs input의 개수
inputs input정보
Number of Outputs ouput의 개수
outputs output정보
Witnesses Witnesses서명데이터
Locktime 트랜잭션 시간 제한

## input구조

Transcation 해시 output이 포함된 txid
output index Tx안에서 seq
Unlocking-script size Unlocking-script크기
Unlocking-script output을 input으로 바꾸는 서명정보
sequence Number 기본값 oxffffff

## output구조

Amount 송금할금액 사토시 단위
Locking-script size
lockking-script 송금자의 정보가 담긴 데이터

## Transaction Fee

input 총힙에서 전체 output의 총합을 뺸 값
블록에서 설명했듯니 채굴자들이 거래를 더 빠르게 하기 위해서 수수료를 높여야 한다

## coinbase

pow에서 채굴에 성공하게 되면 채굴에 성공한 채굴자에게 기본 보상 수수료와 거래 수수료를 보상으로 제공

## 거래방식

## p2pk

우웃풋
이전 아웃풋에 포함된 공개키
op_checksig

input
서명

## p2pkh

퍼블릿키 해시값

## NULL_DATA

블록체인상에 데이터를 저장하는 방식
input scriptsig가 들어가지 않는 방식
OP_RETURN을 사용

## BITCOINT 새로운거래형식

SEGWIT
p2pkh랑 비슷

## TapRoot

2021년부터 업그레이드로 인해 새로운 거래형식
슈노르 서명방식 지원

- 공동 공개키를 셍성하여 하나의 서명으로 공동서명
  MAST지원
  비트코인 스크립트 실행사실을 숨길수 있음
  비트코인 프라이버시를 향상시키고 트랜잭션의 수수료를 감소

## Lightning Network

비트코인 레이어 2기술로 블록체인 상에서 일정 금액을 생성하고
이를 네트워크 상에 배포시키지 않고 잠금된 금액 기반으로 실시간 거래가 가능하도록 하는 기술

엘살바도르 국민들은 이 기술을 비트콘인 법정 화폐

## 비잔틴 장군문제

특정수 이상의 장군이 동시에 공격을 해야 성을 공략할수 있다.
서로 p2p만 연락을 주고 받을떄 첩자의 방해가 있더라도 이 공격을 성공시키는 방법은?

proof of work

## BFT

분산화된 네트워크에서 일부장애가
발생하더라도
네트워크가 정상적으로 동작하도록하는 알고리즘
PBFT가 블록체인생태계에서 사용
Cosmos,하이페브릭

## proof of work

컴퓨팅 파워로 doble spending과 같은 거래 위변조 공격을 막는방법

새로운 블록을 생성하는것이고 그 블록내에 field로 포함되는 nonce값을 찾는것
전체 Network hash에 따라 Difficulty가 변화하고 10분마다 block이 생성되게 조정

## 채굴과정

새로운 블록이 생성됨을 알림받는다
다음 블록 생성을 위해서 임시pending중인 트랜잭셕을 포함한다
Coinbase거래를 임시 블록에 포함한다
이번 블록 a와 트렌잭션들을 포함한 임시 블록 b를 만든다

## Network Hash Rate와 Difficulty

Miner참여자 수가 증가하고 성능이 좋은 채굴 장비를 이용하게 되면 채굴의 속도가 점점 빨라진다.
Difficulty따라 Bit가 조절되고 정답이 되는 Header Hash 의 0의 개수가 늘어난다

## Find Nonce

bits ->hex값으로변경

0x29D72D _22 \*\*(8_(0x17 -3))
= 0x00000
header hash가 targe

## 채굴보상

전체 2100만개

## 비트코인 공격방식

51%어택

동일한 utxo로 두개의 거래를 생성하고 fork를 통해서 공격자가 원하는 거래만 블록에 포함되게 하는 공격
공격자가 더 긴 블록체인을 만들기 위해서는 전체 네트워크 hashRate의 51프로를 가져야 성공 가능성이 높음

## Sybul attack/ dos attic

- 공격자가 수많은 노드를 운영하면서 비트코인 네트워크 block전파를 방해하거나 잘못된 block data를 인접 노드들에게 전송하는 공격
- 특정 노드들에게 비정상적인 거래를 무한정 생성되어 네트워크 전체의 마비를 이르키는 공격

- 비정상적인 거래 블록은 전파하지 않음
- 이중 지불 공격은 전파하지 않음
- 같은 노드에서 전송된 동일블록과 거레는 전파하지 않음
- 아주 작은 단위의 거래를 전송

## Longest chain rule

블록체인 네트워크 전체가 fork가 발생할 떄 하나의 블록체인만을 유지하기 위한 방법

## Asic

특정용도에 맞게 맞춤 제작된 집적 회로를 의미

## Mining Pool

고성능 장비를 구매하기 위한 일반 사용자들이 모여서 채굴에 참여하기 위해 등장

share - 지분투입정보
pay-per-share - 보상에 지분에 따라 지급하는 방식
solo miningpool - 찾은사람이 다갖는
채굴시 일정 지분 등록하고 연산한만큼

## Level db

kb database
관계형 검색이 불가능
하나의 프로세스만이 특정 데이터 베이스 접근가능
읽기 쓰기 성능이 빠르다

입력
조회
삭제

'b'+32-byte block hash /Block index기록
'f'+4-byte file number /파일 정보기록
'i'+4-byte file number
'R'+1-byte boolean /Reindexing여부
'F'+1-byte flag nama length+flag name string /Txindex On/Off여부
't'+32-byte transaction hash /Transaction index기록

'c' +32 byte transaction hash /트랜잭션 내 UTXO 데이터 조회용
'B' -> 32 -byte block hash /가장 최신 Block이 있는지확인용
key값으로만 검색이 가능

## Mempool

아직 블록에 포함되지 않는 pending Transaction들을 저장 및 관리하는 방법
채굴자들은 Mempool중에서 Transaction을 선택해서 신규 Block에 포함시킨다
Mempool에 들어가고도 14일동안 처리되지 않고 남아 있는 Transaction은 Expired된다

## Fork

동시에 블록정답을 찾기에 성공하게 된경우를 분기되었다 또는 Fork라 부른다
Longest Blockchain Rule을 통해 Fork된 네트워크 를 하나로 유지시키고 있다.

- 비트코인 블록은 어떤 블록이 전달될지 모르기 때문에 Fork발생시 2개의 체인을 가지고 있으며 이중 LongestChain을 MainChain으로 유지

- MainBranch가 Longest가 아님을 알게된 순간 SideBranch를 Main으로 변경하고 이데 대한 LevelDB업데이트가 이루어진다.
- soft Fork는 모든 사용자가 Node Upgrade 를 하지 않아도 진행
- Hard Fork는 모든 사용자가 Node Upgrade를 해야하고 하지 않는 경우 네트워크에서 분리

## Soft Fork - Segwit

- segwit은 비트코인 블록 사이즈로 인한 처리 성능을 제한을 해소하고 Transaction Malleability 문제를 해결하는 방안으로 제시

- Soft Fork이기 떄문에 기존 Node가 업그레이드 하지 않아도 segwit Transaction처리 가능
- 대표적인 ASIC채굴자들이 반대하면서 Network Fork분리가 발생됨

## Hard Fork

- 네트워크 분리가 발생한다.특정 기능을 제외한 대부분은 동일
- 탈중앙화된 블록체인 특성 상 업그레이드에 찬성하는 쪽과 반대한 쪽이 나뉘기된다.
- Hard Fork후에는 기존에 연결된 Node상에서 서로 인정하는 Block이 달라지게 되고 자연스럽게 각 네트워크에 참가하는 노드에 따라 네트워크 분리가 발생

- 새로운 Segwit업그레이드가 ASIC에서는 사용이 불가능하기 떄문에
- 채굴자측에서는 블록사이즈 문제는 블록 크기 증가로 가능하다고 하엿다.
- 개발자 측에서는 Segwit적용이 블록체인 확장성 문제를 해결가능하다고함

## Node(peer)

- Explorer
- Exchange

## Explorer

- FULL
- 검색 사이트

## Exchange

- 출금
- 거래소 중앙서버
- 거래서 Hotwallet
- wallet이 tx생성
- Fullnode에게 전달

## Wallet

- 키를 안전하게 관리하고 이를 통해서 사용자가 쉽게 거래를 생성하는 것을 지원

1. 거래조회
2. 사용자 잔액조회
3. 신규 블록 생성알림
4. 주소록관리
5. 사용자 키 관리

- web Wallet,App Wallet, Paper Wallet,Hardware Wallet

#### !) Hardware Wallet

- 가상 자산 고액 자산가들이 대부분 사용

##### !)Samsung Wallet

- 기본탑재 월렛 앱
- 보안 안정공간(Trust Zone)인 Tee에서 사용자의 개인키를 보관관리
- 개인키를 안전하기 보관 Export기능도 제공
- TEE환경에서 동작하기 위해서 Wallet에서 보안 관련 설정이 추가로 진행

#### COLD 와 HOT

- COLD와 HOT은 인터넷환경과 연결된지 안됫는디 에따라 구분
- Exchange는 사용자의 입출금이 활발하고 해킹의 위험이 항상 존재하기 떄문에 Cold와 Hot으로 구분

## 키를방법하는 방법

### 1)Nondeterministic Wallet(Random)

- 100개의 랜텀키 생성 이를 한번씪만 사용
- 주소를 한번만 사용하다보니 privacy보장이 높아짐
- 주기적인 백업이 필요

### 2)Hierarchical Deterministic Wallet

- 하나의 seed값에서 생성된 Master Key를 중심으로 개인키 생성
- 개인키 하나로 여러개의 주소를 관리 가능
- 여러 Branch키를 생성 Branch마다 용도에 맞는 주소 그룹 분류 가능

### 3) Mnemonic

- 새로운 Seed관리방안
- 개인키 분실시 복구가 불가능한것에 대한 방안
- 개인키를 분실해도 Mnemonic을 통해 개인키 재 생성 가능

## SPV

- Bitcoin 사이즈가 커짐에 따라 이를 저장하기 힘든 Ligth-Weigt노드 IoT기기,스마트폰등에 node설치를 위해 나온 node 방안
- 풀 노드로부터 MerkleTree와 Block Header만을 받아 전송바당 트랜잭션 검증

## Merkle Pass

- 필요한 것만 받아서 검증

## Bloom Filter

- SPV노드가 관심있는 트랜잭션ㅇ르 전달할떄 내가 관심있는 트랜잭션을 숨기고 정보를 전달받는 방법을 위한 기법
- False Positive - 관심이 없는것은 보장할수 없다.

- Ligth노드가 Fullnode에게 전달하여 Ligth가 관심있는 거래정보를 실제로 들어내지않고 전달가능

## Bitcoind (Bitcoin Core)

- 사카시나카모토가 개발한 Bitcoin공식 클라이언트 소프트웨어
- 비트코인 풀 노드를 싱크하는데 걸리는 시간은 1주일

## bitcoin이 한계점

- 느린처리속도

- - 1)높은보안성 :블록생성주기 10분 (double spend 방지)
- - 2)Fork 가능성 낮음 : 시간을 10분으로 해서
- - 3)Network Bandwidth 낮음:통신으로 인한 노드의 부하가 낮아질수 있따.
- 높은 에너지 사용
- 제한적인 기능
- 제한적인 익명성

## 대안 블록체인

1. Ripple

- 은행들간의 Swift망을 대체하기위해 제시된 블록체인 프로토콜

2. Fabiic

- 기업용블록체인
- 삼성SDS사용

3. Solana

- 빠른성능 ,저렴한 수수료

## 제한적인 기능

- 저장 송금이외의 기능을 제공하고있지안음
- 디지털골드
- 이더리움:스마트컨트렉트

## 제한적인 익명성

- PKI를 이용한 익명성:신원인증 없이 PKI를 이용해서만 거래
- Key재사용 제한:한번만 사용을 제한
- Mixer:CoinJoin과 같은 Mixer기능을 통해 다른 사용자와의 거래에 나의 거래를 숨길수 있는 기능

## Multi-Signature

- 하나의 거래에 여러명이 서명을 해야 승인될수있는 거래
- 큰 금액을 보관하는 경우 사용자의 키분실이나 해킹위협에 대비하기 위해 사용

- 거래소는 거래소의 모든 월렛의 개인키를 혼자 소유하고 있다가 대표가 사망하자 모든 자산을 찾을수 없게 되었다.

## Custody서바스

- 가상자산의 보관 관리를 대행해주는 서비스
- Multi-Signature기반으로 가상자산의 보관및 입출금지원

## Multi-Party Computation

- 키자체를 쪼개서 거래를 승인하는 새로운 기술

## ZKP

- 서명과 공개키를 공개하지 않고 나의 거래를 증명
- 확률
- A와 B중에 가게한뒤 A라고 햇을떄 A로 나오면 인정
- 수백만번

## Travel Rule

- 입금된 거래는 자금세탁의 위험이 있기 떄문
- 거래소에서 사용자의 입출금을 실명으로 관리하는것이다.
- 금융 실명제와 동일한 효과
- 사용자는특정 금액 이상의 입출금발생시 상대방의 신원정보를 함꼐 등록해야 출금이 가능

## 명령어

```
RUST_LOG=info cargo run --example gen_bc --quiet
```

```

RUST_LOG=info cargo run --example gen_bc --quiet
```

```

RUST_LOG=info cargo run --quiet  server data
```

```

{"Blocks":""}
```

```

{"CreateWallet":"justin"}
```

```

{"Genesis":"1KooomKwhgPCfB2YfnKT7yMUxGcVWqS3ns"}
```

```

{"Blocks":""}
```

```

RUST_LOG=info cargo run --quiet server data1
```

```
{"Sync":""}
```

```

{"Blocks":""}
```

```

{"CreateWallet":"Bob"}
```

```
{"Trans": {"from":"1KooomKwhgPCfB2YfnKT7yMUxGcVWqS3ns","to":"1EuM1UEhJFTDR5UfWzfghzv82bCdwRWk9E","amount":"4"}}
```

```

{"Blocks":""}
```
