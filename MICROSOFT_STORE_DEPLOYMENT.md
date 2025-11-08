# Microsoft Store 배포 가이드

이 문서는 HEIC 변환기 앱을 Microsoft Store에 배포하기 위한 가이드를 제공합니다.

## 구현된 보안 기능

### ✅ 완료된 보안 요소

1. **CSP (Content Security Policy)**
   - Tauri 설정 및 HTML에 CSP 정책 적용
   - 외부 리소스 접근 제한
   - XSS 공격 방지

2. **파일 경로 검증**
   - 경로 정규화 및 정규화 검증
   - 상대 경로(`..`) 처리 방지
   - 경로 길이 제한 (260자)
   - 파일 확장자 검증

3. **입력 검증**
   - 파일 크기 제한 (100MB)
   - 이미지 크기 제한 (32767x32767 픽셀)
   - 메모리 사용량 제한 (500MB)
   - 파일 개수 제한 (100개)
   - 파일명 길이 제한 (255자)

4. **에러 메시지 보안**
   - 민감한 정보(전체 경로) 제거
   - 파일명만 표시
   - 시스템 정보 노출 방지

5. **DoS 방지**
   - 파일 크기 제한
   - 이미지 크기 제한
   - 파일 개수 제한
   - 메모리 사용량 제한

6. **권한 최소화**
   - 필요한 권한만 허용
   - 파일 시스템 접근 제한
   - 네트워크 접근 없음

7. **코드 최적화**
   - 릴리스 모드 최적화
   - 디버그 심볼 제거
   - 코드 크기 최소화

## Microsoft Store 배포 준비

### 1. 개발자 계정 설정

