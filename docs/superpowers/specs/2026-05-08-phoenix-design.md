# Phoenix — Forensic Wallet Recovery Platform

**Sürüm:** 1.0
**Tarih:** 2026-05-08
**Sahip:** Phoenix Maintainer
**Kod adı:** Phoenix (public adı pre-launch'ta workshop ile belirlenecek; "AI wallet recovery" branding'i kullanılmayacak)

---

## 1. Yönetici özeti

Phoenix, **kayıp crypto cüzdanlarının partial-info segmenti** için açık-kaynak, lokal çalışan, AI-destekli forensic recovery platformudur. Bugün artisan recovery dükkanlarının (WRS, KeychainX, ReWallet) manuel yaptığı işi sistemleştirir: yapılandırılmış cognitive interview + deep digital exhaust forensics + Bayesian aday sıralaması + distributed cracking, tek bir tüketici ürünü olarak.

**Mantıksal sınırlar açıkça çizilmiştir:**
- Tam-bilgi-kayıp seed'ler **kurtarılmaz** (128-bit entropy, fizik limiti)
- Hardware-glitch / firmware-fault attack **kapsam dışı** (Praefortis/Unciphered alanı)
- "AI wallet recovery" buzzword'ü **kullanılmaz** (scam-genre association)
- Hiçbir tohum/anahtar **kullanıcı makinesinden çıkmaz**

**Hedef başarı oranı (target segment içinde):** %35-50 v1, %50-70 federated learning ile 12 ay sonra.

**Çekirdek devrim iddiası:** Recovery industry'sinin ilk endüstrileşmesi. Bugün artisan = yarın ürün.

---

## 2. Problem ve pazar

### 2.1 Sayılarla pazar
- Toplam kayıp BTC: ~3.7M coin (Chainalysis 2024) ≈ **$200B+**
- Pump.fun: 1.4M aktif cüzdan, sürekli yeni "kayıp dust position" akışı
- Reddit r/Bitcoin / r/Ethereum / r/CryptoCurrency "lost wallet" thread'leri: günde 25-50 yeni post
- Recovery market hacmi (artisan toplam): ~$50-100M yıllık tahmini, %20+ büyüme

### 2.2 Mevcut servisler ve eksiklikleri

| Servis | Model | Fee | Eksiklik |
|---|---|---|---|
| Wallet Recovery Services (Dave Bitcoin, 2013) | Manuel danışmanlık | %20 | Saatlerce telefon, scale yok, AI yok |
| Crypto Asset Recovery (Chris Brooks) | Manuel + YouTube viral | %20 | Aile şirketi, kapasite kısıtlı |
| KeychainX (CH) | Manuel + glitch | %10-20 | Kurumsal odaklı, retail UX yok |
| ReWallet (DE/CH) | Air-gapped HPC | %20 | Almanca-öncelikli, dil sınırlı |
| Praefortis / Unciphered (US) | Hardware glitch | Vakaya özel | Pahalı, sadece donanım |
| BTCRecover (open-source CLI) | DIY | Bedava | %95 kullanıcının kapasitesi dışında |
| seedcat / WalletGen | DIY GPU brute | Bedava | CLI, Python gerekir |

**Eksik:** Artisan ile CLI arasında **ürünleştirilmiş orta** yok.

### 2.3 Niye 2026'da yapılabilir, eskiden yapılamazdı

1. **LLM-tabanlı cognitive interview:** Opus 4.7 / Llama 4 seviyesi LLM'ler artık yapılandırılmış memory recovery interview'ü yapabiliyor — 2 yıl önce kapasite yoktu
2. **Local LLM ekonomisi:** 32GB VRAM'de Llama 3.3 70B çalışıyor → kullanıcı verisi cloud'a çıkmadan yorumlanır
3. **GPU commodity:** RTX 4090 saniyede 10⁹ BIP-39 deneme; kiralık cloud GPU saatlik $1.50
4. **Tauri/Rust:** Desktop app güvenli ve hızlı paketleniyor
5. **MPC olgunluğu:** Atomic success-fee threshold ECDSA ile production-ready

