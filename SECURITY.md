# 보안 가이드

이 문서는 Microsoft Store 배포를 위한 보안 설정 및 체크리스트를 포함합니다.

## 구현된 보안 기능

### 1. CSP (Content Security Policy)
- ✅ Tauri 설정에 CSP 정책 적용
- ✅ HTML 메타 태그에 CSP 추가
- ✅ 인라인 스크립트 및 스타일 허용 (필요시)
- ✅ 외부 리소스 접근 제한

### 2. 파일 경로 검증
- ✅ 경로 정규화 및 정규화 검증
- ✅ 상대 경로(`..`) 처리 방지
- ✅ 경로 길이 제한 (260자)
- ✅ 파일 확장자 검증 (HEIC/HEIF만 허용)

### 3. 입력 검증
- ✅ 파일 크기 제한 (최대 100MB)
- ✅ 이미지 크기 제한 (최대 32767x32767 픽셀)
- ✅ 메모리 사용량 제한 (500MB)
- ✅ 파일 개수 제한 (최대 100개)
- ✅ 파일명 길이 제한 (255자)
- ✅ 출력 형식 검증 (JPG/PNG만 허용)

### 4. 에러 메시지 보안
- ✅ 민감한 정보(전체 경로) 제거
- ✅ 파일명만 표시
- ✅ 시스템 정보 노출 방지

### 5. DoS 방지
- ✅ 파일 크기 제한
- ✅ 이미지 크기 제한
- ✅ 파일 개수 제한
- ✅ 메모리 사용량 제한

### 6. 권한 최소화
- ✅ 필요한 권한만 허용
- ✅ 파일 시스템 접근 제한
- ✅ 네트워크 접근 없음

## Microsoft Store 배포 전 체크리스트

### 코드 서명
- [ ] 코드 서명 인증서 획득
- [ ] 빌드 스크립트에 서명 자동화 추가
- [ ] 서명 검증 테스트

### 패키징
- [ ] MSIX 패키지 생성
- [ ] 패키지 매니페스트 검증
- [ ] 의존성 확인

### 보안 검사
- [ ] 정적 분석 도구 실행 (예: Clippy, ESLint)
- [ ] 보안 취약점 스캔
- [ ] 코드 리뷰 완료

### 테스트
- [ ] 경로 조작 공격 테스트
- [ ] 파일 크기 제한 테스트
- [ ] 입력 검증 테스트
- [ ] 에러 메시지 테스트

## 코드 서명 설정

Microsoft Store에 앱을 게시하려면 코드 서명 인증서가 필요합니다.

### 인증서 획득 방법
1. **Microsoft Store 인증서 (권장)**
   - Microsoft Partner Center에서 자동 생성
   - 앱 업로드 시 자동 서명

2. **자체 서명 인증서 (테스트용)**
   ```powershell
   New-SelfSignedCertificate -Type CodeSigningCert -Subject "CN=YourApp" -CertStoreLocation Cert:\CurrentUser\My
   ```

### 빌드 스크립트 설정

`tauri.conf.json`에 다음 설정 추가:

```json
{
  "bundle": {
    "windows": {
      "certificateThumbprint": "YOUR_CERTIFICATE_THUMBPRINT",
      "digestAlgorithm": "sha256",
      "timestampUrl": "http://timestamp.digicert.com"
    }
  }
}
```

## 보안 모범 사례

### 파일 처리
- ✅ 사용자가 선택한 파일만 처리
- ✅ 파일 확장자 검증
- ✅ 파일 크기 제한
- ✅ 임시 파일 즉시 삭제

### 메모리 관리
- ✅ 큰 파일 스트리밍 처리
- ✅ 메모리 사용량 제한
- ✅ 리소스 해제 확인

### 에러 처리
- ✅ 민감한 정보 제거
- ✅ 일반적인 에러 메시지 사용
- ✅ 로깅에 민감 정보 제외

## 추가 보안 권장사항

### 1. 업데이트 메커니즘
- Tauri의 자동 업데이트 기능 사용
- 서명된 업데이트만 허용

### 2. 로깅
- 민감한 정보 로깅 방지
- 로그 파일 보안 저장

### 3. 사용자 데이터
- 로컬 저장소 암호화
- 개인정보 보호 준수

## 보안 취약점 보고

보안 취약점을 발견한 경우 다음으로 연락하세요:
- 이메일: [보안팀 이메일]
- GitHub: [보안 이슈 페이지]

## 참고 자료

- [Tauri 보안 가이드](https://tauri.app/v1/guides/security/)
- [Microsoft Store 보안 요구사항](https://docs.microsoft.com/windows/msix/package/security-best-practices)
- [OWASP 보안 체크리스트](https://owasp.org/www-project-web-security-testing-guide/)

