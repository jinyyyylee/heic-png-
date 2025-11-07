# HEIC 변환기 - 빌드 및 배포 가이드

## 📦 Windows exe 파일 빌드 방법

### 사전 요구사항

1. **Node.js 및 패키지 관리자**
   - Node.js (v18 이상 권장)
   - npm 또는 yarn이 설치되어 있어야 합니다

2. **Rust 설치**
   ```bash
   # Rust 공식 웹사이트에서 설치
   # https://www.rust-lang.org/tools/install
   # 또는 다음 명령어로 설치:
   # curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. **Microsoft Visual C++ 빌드 도구**
   - Windows에서 Rust 컴파일에 필요합니다
   - Visual Studio Build Tools 또는 Visual Studio Community 설치 필요
   - C++ 빌드 도구 포함해야 합니다

### 빌드 단계

#### 1. 의존성 설치

```bash
# 프로젝트 루트에서 실행
npm install
# 또는
yarn install
```

#### 2. 프로덕션 빌드

```bash
# Tauri 앱 빌드 (exe 파일 생성)
npm run tauri:build
# 또는
yarn tauri:build
```

이 명령어는 다음 작업을 수행합니다:
- 프론트엔드 빌드 (`yarn build`)
- Rust 백엔드 컴파일
- Windows 설치 프로그램 및 exe 파일 생성

### 빌드 결과물 위치

빌드가 완료되면 다음 위치에서 결과물을 찾을 수 있습니다:

```
src-tauri/target/release/
├── tauri-app.exe          # 단일 실행 파일 (배포용)
└── bundle/
    └── msi/
        └── HEIC 변환기_1.0.0_x64_en-US.msi  # 설치 프로그램
```

### 배포 옵션

#### 옵션 1: 단일 exe 파일 배포
- `src-tauri/target/release/tauri-app.exe` 파일을 배포
- 단순하고 빠르지만, 사용자가 직접 실행 파일을 받아야 함
- 코드 서명 없이는 Windows Defender 경고가 발생할 수 있음

#### 옵션 2: MSI 설치 프로그램 배포
- `src-tauri/target/release/bundle/msi/` 폴더의 `.msi` 파일 배포
- 사용자가 설치 프로그램으로 설치 가능
- 더 전문적이고 안전한 배포 방법

### 코드 서명 (선택사항)

배포 시 Windows Defender 경고를 피하려면 코드 서명 인증서가 필요합니다:

1. 코드 서명 인증서 구매 (예: DigiCert, Sectigo)
2. `tauri.conf.json`에 서명 설정 추가

```json
{
  "bundle": {
    "windows": {
      "certificateThumbprint": "인증서 지문",
      "digestAlgorithm": "sha256",
      "timestampUrl": ""
    }
  }
}
```

### 빌드 최적화

#### 릴리스 모드 빌드 (기본)

기본적으로 `tauri:build`는 릴리스 모드로 빌드됩니다. 더 최적화하려면:

```bash
# Rust 릴리스 모드 최적화 (Cargo.toml에서)
[profile.release]
opt-level = "z"     # 코드 크기 최소화
lto = true          # Link Time Optimization
```

#### 파일 크기 최적화

프론트엔드 빌드 최적화는 `vite.config.js`에서 설정할 수 있습니다.

### 문제 해결

#### 빌드 오류: "error: linker `link.exe` not found"
- Visual Studio Build Tools를 설치하세요
- 또는 Rust 설치 후 `rustup toolchain install stable-x86_64-pc-windows-msvc` 실행

#### 빌드 오류: "error: failed to run custom build command for `tauri-build`"
- `cargo clean` 실행 후 다시 빌드 시도
- Rust와 Tauri 버전 호환성 확인

#### 빌드 시간이 오래 걸림
- 첫 빌드는 의존성 컴파일로 인해 시간이 오래 걸립니다
- 이후 빌드는 변경사항만 컴파일하므로 더 빠릅니다

### 배포 체크리스트

- [ ] `tauri.conf.json`에서 버전 번호 확인
- [ ] `package.json`에서 버전 번호 확인
- [ ] 앱 아이콘 설정 확인 (`src-tauri/icons/`)
- [ ] 빌드 성공 확인
- [ ] 생성된 exe 파일 테스트
- [ ] 다른 Windows PC에서 테스트 (필요시)
- [ ] 바이러스 검사 (코드 서명 없이는 오탐지 가능)

### 추가 참고사항

- 빌드된 exe 파일은 Windows 10 이상에서 실행 가능합니다
- .NET Framework나 추가 런타임 설치가 필요하지 않습니다
- 모든 의존성이 exe 파일에 번들로 포함됩니다

