# Vcpkg 설치 가이드 - Windows에서 libheif 빌드

Windows에서 HEIC 변환기 앱을 빌드하려면 Vcpkg가 필요합니다.

## Vcpkg 설치 방법

### 1. Git 설치 확인

Vcpkg는 Git을 통해 설치됩니다. Git이 설치되어 있지 않다면 먼저 설치하세요:

- [Git 다운로드](https://git-scm.com/download/win)

### 2. Vcpkg 설치

#### 방법 1: PowerShell에서 설치 (권장)

```powershell
# 원하는 위치로 이동 (예: C:\dev)
cd C:\dev

# Vcpkg 클론
git clone https://github.com/Microsoft/vcpkg.git

# Vcpkg 디렉토리로 이동
cd vcpkg

# Windows에서 bootstrap 실행
.\bootstrap-vcpkg.bat

# 시스템 통합 (선택사항, 관리자 권한 필요)
.\vcpkg integrate install
```

#### 방법 2: 수동 설치

1. [Vcpkg GitHub](https://github.com/Microsoft/vcpkg)에서 다운로드
2. 압축 해제
3. 해당 폴더에서 PowerShell 실행
4. `.\bootstrap-vcpkg.bat` 실행

### 3. 환경 변수 설정

#### VCPKG_ROOT 환경 변수 설정

**PowerShell에서 (현재 세션만):**

```powershell
$env:VCPKG_ROOT = "C:\dev\vcpkg"
```

**영구적으로 설정 (시스템 환경 변수):**

1. Windows 검색에서 "환경 변수" 검색
2. "시스템 환경 변수 편집" 선택
3. "환경 변수" 버튼 클릭
4. "시스템 변수" 섹션에서 "새로 만들기" 클릭
5. 변수 이름: `VCPKG_ROOT`
6. 변수 값: Vcpkg 설치 경로 (예: `C:\dev\vcpkg`)
7. 확인 클릭

**또는 PowerShell에서 영구 설정:**

```powershell
[System.Environment]::SetEnvironmentVariable("VCPKG_ROOT", "C:\dev\vcpkg", "Machine")
```

### 4. libheif 설치

Vcpkg가 설치되면 libheif를 설치합니다:

```powershell
# Vcpkg 디렉토리로 이동
cd C:\dev\vcpkg

# libheif 설치 (x64)
.\vcpkg install libheif:x64-windows

# 또는 정적 링크로 설치
.\vcpkg install libheif:x64-windows-static
```

**설치 시간:** libheif와 의존성 빌드에 10-30분 정도 소요될 수 있습니다.

### 5. 빌드 재시도

환경 변수를 설정한 후 **새 PowerShell 창**을 열고 빌드를 다시 시도하세요:

```powershell
cd C:\Users\jinyy\IdeaProjects\tauri-app
yarn tauri:build
```

## 문제 해결

### Vcpkg를 찾을 수 없다는 오류

- 환경 변수 `VCPKG_ROOT`가 제대로 설정되었는지 확인
- **새 PowerShell/터미널 창**을 열어야 환경 변수가 적용됩니다
- PowerShell에서 확인: `echo $env:VCPKG_ROOT`

### Visual C++ 빌드 도구 오류

- Visual Studio Build Tools 또는 Visual Studio Community가 설치되어 있어야 합니다
- 설치 시 "C++를 사용한 데스크톱 개발" 워크로드 선택 필요

### 빌드 시간이 너무 오래 걸림

- libheif와 의존성(libx265, libde265 등)을 처음 빌드하는 경우 시간이 오래 걸립니다
- 이후 빌드는 훨씬 빠릅니다

## 대안: Docker를 사용한 빌드

Windows에서 Vcpkg 설치가 어려운 경우, Docker를 사용할 수 있습니다:

```bash
# Docker 이미지에서 빌드
docker run -v ${PWD}:/app -w /app tauri-builder yarn tauri:build
```

## 추가 참고사항

- Vcpkg는 대용량 도구입니다 (수 GB)
- libheif 빌드에 상당한 시간이 소요될 수 있습니다
- 빌드된 라이브러리는 `vcpkg\installed` 폴더에 저장됩니다