---

## 3. Açık olarak yapmadıklarımız (non-goals)

- Tam-bilgi-kayıp seed kurtarımı (matematiksel imkansız — 128-bit entropy)
- Hardware-glitch / firmware-fault attacks (Praefortis/Unciphered alanı)
- Çalıntı/phishing fonu geri alma (AssetReality, Aegis alanı)
- Pre-protection (multi-sig, social recovery wallet) — Argent/Safe/Daimo alanı
- Quantum decryption (Q-Day gelirse zaten protokol post-quantum'a geçer)
- Custodian dormant account recovery (Coinbase/Binance kendi süreci)
- "Guaranteed recovery" ya da "%90 success rate" iddiaları (scam dilinde olur)

---

## 4. Hedef kullanıcı segmenti

### 4.1 Çözülebilirlik haritası

| Vaka | Yaygınlık | Çözülebilirlik | Phoenix v1? |
|---|---|---|---|
| **A.** 11/12 kelime hatırlanıyor | Yüksek | 🟢 dakikalar | ✅ |
| **B.** Tüm kelimeler ama sıra/yazım yanlış | Çok yüksek | 🟢 saatler | ✅ |
| **C.** Şifreli wallet.dat + parola pattern'i hatırlanıyor | Yüksek | 🟡 günler | ✅ |
| **D.** Eski cihaz/yedek var ama erişim yok | Orta | 🟡 forensic + cracking | ✅ |
| **E.** Seed fotoğraflanmış, fotoğraf silinmiş | Orta | 🟡 thumbnail recovery + OCR | ✅ |
| **F.** Hardware wallet PIN forgotten (Trezor) | Düşük-yüksek değer | 🟡 glitch attack | ❌ (kapsam dışı) |
| **G.** 12 kelime tamamen kayıp, sıfır iz | Orta | 🔴 imkansız | ❌ |

**Phoenix v1 segment:** A + B + C + D + E. Tahmini hacim: kayıp cüzdanların **%50-60'ı**.

### 4.2 Persona

**Birincil — "Frustrated Self-Custodian":**
- 30-50 yaş, teknik okuryazarlık var ama CLI seviyesinde değil
- 2017-2021 arası BTC/ETH almış, cüzdan oluşturmuş
- Şimdi $5k-$500k arası fonu unutulmuş cüzdanda
- BTCRecover deneyip vazgeçmiş veya WRS fiyatından çekinmiş
- Reddit/Twitter'da "lost my crypto" thread'i atmış

**İkincil — "Heir / Inheritor":**
- Yakını ölmüş, şifre/seed bırakmadan
- Bilgisayar, telefon, kâğıtlar miras kalmış
- Hukuki süreç tamamlanmış, ownership doğrulanabilir
- Phoenix bu cihazlardan iz çıkarması için ideal

---

## 5. Sistem mimarisi — 6 katman

```
┌──────────────────────────────────────────────────────────┐
│  Tauri Desktop Shell (Rust + React UI)                   │
└────────────┬─────────────────────────────────────────────┘
             │
┌────────────▼─────────────────────────────────────────────┐
│  Layer 1: Cognitive Excavation Engine                    │
│  - Local LLM (Llama 3.3 70B via Ollama)                  │
│  - Cognitive Interview Protocol (Fisher & Geiselman)     │
│  - RL interview policy (Bayesian Optimal Experimental)   │
│  - Multi-agent debate cross-questioning                  │
└────────────┬─────────────────────────────────────────────┘
             │
┌────────────▼─────────────────────────────────────────────┐
│  Layer 2: Digital Forensic Excavator                     │
│  - File carving (Foremost/Scalpel algorithms)            │
│  - OCR + VLM (Tesseract + Llama Vision)                  │
│  - Browser forensics (Chrome/Firefox)                    │
│  - Password manager dump parsing                         │
│  - Email backup mining                                    │
│  - Photo EXIF + perceptual hashing                       │
│  - iCloud/Drive backup parsing (with permission)         │
└────────────┬─────────────────────────────────────────────┘
             │
┌────────────▼─────────────────────────────────────────────┐
│  Layer 3: Constraint Propagation & Inference             │
│  - CSP (arc consistency, AC-3, backjumping)              │
│  - Bayesian Networks (Pearl 1988)                        │
│  - Hidden Markov Models for pattern modeling             │
│  - MCMC for low-prob candidate sampling                  │
│  - Damerau-Levenshtein for typo prediction               │
│  - PassGPT-style transformer (per-user fine-tune)        │
└────────────┬─────────────────────────────────────────────┘
             │
┌────────────▼─────────────────────────────────────────────┐
│  Layer 4: Distributed Cracking Engine                    │
│  - hashcat (modes 11300/12700/15700/18800)               │
│  - seedcat (BIP-39 GPU brute)                            │
│  - Custom CUDA kernels (Phantom/Solflare/Backpack)       │
│  - Bloom filter cache (skip tested candidates)           │
│  - Multi-tier dispatcher (local → optional cloud)        │
│  - Asenkron iş kuyruğu, resume-from-checkpoint           │
└────────────┬─────────────────────────────────────────────┘
             │
┌────────────▼─────────────────────────────────────────────┐
│  Layer 5: Verification & Wallet Restoration              │
│  - MPC threshold ECDSA (Gennaro-Goldfeder 2018)          │
│  - TEE attestation (Intel SGX / Apple Secure Enclave)    │
│  - Smart contract escrow (atomic success-fee)            │
│  - Onchain proof-of-recovery log                         │
└────────────┬─────────────────────────────────────────────┘
             │
┌────────────▼─────────────────────────────────────────────┐
│  Layer 6: Federated Learning Loop (moat)                 │
│  - Differential Privacy (Dwork 2006)                     │
│  - Federated Averaging (McMahan 2017)                    │
│  - Secure Aggregation                                    │
│  - Anonymous case telemetry → model improves weekly      │
└──────────────────────────────────────────────────────────┘
```

### 5.1 Layer detayları

#### Layer 1: Cognitive Excavation Engine
**Amaç:** Kullanıcının zihninden mümkün olan en zengin ipucu setini çıkarmak.

**Akış:**
1. Onboarding: cüzdan tipi, oluşturma dönemi, cihaz
2. Yapılandırılmış görüşme (30-90 dk):
   - Free recall (Fisher-Geiselman)
   - Context reinstatement (cüzdanı oluşturduğunda nerede, hangi ruh halinde)
   - Reverse order recall
   - Change perspective recall
3. RL policy en yüksek bilgi-getirisi sorularını seçer (Bayesian Optimal Experimental Design — Lindley 1956)
4. Multi-agent debate: 3 LLM ajan kullanıcının cevaplarını cross-question eder, çelişki tespit eder

**Çıktı:** Hatırlanan-pattern dizisi + her pattern için güven skoru

#### Layer 2: Digital Forensic Excavator
**Amaç:** Kullanıcının lokal makinesinden ve onayladığı yedeklerden seed/parola adaylarını çıkarmak.

**Modüller:**
- **File carving:** Foremost/Scalpel byte-pattern matching ile silinmiş dosyalardan seed/wallet recovery
- **OCR:** Tesseract + Llama Vision ile el yazılı seed fotoğraflarını tarama
- **Browser:** Chrome/Firefox cache + history + autofill (BrowsingHistoryView, Hindsight)
- **Password managers:** KeePass/LastPass/1Password export parsing
- **Email:** Gmail Takeout / iCloud Mail / IMAP backup'larında seed-pattern arama (embedding search)
- **Photos:** EXIF + perceptual hash ile silinmiş seed fotoğraflarının thumbnail'larını bulma
- **Backups:** iCloud / Google Drive / Dropbox lokal indir + parse (kullanıcı onayıyla)

**Çıktı:** Aday seed parçaları, parola fragmentleri, tarihsel kullanım pattern'leri

#### Layer 3: Constraint Propagation & Inference
**Amaç:** Layer 1 + Layer 2 çıktılarını birleştirip sıralı aday listesi üretmek.

**Algoritmalar:**
- CSP (AIMA Bölüm 6) — arc consistency, AC-3, conflict-directed backjumping
- Bayesian Network (Pearl 1988) — kanıt parçaları → aday olasılıkları
- HMM — kullanıcının pattern üretim alışkanlığını model'leme
- MCMC — düşük olasılıklı ama mümkün adayları sample'lama
- Damerau-Levenshtein — yazım hatası tahmini
- PassGPT-style transformer (Rando 2023) — kullanıcının kişisel pattern'inde fine-tune

**Çıktı:** Priority queue olarak sıralı aday listesi

#### Layer 4: Distributed Cracking Engine
**Amaç:** Adayları gerçekten test etmek.

**Bileşenler:**
- hashcat wrapper (modes 11300/12700/15700/18800)
- seedcat (BIP-39 GPU brute)
- Custom CUDA kernels: Phantom, Solflare, Backpack lokal storage formatları
- Bloom filter cache: önceden test edilmiş adayları skip
- Multi-tier dispatcher:
  - Yüksek-olasılık adaylar → kullanıcının lokal GPU'sunda
  - Düşük-olasılık adaylar → kullanıcının onayıyla kiralık cloud GPU
- Async iş kuyruğu, fail-over, resume-from-checkpoint

**Çıktı:** Çalışan seed/parola

#### Layer 5: Verification & Wallet Restoration
**Amaç:** Cüzdan açıldığında güvenli teslim ve atomic fee.

**Mekanizmalar:**
- MPC threshold ECDSA (Gennaro-Goldfeder 2018) — split-key reveal, Phoenix anahtarı asla görmez
- TEE attestation — cracking node attest eder ki output sızdırılmadı
- Smart contract escrow:
  - Free / Pro tier: hizmet bedeli sub'tan, recovery bedava
  - Success-fee tier: %5, atomik (escrow + reveal aynı transaction'da)
- Onchain proof-of-recovery log → marketing case study'leri için (anonim)

#### Layer 6: Federated Learning Loop (moat)
**Amaç:** Her başarılı recovery'den ders çıkarmak, model'i her hafta zekleştirmek.

**Mekanizmalar:**
- Differential Privacy (Dwork 2006) — kullanıcı verisi sızmadan öğrenme
- Federated Averaging (McMahan et al. 2017)
- Secure Aggregation (Bonawitz et al. 2017)
- Anonim telemetri: hangi cognitive interview pattern'leri başarıya götürdü?

**Niye moat:** Recovery shop'ları veriyi paylaşmaz. Phoenix'in interview policy'si her hafta zekleşir, rakipler sıfırdan başlar. Compounding moat.

---

## 6. Algoritma & literatür kataloğu

| Alan | Referans |
|---|---|
| Cognitive interview | Fisher & Geiselman 1992 |
| Bayesian deneysel tasarım | Lindley 1956, Chaloner 1995 |
| CSP & search | Russell & Norvig (AIMA, 4th ed.) |
| Bayesian networks | Pearl 1988, Koller & Friedman 2009 |
| MCMC | Metropolis 1953, Hastings 1970 |
| Differential privacy | Dwork 2006, Abadi et al. 2016 |
| Federated learning | McMahan et al. 2017 |
| Secure aggregation | Bonawitz et al. 2017 |
| Threshold ECDSA | Gennaro-Goldfeder 2018, Lindell 2017 |
| Side-channel resistance | Kocher 1996 |
| File carving | Garfinkel 2007 |
| LLM password modeling | PassGPT (Rando 2023) |
| RL for interview policy | Sutton & Barto 2018 |
| Multi-agent LLM debate | Du et al. 2023 |
| File-system forensics | Carrier 2005 |

---

## 7. Trust & integrity modeli

### 7.1 Lokal-only çalışma kanıtı
- Tüm core algorithm'lar Tauri sandbox içinde
- Network isolation: cracking sırasında dışarı sadece imzalı update server + opsiyonel cloud GPU'ya istek (kullanıcı onayıyla)
- Wireshark-grade audit log → kullanıcı her byte'ı görebilir
- Open-source: GitHub'da public, deterministic / reproducible builds

### 7.2 Güven mekanizmaları (rollout)
- **Day 1:** Kod public, README + threat model
- **Day 30:** Trail of Bits engagement açıldı
- **Day 60:** Sigstore + Cosign signed binaries
- **Day 90:** Apple notarization + Microsoft signed
- **Day 120:** Independent reviewer program (top 10 crypto YouTubers free copy + audit erişimi)
- **Day 180:** Trail of Bits audit raporu yayınlandı

### 7.3 Çalıntı cüzdan filtresi
- Recovery başlamadan önce target adres → Chainalysis Reactor + TRM Labs API check
- Çalıntı / sanction'lı flag varsa hizmet **vermiyoruz** (clear UX message)
- Ownership attestation: KYC doc + creation-date proof + 2 secondary references (success-fee tier)

### 7.4 Anti-scam-genre marketing
- Reklam dilinde "AI" yok
- Hiçbir success rate >%70 iddiası yok
- "Guaranteed" kelimesi marketing'de geçmez
- Independent reviewer linkleri her sayfada

---

## 8. Legal & compliance

### 8.1 Yargı yetkileri
- Birincil registration: İsviçre veya Estonya (low-friction crypto-recovery legal framework)
- ABD ops: Money Transmitter License gerek değil (custody yok)
- AB: GDPR-uyumlu, sadece lokal data processing

### 8.2 AML/KYC
- Free tier: KYC yok, lokal araç olarak kullanılır
- Pro tier ($99/ay): KYC opsiyonel
- Success-fee tier: KYC zorunlu + ownership attestation
- $5K+ recovered transaction'lar: OFAC sanction list check

### 8.3 Disclaimers
- "Phoenix is a tool, not a service. Outcomes depend on user-provided information."
- "We do not guarantee recovery."
- "By using, you attest you are the rightful owner of the wallet under recovery."

---

## 9. Para modeli

| Tier | Pricing | Hedef |
|---|---|---|
| **Free** | İlk 1M brute attempt + 30 dk LLM interview | Tire-kicker'lar, validation |
| **Pro** | $99/ay unlimited brute + extended interview + cloud GPU offload | Aktif kurtarma deneyenler |
| **Recovery Success Fee** | %5 (smart contract atomic) | Başarılı vakalar |
| **Enterprise API** | $5k-$50k/yıl | Custodian/exchange dormant account programı |
| **Premium Human Consult** | $500/saat | Karmaşık vakalar, opsiyonel |

### 9.1 Revenue projeksiyonu
- **Yıl 1:** 10 başarılı recovery × $50k ortalama = $25k success fee + $50k Pro sub = $75k
- **Yıl 2:** 100 recovery × $30k = $150k success + $300k Pro = $450k
- **Yıl 3:** 500 recovery × $25k = $625k success + $1M Pro + ilk Enterprise = $2M+
- **Yıl 4-5:** $5-15M ARR ufku

---

## 10. Naming & positioning

### 10.1 Naming
- Kod adı (internal): **Phoenix**
- Public ad pre-launch'ta belirlenecek; kriterler:
  - "AI" yok
  - Forensic / archaeology imgesi
  - .com/.io domain available
  - Trademark clean
  - Crypto'da kullanılmamış

**Aday isimler (workshop'ta daraltılacak):**
- Lazarus, Excavate, Vault Hunter, Echo, Crypt, Phoenix, Reclaim, Foundling, Resurgo

### 10.2 Positioning
- ❌ "AI wallet recovery" (scam genre)
- ✅ "Open-source forensic recovery assistant for civilians"
- ✅ "BTCRecover with a brain"
- ✅ "The world's first systematized recovery platform"

### 10.3 Marketing kanalları
- Reddit lost-wallet thread'lerine **hizmet** (ücretsiz değerli yorum, hard-sell yok)
- YouTube case study'leri (Joe Grand modelinin sistematik versiyonu)
- Crypto Twitter (BTCRecover, Bankr, Polymarket post-mortem'leri)
- SEO: "I lost my wallet" long-tail
- Open-source community (HN, ProductHunt launch)
- Türkiye + DACH bölgesi cold-outreach (mevcut outreach altyapı)

---

## 11. Rekabet farklılaşma matrisi

| Capability | WRS | KeychainX | BTCRecover | Phoenix |
|---|---|---|---|---|
| Productized | ❌ | ❌ | Partial | ✅ |
| AI cognitive interview | ❌ | ❌ | ❌ | ✅ |
| Digital exhaust parser | Partial (manual) | Partial | ❌ | ✅ |
| Local-only | ❌ (kullanıcı dosya gönderir) | ❌ | ✅ | ✅ |
| Open-source | ❌ | ❌ | ✅ | ✅ |
| Federated learning | ❌ | ❌ | ❌ | ✅ |
| Atomic success fee | ❌ | ❌ | N/A | ✅ |
| GUI | ❌ | ❌ | ⚠️ (kötü) | ✅ |
| Türkçe / multilingual | ❌ | ❌ | ❌ | ✅ (planda) |

---

## 12. 8 hafta MVP scope

### Hafta 1 — Foundation
- Tauri + React + Rust workspace setup
- Llama 3.3 70B integration via Ollama
- Logging + telemetry framework (opt-in only)
- CI/CD baseline (GitHub Actions, signed builds)

### Hafta 2 — Cognitive Interview MVP
- Initial 50 yapılandırılmış interview sorusu (manuel-tuned)
- LLM-as-interviewer with policy module
- User memory state representation (graph + embeddings)
- Çıktı: ranked candidate text list

### Hafta 3 — Forensic Layer A
- Browser forensics (Chrome / Firefox cache + history + autofill)
- Password manager dump parser (KeePass, 1Password export)
- İlk photo OCR (Tesseract baseline)

### Hafta 4 — Forensic Layer B
- Email backup mining (Gmail Takeout, IMAP)
- iCloud / Google Drive backup lokal indir + parse
- File carving (Foremost binding)

### Hafta 5 — Inference + Cracking
- Bayesian candidate ranker
- hashcat + seedcat wrapper
- İlk custom CUDA kernel (Phantom)

### Hafta 6 — Validation Push
- Manual outreach: BTCRecover forum, r/Bitcoin lost-wallet thread'leri, KeychainX'ten reject olanlar
- Hedef: 10 ödeyen pilot ($99 Pro tier)
- Daily user calls

### Hafta 7 — First Recovery
- Pilot feedback'e göre refine
- Hedef: 1 gerçek successful recovery
- Anonim case study draft

### Hafta 8 — Launch Prep
- Trail of Bits engagement açma
- GitHub public open
- ProductHunt + HN draft
- Marketing site (Bölüm 10 positioning)

---

## 13. Başarı kriterleri & validation gates

### 13.1 v1 GATE (Hafta 8 sonu)

**Devam:**
- ≥1 gerçek recovery (kullanıcı doğrulanmış, anonim case study yayında)
- ≥10 ödeyen pilot ($99 Pro)
- Trail of Bits engagement açıldı
- NPS >40 from pilot users

**Kill (hiçbiri yoksa):**
- 0 recovery
- ≤3 ödeyen pilot
- Audit firma reddetti veya engage olmadı

### 13.2 v2 GATE (Yıl 1 sonu)
- ≥10 başarılı recovery
- $200k revenue
- Federated learning loop aktif
- Open-source community ≥500 GitHub star

### 13.3 v3 GATE (Yıl 2 sonu)
- ≥100 başarılı recovery
- $1M revenue
- İlk Enterprise müşteri (Coinbase/Binance/Kraken/Crypto.com dormant program'ı)

---

## 14. Açık riskler ve sorular

### 14.1 Yüksek-etki riskler

1. **Naming category zehiri:** "AI wallet recovery" Google sonuçları scam dolu; open-source + audit yetmeyebilir. Mitigasyon: "AI" kelimesi marketing'de YOK; sadece "forensic" / "guided." Independent reviewer'lar gün 1'den.
2. **AML legal exposure:** Kullanıcı Phoenix ile çalıntı cüzdana erişirse legal sorumluluk. Mitigasyon: Chainalysis/TRM check + ownership attestation + KYC at success-fee tier.
3. **Apple/Google app store red'i:** Mitigasyon: desktop-only başlangıç, web fallback, mobile için sadece interview+report (cracking desktop'ta).
4. **Federated learning privacy attack:** Mitigasyon: differential privacy + secure aggregation + threat model audit.

### 14.2 Orta-etki riskler

5. Hashcat/seedcat upstream breaking change → CI matrix testler, version pinning
6. LLM hallucination cognitive interview'da → multi-agent debate ile cross-check, human-in-loop
7. iCloud/Google Drive API değişiklikleri → fallback parsers, format detection

### 14.3 Açık sorular (resolve edilmeli)

- v1 yargı yetkisi: İsviçre mi Estonya mı? **Aksiyon: hukuki danışman ile 30 dakikalık call (hafta 1)**
- Naming workshop kim yapacak? Branding agency $5k-15k vs. DIY
- İlk 10 pilot user'ı nasıl bulacağız? Outreach playbook detayı
- Trail of Bits engagement maliyeti: $50k-$200k tahmini, finansman planı
- Ollama vs. llama.cpp vs. MLX (Apple Silicon) performans karşılaştırması

---

## 15. v1 sonrası yol haritası

### Yıl 1 Q3-Q4
- Mobile app (iOS/Android) — sadece interview + report (cracking desktop'ta)
- Hardware wallet seed extraction (software-only, Praefortis ile rekabet etmiyor)
- Multilingual interview (Türkçe, Almanca, İspanyolca, Mandarin)

### Yıl 2
- Enterprise SaaS: borsa / custodian dormant account API
- Heir / inheritance specific UX (estate executor partnership)
- Insurance partnership: pre-protection sigorta + recovery rider

### Yıl 3
- Phoenix Foundation (open-source governance)
- Recovery network: federated GPU compute pool
- Token (opsiyonel) — Pro tier holder'lar premium tier erişim

---

## EK A: Tasarım kararları & gerekçeleri

| Karar | Niye |
|---|---|
| Tauri vs Electron | Bundle boyutu, performans, Rust güvenlik |
| Llama 3.3 vs GPT-5 API | Lokal-only ilkesi, kullanıcı verisi cloud'a çıkmaz |
| hashcat wrapper vs custom | Decade-tested, regression riski düşük |
| MPC threshold ECDSA vs trust | Phoenix anahtarı asla görmez = güven moat |
| Open-source MIT vs proprietary | Scam-genre'den ayrışmak için zorunlu |
| %5 success fee vs %15 artisan | Düşük fee + ürün scale = daha yüksek mutlak gelir |
| Federated learning vs central | Privacy + moat (rakipler sıfırdan başlar) |

---

## EK B: Threat model (ana hatlar)

### Adversary modelleri
1. **Pasif gözlemci:** Network trafiğini izler. Mitigasyon: lokal-only çalışma, network isolation.
2. **Malicious cloud GPU:** Kiralık cloud'da seed sızdırma. Mitigasyon: TEE attestation, sadece encrypted candidate testing.
3. **Çalıntı cüzdan kullanıcısı:** Phoenix'i çalıntı cüzdan açmak için kullanır. Mitigasyon: Chainalysis check + KYC + attestation.
4. **Phoenix ekibi malicious:** Backdoor inject. Mitigasyon: open-source + reproducible builds + community audit.
5. **Federated learning poisoning:** Adversary fake telemetry ile model'i bozmaya çalışır. Mitigasyon: secure aggregation + outlier detection.

### Detaylı threat model: hafta 4'te security review öncesi yazılacak.

---

## EK C: İlk 10 pilot outreach playbook (hafta 6 öncesi yazılacak)

---

## Spec sonu

Bu spec, brainstorming skill'i kapsamında **Phoenix v1 design dokümanı** olarak commit edilmiştir. İmplementation plan ayrı bir dokümanda ele alınacak (writing-plans skill).

**Onay durumu:** Sahip (Enes) tarafından 2026-05-08'de toplu onay verildi.