1. [Microsoft Partner Center](https://partner.microsoft.com/)에 가입
2. 개발자 계정 생성 및 결제 정보 입력
3. 앱 등록 준비

### 2. 앱 패키징

#### MSIX 패키지 생성

Tauri는 자동으로 MSIX 패키지를 생성합니다:

```bash
yarn tauri:build
```

빌드 결과물:
- `src-tauri/target/release/bundle/msix/` - MSIX 패키지

#### 패키지 정보 확인

- 패키지 이름: `HEIC 변환기`
- 버전: `1.0.0`
- 식별자: `com.jinyoung.heic-converter`

### 3. 코드 서명

Microsoft Store에 업로드하면 자동으로 서명됩니다. 로컬 테스트를 위해서는:

1. **자체 서명 인증서 생성** (테스트용):
   ```powershell
   New-SelfSignedCertificate -Type CodeSigningCert -Subject "CN=HEIC Converter" -CertStoreLocation Cert:\CurrentUser\My
   ```

2. **인증서 지문 확인**:
   ```powershell
   Get-ChildItem Cert:\CurrentUser\My -CodeSigningCert
   ```

3. **tauri.conf.json에 설정 추가**:
   ```json
   {
     "bundle": {
       "windows": {
         "certificateThumbprint": "인증서_지문"
       }
     }
   }
   ```

### 4. 앱 매니페스트 설정

Tauri가 자동으로 생성하는 매니페스트를 확인하고 필요시 수정:

- 기능 선언 (Capabilities)
- 권한 설정
- 파일 연결 설정

### 5. 스토어 제출

1. **Partner Center에서 앱 생성**
   - 앱 이름: `HEIC 변환기`
   - 앱 유형: 데스크톱 앱
   - 카테고리: 생산성 도구

2. **앱 정보 입력**
   - 설명
   - 스크린샷
   - 아이콘 (300x300, 150x150, 71x71)
   - 개인정보 보호 정책

3. **패키지 업로드**
   - MSIX 패키지 업로드
   - 자동 서명 처리
   - 인증 완료 대기

4. **제출 및 심사**
   - 제출
   - 심사 대기 (보통 1-3일)
   - 승인 후 스토어에 게시

## 보안 체크리스트

배포 전 다음 사항을 확인하세요:

### 코드 보안
- [x] CSP 정책 적용
- [x] 입력 검증 구현
- [x] 파일 경로 검증
- [x] 에러 메시지 보안
- [x] DoS 방지
- [x] 권한 최소화

### 빌드 보안
- [x] 릴리스 모드 최적화
- [x] 디버그 심볼 제거
- [x] 코드 최적화
- [ ] 코드 서명 (Microsoft Store에서 자동 처리)

### 테스트
- [ ] 경로 조작 공격 테스트
- [ ] 파일 크기 제한 테스트
- [ ] 입력 검증 테스트
- [ ] 에러 메시지 테스트
- [ ] 다양한 Windows 버전 테스트

### 문서화
- [x] 보안 가이드 작성
- [x] 배포 가이드 작성
- [ ] 개인정보 보호 정책 작성
- [ ] 사용자 가이드 작성

## 필수 아이콘 및 이미지

Microsoft Store에 제출하기 위해 다음 이미지가 필요합니다:

1. **스퀘어 로고**
   - 300x300 픽셀 (필수)
   - 150x150 픽셀
   - 71x71 픽셀

2. **스토어 로고**
   - 50x50 픽셀
   - 200x200 픽셀

3. **스크린샷**
   - 최소 1개, 최대 9개
   - 1366x768 또는 1920x1080 권장

4. **타일 이미지**
   - 310x150 픽셀 (선택사항)

현재 프로젝트에 있는 아이콘:
- `src-tauri/icons/Square310x310Logo.png` - 310x310
- `src-tauri/icons/Square150x150Logo.png` - 150x150
- `src-tauri/icons/Square71x71Logo.png` - 71x71
- `src-tauri/icons/StoreLogo.png` - 스토어 로고

## 개인정보 보호 정책

Microsoft Store 제출 시 개인정보 보호 정책이 필요합니다. 다음 내용을 포함해야 합니다:

1. **수집하는 정보**
   - 현재 앱은 사용자 데이터를 수집하지 않습니다.
   - 로컬 파일만 처리하며, 네트워크로 전송하지 않습니다.

2. **데이터 사용**
   - 사용자가 선택한 HEIC 파일만 로컬에서 처리합니다.
   - 변환된 이미지는 사용자가 지정한 위치에 저장됩니다.

3. **데이터 공유**
   - 외부 서버나 서비스와 데이터를 공유하지 않습니다.

4. **보안**
   - 모든 처리는 로컬에서 이루어집니다.
   - 네트워크 연결이 필요하지 않습니다.

## 문제 해결

### 빌드 오류

1. **WebView2 설치 오류**
   - Windows 10 이상 필요
   - WebView2 런타임 자동 설치 확인

2. **코드 서명 오류**
   - Microsoft Store에 업로드하면 자동 서명
   - 로컬 테스트용 인증서 확인

3. **패키지 검증 오류**
   - 매니페스트 검증
   - 아이콘 파일 확인
   - 버전 번호 확인

### 스토어 심사 거부

1. **보안 문제**
   - 보안 체크리스트 확인
   - 보안 가이드 문서 참고

2. **기능 문제**
   - 앱 기능 설명 명확화
   - 스크린샷 업데이트

3. **정책 위반**
   - Microsoft Store 정책 확인
   - 개인정보 보호 정책 업데이트

## 참고 자료

- [Tauri 배포 가이드](https://tauri.app/v1/guides/distribution/)
- [Microsoft Store 앱 제출 가이드](https://docs.microsoft.com/windows/msix/package/packaging-uwp-apps)
- [Microsoft Store 정책](https://docs.microsoft.com/legal/windows/agreements/store-policies)
- [MSIX 패키징 가이드](https://docs.microsoft.com/windows/msix/)

## 다음 단계

1. ✅ 보안 기능 구현 완료
2. ⏳ 앱 테스트
3. ⏳ 스토어 제출 준비
4. ⏳ 패키지 생성 및 업로드
5. ⏳ 심사 및 게시

